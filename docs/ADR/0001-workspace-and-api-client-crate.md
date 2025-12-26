# ADR-0001: Workspace and Dedicated API Client Crate

- **Status:** Accepted
- **Date:** 2025-10-25

## Context and Problem Statement

We need to interact with the external Taiga.io REST API from our Rust backend. A key architectural goal is to maintain low coupling and high cohesion, ensuring that our core application logic is not tightly bound to the specifics of the Taiga API. We need to decide where and how to implement the API communication code.

## Considered Options

1.  **Direct Implementation:** Implement the API calling logic directly within the Tauri command handlers in the `src-tauri` crate.
2.  **Separate Module:** Create a `taiga_api` module within the `src-tauri` crate.
3.  **Dedicated Crate in a Workspace:** Create a new, independent `taiga-client` crate and manage it alongside the `src-tauri` crate within a unified Cargo Workspace.

## Decision

We have chosen **Option 3**. We will create a dedicated `taiga-client` crate within a Cargo Workspace.

## Rationale and Consequences

### Rationale

- **Separation of Concerns:** This approach creates a strong boundary. The `taiga-client` crate has one responsibility: communicating with the Taiga API. The `larch-app` (Tauri) crate has a different responsibility: orchestrating the application and handling UI logic.
- **Testability:** The `taiga-client` can be tested in complete isolation from the Tauri application, allowing for focused unit and integration tests on the API logic.
- **Reusability:** While not a current requirement, a separate crate could theoretically be published and reused in other projects.
- **Low Coupling:** The main application depends on the `taiga-client`'s public API, not its implementation details. We can change the underlying HTTP client (`reqwest`) or parsing logic (`serde`) within the client crate without affecting the main application, as long as the public interface remains stable.

### Consequences

- **Positive:** Enforces a clean, decoupled architecture from the start.
- **Negative:** Slightly increases the initial setup complexity due to the creation of a workspace and an additional crate. This is a small, one-time cost for significant long-term architectural benefits.
