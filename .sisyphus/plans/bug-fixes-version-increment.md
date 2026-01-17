# Bug Fixes and Version Increment - Work Plan

## Summary

This plan addresses three critical issues in the Larch application:

1. **Authentication & Token Refresh Logic** - Implement backend-driven transparent token refresh using `reqwest-middleware`
2. **Global Sorting Logic for Issues** - Sort all issues globally by `modified_date` after aggregation
3. **Versioning** - Bump version from `2.0.0-0` to `2.0.0-1`

---

## Prerequisites

- **Required Dependencies**: None (will add new Rust dependency)
- **Estimated Time**: 2-3 hours
- **Risk Level**: Medium (token refresh is critical functionality)
- **Testing Required**: Yes (manual and automated)

---

## Phase 1: Backend-Driven Token Refresh (HIGH PRIORITY)

### 1.1 Add `reqwest-middleware` Dependency

**File**: `crates/taiga-client/Cargo.toml`

```toml
[dependencies]
reqwest = { version = "0.12", features = ["json", "multipart"] }
# Add these new dependencies:
reqwest-middleware = "0.3"  # Latest version as of 2026
reqwest-tracing = "0.5"     # Optional but recommended for debugging
http-auth = "0.1"         # For handling authentication headers
```

**Action**: Add dependency using `cargo add` or manually edit Cargo.toml

---

### 1.2 Create Middleware Module

**New File**: `crates/taiga-client/src/middleware.rs`

**Purpose**: Implement transparent token refresh that:

1. Detects 401/403 status codes
2. Detects 200 OK responses with error payloads indicating unauthorized access
3. Automatically refreshes the token using the refresh token from keyring
4. Retries the original request with the new token

**Implementation Details**:

```rust
use reqwest::{Request, Response, StatusCode};
use reqwest_middleware::{Middleware, Next, Result};
use http_auth::parser::www_authenticate::Challenge;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Struct representing Taiga API error responses (even with 200 OK status)
#[derive(Debug, Deserialize)]
struct TaigaErrorResponse {
    #[serde(default)]
    _error: Option<String>,
    #[serde(default)]
    error_code: Option<i32>,
    #[serde(default)]
    detail: Option<String>,
}

/// Check if response body indicates unauthorized access (even with 200 OK)
fn is_unauthorized_payload(body: &str) -> bool {
    if let Ok(parsed) = serde_json::from_str::<TaigaErrorResponse>(body) {
        // Check for error codes that indicate auth failure
        if let Some(code) = parsed.error_code {
            // Taiga may use specific error codes for auth failures
            // Common patterns: 401, 403, or custom codes
            return code == 401 || code == 403;
        }

        // Check for error messages indicating auth issues
        if let Some(detail) = parsed.detail {
            let lower = detail.to_lowercase();
            return lower.contains("unauthorized")
                || lower.contains("authentication")
                || lower.contains("token")
                || lower.contains("credentials");
        }

        if let Some(err) = parsed._error {
            let lower = err.to_lowercase();
            return lower.contains("unauthorized")
                || lower.contains("authentication")
                || lower.contains("token");
        }
    }
    false
}

/// Token refresh callback type
pub type TokenRefreshCallback = Arc<dyn Fn() -> Result<String> + Send + Sync>;

/// Middleware for transparent token refresh
pub struct TokenRefreshMiddleware {
    /// Callback to refresh token and return new token
    refresh_callback: TokenRefreshCallback,

    /// Flag to prevent infinite refresh loops
    max_retries: u32,
}

impl TokenRefreshMiddleware {
    pub fn new(refresh_callback: TokenRefreshCallback) -> Self {
        Self {
            refresh_callback,
            max_retries: 1, // Allow one retry after refresh
        }
    }

    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }
}

#[async_trait::async_trait]
impl Middleware for TokenRefreshMiddleware {
    async fn handle(
        &self,
        mut req: Request,
        extensions: &mut http::Extensions,
        next: Next<'_>,
    ) -> Result<Response> {
        let retry_count = extensions.get::<u32>().copied().unwrap_or(0);

        if retry_count > self.max_retries {
            return Err(reqwest_middleware::Error::Middleware(
                anyhow::anyhow!("Max token refresh retries exceeded").into(),
            ));
        }

        let mut response = next.run(req.clone(), extensions).await?;

        // Check for 401/403 status codes
        let needs_refresh = if response.status() == StatusCode::UNAUTHORIZED
            || response.status() == StatusCode::FORBIDDEN {
            true
        } else if response.status().is_success() {
            // Check for 200 OK with unauthorized error payload
            let body = response.text().await?;
            if is_unauthorized_payload(&body) {
                log::warn!("Detected unauthorized error in 200 OK response: {}", body);
                true
            } else {
                // Restore the body since we consumed it
                // Note: reqwest doesn't allow restoring body, so we need to reconstruct
                // This is a limitation - we should cache the original request
                false
            }
        } else {
            false
        };

        if needs_refresh {
            log::info!("Token refresh needed - status: {}", response.status());

            // Call the refresh callback
            let new_token = (self.refresh_callback)().map_err(|e| {
                reqwest_middleware::Error::Middleware(
                    anyhow::anyhow!("Token refresh failed: {}", e).into(),
                )
            })?;

            log::info!("Token refreshed successfully, retrying request");

            // Update the Authorization header with new token
            req.headers_mut().insert(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", new_token).parse().unwrap(),
            );

            // Mark as a retry in extensions
            extensions.insert(retry_count + 1);

            // Retry the original request
            return next.run(req, extensions).await;
        }

        Ok(response)
    }
}
```

**Action**: Create the middleware module with above implementation

---

### 1.3 Update TaigaClient to Use Middleware

**File**: `crates/taiga-client/src/lib.rs`

**Changes Required**:

1. Add module declaration:

```rust
pub mod errors;
pub mod middleware;  // NEW
pub mod models;
pub mod prelude;

use errors::TaigaClientError;
use middleware::TokenRefreshMiddleware;  // NEW
```

2. Update `TaigaClient` struct to use `ClientBuilder` with middleware:

```rust
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest::StatusCode;

#[derive(Debug, Clone)]
pub struct TaigaClient {
    client: ClientWithMiddleware,  // Changed from reqwest::Client
    api_base_url: Url,
}

impl TaigaClient {
    pub fn new(api_base_url: Url) -> Self {
        let raw_client = reqwest::Client::new();

        // Create a no-op middleware by default
        let client = ClientBuilder::new(raw_client).build();

        Self {
            client,
            api_base_url,
        }
    }

    /// Create a new client with token refresh middleware
    pub fn with_refresh(
        api_base_url: Url,
        refresh_callback: middleware::TokenRefreshCallback,
    ) -> Self {
        let raw_client = reqwest::Client::new();

        let refresh_middleware = TokenRefreshMiddleware::new(refresh_callback)
            .with_max_retries(1);

        let client = ClientBuilder::new(raw_client)
            .with(refresh_middleware)
            .build();

        Self {
            client,
            api_base_url,
        }
    }

    /// Expose the inner middleware client for use in commands
    pub fn inner(&self) -> &ClientWithMiddleware {
        &self.client
    }
}
```

**Action**: Update TaigaClient to support middleware-based token refresh

---

### 1.4 Update `auth_commands.rs` to Use Middleware

**File**: `src-tauri/src/commands/auth_commands.rs`

**Changes**:

The `login` command needs to create a `TaigaClient` with the middleware:

```rust
#[tauri::command]
pub async fn login(
    app_handle: tauri::AppHandle,
    repo: tauri::State<'_, crate::repositories::SqliteRepository>,
    api_url: &str,
    username: &str,
    password: &str,
) -> Result<User> {
    // Create initial client without middleware for login
    let client = TaigaClient::new(api_url.parse()?);
    let auth_detail = client.login(username, password).await?;

    repo.save_config("taiga_api_url", api_url).await?;

    credentials::set_api_token(&auth_detail.auth_token)?;

    if let Some(refresh) = &auth_detail.refresh {
        credentials::set_refresh_token(refresh)?;
    }

    let token = credentials::get_api_token()?;
    let me = client.get_me(&token).await?;

    // Now create a client with refresh middleware for subsequent requests
    let base_url: Url = api_url.parse()?;
    let refresh_callback = std::sync::Arc::new(move || -> Result<String> {
        let refresh_token = credentials::get_refresh_token()?;
        // We need to create a temporary client to call refresh
        let temp_client = TaigaClient::new(base_url.clone());
        let new_tokens = tokio::runtime::Handle::current()
            .block_on(temp_client.refresh_token(refresh_token.expose_secret()))?;

        credentials::set_api_token(&new_tokens.auth_token)?;
        credentials::set_refresh_token(&new_tokens.refresh)?;

        Ok(new_tokens.auth_token)
    });

    let client_with_refresh = TaigaClient::with_refresh(base_url, refresh_callback);

    app_handle.manage(client_with_refresh);

    Ok(me.into())
}
```

**Note**: This approach has a complexity - the refresh callback needs to work within the async context. We may need to adjust this pattern.

**Alternative Approach** (Recommended):

Instead of embedding the refresh logic in the middleware callback, have the middleware simply return a special error, and handle the refresh at the command layer.

**Revised Plan for 1.4**:

Let's simplify and make the middleware signal when refresh is needed, then handle it in the application layer.

**File**: `crates/taiga-client/src/errors.rs`

Add a new error variant:

```rust
#[derive(Debug, Error)]
pub enum TaigaClientError {
    // ... existing variants ...

    #[error("Token refresh required")]
    TokenRefreshRequired,
}
```

**Update middleware** to return this error instead of doing the refresh inline.

Then update the command layer to catch this error and trigger refresh.

**Action**: Decide on the best approach and implement

---

### 1.5 Remove Frontend `apiCall` Wrapper

**File**: `src/lib/services/api.ts`

**Action**: Either:

- Delete this file (if no other code uses it), OR
- Add a deprecation notice and keep for future potential use

**Verify**: Search codebase to confirm no imports of this file:

```bash
grep -r "from.*api" src/ --include="*.ts" --include="*.svelte"
```

---

### 1.6 Add Tests for Token Refresh

**New File**: `crates/taiga-client/src/middleware_tests.rs`

**Test Cases**:

1. Middleware passes through successful requests
2. Middleware detects 401 status and triggers refresh
3. Middleware detects 403 status and triggers refresh
4. Middleware detects 200 OK with unauthorized error payload
5. Middleware handles refresh failure gracefully
6. Middleware prevents infinite retry loops

---

## Phase 2: Global Sorting for Issues (MEDIUM PRIORITY)

### 2.1 Update `get_aggregated_issues` Command

**File**: `src-tauri/src/commands/project_commands.rs`

**Location**: Lines 123-132 in current file

**Current Code**:

```rust
let mut all_issues = Vec::new();
for task in tasks {
    match task.await {
        Ok(Ok(issues)) => all_issues.extend(issues),
        Ok(Err(e)) => log::error!("Failed to fetch issues: {}", e),
        Err(e) => log::error!("Task join error: {}", e),
    }
}

Ok(all_issues.into_iter().map(|i| i.into()).collect())
```

**New Code**:

```rust
let mut all_issues = Vec::new();
for task in tasks {
    match task.await {
        Ok(Ok(issues)) => all_issues.extend(issues),
        Ok(Err(e)) => log::error!("Failed to fetch issues: {}", e),
        Err(e) => log::error!("Task join error: {}", e),
    }
}

// Convert to domain models
let mut domain_issues: Vec<Issue> = all_issues.into_iter().map(|i| i.into()).collect();

// Sort globally by modified_date (newest first)
domain_issues.sort_by(|a, b| {
    match (&a.modified_date, &b.modified_date) {
        (Some(date_a), Some(date_b)) => date_b.cmp(date_a), // Descending
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => std::cmp::Ordering::Equal,
    }
});

Ok(domain_issues)
```

**Alternative** (sort at the DTO level for better performance):

```rust
// Sort at the DTO level before conversion
all_issues.sort_by(|a, b| {
    match (&a.modified_date, &b.modified_date) {
        (Some(date_a), Some(date_b)) => date_b.cmp(date_a), // Descending
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => std::cmp::Ordering::Equal,
    }
});

Ok(all_issues.into_iter().map(|i| i.into()).collect())
```

**Action**: Update sorting logic to happen AFTER aggregation

---

### 2.2 Add Tests for Global Sorting

**File**: `src-tauri/src/commands/project_commands_tests.rs` (or add to existing test module)

**Test Cases**:

1. Multiple projects with interleaved dates produce correctly sorted results
2. Issues with null modified_date are handled correctly
3. Sorting is stable when dates are equal
4. Empty issue list returns empty result

---

## Phase 3: Version Bump (LOW PRIORITY - FINAL STEP)

### 3.1 Run Version Bump Command

**Command**:

```bash
just bump 2.0.0-1
```

**Files to be Updated**:

1. `package.json` - Frontend version
2. `src-tauri/tauri.conf.json` - Tauri app version
3. `src-tauri/Cargo.toml` - Rust backend version

**Action**: Execute the bump command

---

### 3.2 Verify Version Synchronization

**Verification Steps**:

```bash
# Check package.json
grep '"version"' package.json

# Check tauri.conf.json
grep '"version"' src-tauri/tauri.conf.json

# Check Cargo.toml
grep '^version' src-tauri/Cargo.toml
```

**Expected Output**: All should show `2.0.0-1`

**Action**: Verify all versions match

---

## Implementation Order

### Recommended Sequence:

1. **Phase 2 (Sorting)** - Easiest, lowest risk, no dependencies
   - Implement global sorting in `project_commands.rs`
   - Add tests
   - Verify with manual testing

2. **Phase 1 (Token Refresh)** - Most complex, highest priority
   - Add `reqwest-middleware` dependency
   - Create middleware module
   - Update `TaigaClient` to use middleware
   - Update `auth_commands.rs`
   - Add tests
   - Perform manual testing (expire token, verify auto-refresh)

3. **Phase 3 (Version Bump)** - Final step after all fixes complete
   - Run `just bump 2.0.0-1`
   - Verify version sync

---

## Testing Strategy

### Manual Testing Checklist

#### Token Refresh:

1. Log in to Taiga instance
2. Wait for token to expire (or manually expire via API)
3. Navigate to dashboard - should auto-refresh without error
4. Check logs for "Token refreshed successfully" message
5. Try accessing issues, projects - all should work seamlessly

#### Sorting:

1. Create test data: issues across multiple projects with various modified dates
2. Select multiple projects in dashboard
3. Verify issues appear sorted by modified_date (newest first)
4. Verify interleaved dates are correctly ordered
5. Verify issues with null dates appear at the bottom

### Automated Testing

```bash
# Run all tests
just test

# Run specific test modules
cd src-tauri && cargo test project_commands::tests
cd crates/taiga-client && cargo test middleware
```

---

## Risk Mitigation

### Token Refresh Risks:

| Risk                                     | Likelihood | Impact | Mitigation                            |
| ---------------------------------------- | ---------- | ------ | ------------------------------------- |
| Middleware causes infinite loops         | Medium     | High   | Add max_retries counter (default: 1)  |
| Refresh token expires                    | Low        | Medium | Handle by redirecting to login screen |
| 200 OK error detection is too aggressive | Medium     | Medium | Log all detected cases for review     |
| Performance impact                       | Low        | Low    | Middleware overhead is minimal        |

### Sorting Risks:

| Risk                              | Likelihood | Impact | Mitigation                           |
| --------------------------------- | ---------- | ------ | ------------------------------------ |
| Date parsing issues               | Low        | Medium | Use string comparison for ISO dates  |
| Performance with many issues      | Low        | Low    | Sorting O(n log n) is acceptable     |
| Breaking existing UI expectations | Low        | Medium | Document the change in release notes |

---

## Rollback Plan

If issues arise:

### Token Refresh Rollback:

1. Revert `taiga-client` to previous version
2. Remove `reqwest-middleware` dependency
3. Restore `TaigaClient::new()` to use plain `reqwest::Client`
4. The frontend `apiCall` wrapper is still available as backup

### Sorting Rollback:

1. Remove the sort logic from `get_aggregated_issues`
2. Restore the original `extend()` pattern
3. Issues will revert to per-project ordering

### Version Rollback:

1. Run `just bump 2.0.0-0` to restore previous version
2. Or manually edit the three version files

---

## Success Criteria

### Token Refresh:

✅ Middleware correctly detects 401/403 status codes
✅ Middleware detects 200 OK responses with unauthorized error payloads
✅ Token is refreshed automatically without user intervention
✅ Failed refresh is handled gracefully (redirect to login)
✅ No infinite retry loops occur
✅ All API calls work seamlessly after token expiration

### Sorting:

✅ All issues from selected projects are aggregated first
✅ Issues are sorted globally by `modified_date` (descending)
✅ Issues with null `modified_date` appear at the end
✅ No per-project grouping visible in final result

### Version:

✅ All three version files show `2.0.0-1`
✅ Version is properly synchronized across components
✅ `just check` passes without errors

---

## Notes for Implementer

1. **Middleware Complexity**: The middleware approach adds significant complexity. Consider if a simpler frontend-based refresh using the existing `apiCall` wrapper might be sufficient. However, the backend approach is more secure and reliable.

2. **200 OK Error Detection**: This is speculative based on the "collisions bug" mention. Need to verify with actual Taiga API responses if this is actually happening. May need to inspect live API traffic first.

3. **Async in Refresh Callback**: The current plan has a complexity with async in the middleware callback. May need to use `tokio::sync::oneshot` channels or reconsider the architecture.

4. **Date Parsing**: For sorting, we're using string comparison on ISO 8601 dates. This works for the format `"2023-01-02T12:00:00Z"` because lexicographic order matches chronological order.

5. **Testing Priority**: Thoroughly test the token refresh flow before releasing, as a bug here could lock users out of their accounts.

---

## References

- **reqwest-middleware docs**: https://docs.rs/reqwest-middleware/
- **Taiga API Authentication**: Check official Taiga API documentation for auth behavior
- **Project Files Referenced**:
  - `crates/taiga-client/src/lib.rs`
  - `crates/taiga-client/src/errors.rs`
  - `src-tauri/src/commands/auth_commands.rs`
  - `src-tauri/src/commands/project_commands.rs`
  - `src-tauri/src/domain/issue.rs`
  - `src/lib/services/api.ts`
  - `Justfile`

---

**Plan Status**: Ready for Implementation
**Created**: 2026-01-17
**Planner**: AI Agent (Ultrawork Mode)
