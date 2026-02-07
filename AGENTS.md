# AGENTS.md - Larch Project Guidelines

This document provides context and guidelines for AI coding agents working in the Larch codebase.
Larch is a cross-platform desktop client for Taiga.io built with Tauri v2, Rust, and Svelte 5.

---

## 1. Build, Lint, and Test Commands

### Quick Reference (via Justfile)

```bash
just dev              # Run in development mode (pnpm tauri dev)
just check            # Run all checks (clippy + svelte-check)
just check-rust       # Run clippy on Rust code
just check-svelte     # Run svelte-check
just test             # Run all tests
just test-rust        # Run Rust tests only
just format           # Format all code
just format-rust      # Format Rust code only
just format-svelte    # Format Svelte/TS code only
just bump <version>   # Bump version (e.g., just bump 2.0.0-beta.1)
```

### Running Individual Tests (Rust)

```bash
# Run a single test by name
cd src-tauri && cargo test test_save_and_get_config

# Run tests in a specific module
cd src-tauri && cargo test repositories::tests

# Run taiga-client tests
cd crates/taiga-client && cargo test
```

### Frontend Commands (pnpm)

```bash
pnpm check            # svelte-check + tsc
pnpm lint             # prettier --check + eslint
pnpm format           # prettier --write
pnpm build            # vite build
```

---

## 2. Project Architecture

### Directory Structure

```text
src/                    # Svelte 5 frontend (TypeScript)
  lib/
    components/         # Reusable UI components
    screens/            # Top-level page components
    stores/             # Svelte stores (use $state for local state)
    commands.svelte.ts  # Centralized Tauri command names
    types.ts            # TypeScript interfaces
src-tauri/              # Rust backend (Tauri v2)
  src/
    commands/           # Tauri command handlers
    domain/             # Domain models (anti-corruption layer)
    entities/           # SeaORM database entities
    repositories/       # Data access layer (Repository pattern)
    services/           # Business services (credentials, db)
    error.rs            # Unified error types
crates/
  taiga-client/         # Independent API client crate
```

### Key Design Patterns

- **Clean Architecture**: Commands -> Domain -> Repositories -> Database
- **Anti-Corruption Layer**: DTOs from `taiga-client` are converted to domain models via `From` trait
- **Dynamic DI**: TaigaClient is instantiated at login and stored in Tauri managed state
- **Repository Pattern**: SqliteRepository abstracts SeaORM for data access
- **Secure Credentials**: API tokens stored in OS keyring, never in app state

---

## 3. Code Style Guidelines

### Rust

- **Edition**: 2021, MSRV 1.77.2
- **Error Handling**: Use `thiserror` for error types. Prefer `expect("message")` over `unwrap()`
- **Linting**: `cargo clippy -- -D warnings` must pass
- **Formatting**: `cargo fmt`
- **Result Type**: Use crate-level `Result<T>` aliased to `std::result::Result<T, Error>`
- **Logging**: Use `log` crate macros (`log::info!`, `log::error!`)

```rust
// Good: Clear error context
let token = credentials::get_api_token()?;
let data = client.get_issue(&token, id).await?;

// Good: Custom error types with From implementations
#[derive(Debug, Error, Serialize)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(String),
}

impl From<sea_orm::DbErr> for Error {
    fn from(e: sea_orm::DbErr) -> Self {
        Error::Database(e.to_string())
    }
}
```

### TypeScript/Svelte

- **Svelte 5 Runes**: Use `$state`, `$derived`, `$effect`, `$props`, `$bindable`
- **Never use**: Legacy Svelte 4 stores (`writable`, `derived` from svelte/store)
- **DOM Timing**: Use `await tick()` from `'svelte'`, never `setTimeout(..., 0)`
- **Variable Shadowing**: Never shadow props or outer variables inside functions
- **Formatting**: Prettier with tabs, single quotes, no trailing commas

```svelte
<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { CMD_GET_PROJECTS } from '$lib/commands.svelte';
	import type { Project } from '$lib/types';
	import { t } from 'svelte-i18n';

	let projects = $state<Project[]>([]);
	let loading = $state(false);
	let filteredProjects = $derived(projects.filter((p) => p.active));

	// Props with destructuring
	let { issue, onUpdate }: { issue: Issue; onUpdate?: () => void } = $props();
</script>
```

### i18n

- **All user-facing strings** must use `$t()` from `svelte-i18n`
- Translation files: `src/lib/locales/en.json` and `static/locales/en.json`

```svelte
<!-- Good -->
<h2>{$t('dashboard.title')}</h2>

<!-- Bad -->
<h2>Dashboard</h2>
```

### Security

- **XSS Prevention**: Escape HTML entities before rendering user markdown
- **Use `{@html}` sparingly**: Only with sanitized content
- **Tauri V2 Capabilities**: Check `src-tauri/capabilities/default.json`

---

## 4. Imports and File Organization

### Rust Import Order

```rust
// 1. Crate-level imports
use crate::domain::issue_detail::IssueDetail;
use crate::error::Result;
use crate::repositories::Repository;

// 2. External crates
use serde::Serialize;
use taiga_client::TaigaClient;
use tauri::State;
```

### TypeScript/Svelte Import Order

```typescript
// 1. Svelte imports
import { onMount } from 'svelte';
import { invoke } from '@tauri-apps/api/core';

// 2. Internal lib imports (use $lib alias)
import { CMD_GET_PROJECTS } from '$lib/commands.svelte';
import type { Project } from '$lib/types';
import IssueTable from '$lib/components/dashboard/IssueTable.svelte';

// 3. UI components
import { Button } from '$lib/components/ui/button';
import { Input } from '$lib/components/ui/input';

// 4. Icons and external
import { RefreshCw, Search } from '@lucide/svelte';
import { toast } from 'svelte-sonner';
import { t } from 'svelte-i18n';
```

---

## 5. Naming Conventions

### Rust

- **Modules**: snake_case (`issue_commands.rs`, `issue_detail.rs`)
- **Types/Structs**: PascalCase (`IssueDetail`, `TaigaClient`)
- **Functions/Methods**: snake_case (`get_issue_detail`, `from_dto`)
- **Constants**: SCREAMING_SNAKE_CASE

### TypeScript/Svelte

- **Files**: PascalCase for components (`IssueTable.svelte`), camelCase for utilities
- **Types/Interfaces**: PascalCase (`IssueDetail`, `FilterObject`)
- **Functions/Variables**: camelCase (`handleClick`, `filteredIssues`)
- **Command constants**: SCREAMING_SNAKE_CASE (`CMD_GET_PROJECTS`)

---

## 6. Error Handling

### Rust Error Pattern

```rust
// Define errors with thiserror
#[derive(Debug, thiserror::Error, Serialize)]
pub enum Error {
    #[error("Taiga client error: {0}")]
    TaigaClient(String),

    #[error("Version conflict")]
    VersionConflict,
}

// Implement From for automatic conversion
impl From<taiga_client::errors::TaigaClientError> for Error {
    fn from(e: taiga_client::errors::TaigaClientError) -> Self {
        match e {
            TaigaClientError::VersionConflict(_) => Error::VersionConflict,
            other => Error::TaigaClient(other.to_string()),
        }
    }
}
```

### Frontend Error Pattern

```typescript
try {
	issues = await invoke(CMD_GET_AGGREGATED_ISSUES, { filters });
} catch (error) {
	console.error('Failed to fetch issues:', error);
	toast.error($t('errors.unknown'));
}
```

---

## 7. Tauri Commands

- **Define commands** in `src-tauri/src/commands/` grouped by domain
- **Export command names** in `src/lib/commands.svelte.ts`
- **Use State guards** for accessing managed state

```rust
#[tauri::command]
pub async fn get_issue_detail(
    client: tauri::State<'_, TaigaClient>,
    issue_id: i64,
) -> Result<IssueDetail> {
    let token = credentials::get_api_token()?;
    let issue_dto = client.get_issue(&token, issue_id).await?;
    Ok(IssueDetail::from_dto(issue_dto))
}
```

---

## 8. Testing

### Rust Tests

- Use `#[tokio::test]` for async tests
- Use in-memory SQLite for repository tests
- Place tests in `mod tests` or separate `tests.rs` files

```rust
#[tokio::test]
async fn test_draft_lifecycle() {
    let conn = create_test_db().await;
    let repo = SqliteRepository::new(conn);

    repo.save_draft("issue_123", "comment", "Content").await.unwrap();
    let fetched = repo.get_draft("issue_123", "comment").await.unwrap();
    assert_eq!(fetched, Some("Content".to_string()));
}
```

---

## 9. Source of Truth

Before implementing, always check these documents:

1. **Product Definition**: `docs/doc-design.md` (features, scope, libraries)
2. **Architecture**: `docs/architecture.md` (structural patterns)
3. **ADRs**: `docs/ADR/*` (technical decisions)

If your implementation contradicts a document, raise the conflict before proceeding.

---

## 10. Libraries Reference

| Layer         | Library                | Version |
| ------------- | ---------------------- | ------- |
| Framework     | Tauri                  | 2.x     |
| Frontend      | Svelte                 | 5.x     |
| UI Components | shadcn-svelte, bits-ui | latest  |
| Styling       | TailwindCSS            | 4.x     |

**shadcn-svelte Documentation**: For component API reference and examples, use Context7 with library ID `/llmstxt/shadcn-svelte_llms_txt` or visit https://shadcn-svelte.com/llms.txt
| Icons | @lucide/svelte | latest |
| i18n | svelte-i18n | 4.x |
| ORM | SeaORM | 1.x |
| HTTP Client | reqwest | 0.12 |
| Secrets | keyring-rs | 2.x |

**MCP Context7**: Use for Tauri v2 / Svelte 5 API lookups - training data may be outdated.

---

## 11. AI Agent Guidelines

### Documentation Format Reminders

- **AGENTS.md is MARKDOWN**, not JSON or any other format
- All documentation files in this project use standard Markdown syntax
- For code blocks, always specify the language: `rust, `typescript, ```svelte
- Markdown images use standard syntax: `![alt text](url)` - never `![] (url)`
- File names are case-sensitive and use kebab-case or PascalCase as appropriate

### Common Pitfalls to Avoid

1. **Document Format Confusion**: Never mistake .md files for JSON or other formats
2. **Image Syntax**: Always use `![alt text](url)` - the `![] (url)` format is invalid
3. **Language Hints**: Always add language hints to code blocks for proper highlighting
4. **File Naming**: Check existing patterns before creating/referencing files

### When in Doubt

1. Check existing documentation files for format examples
2. Use standard GitHub-flavored Markdown syntax
3. Verify file names match what exists in the repo
4. Test image syntax in a markdown viewer if unsure
