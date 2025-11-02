# Larch Architecture

This document provides a high-level overview of the Larch application architecture. For detailed reasoning behind specific architectural decisions, please refer to the [Architecture Decision Records (ADRs)](./ADR/).

## Core Principles

Our architecture is guided by the principles of **Clean Architecture**:

- **Low Coupling:** Components should be independent and unaware of the implementation details of others.
- **High Cohesion:** Each component should have a single, well-defined responsibility.
- **Testability:** Components should be easily testable in isolation.

## High-Level Structure

Larch is a [Tauri](https://tauri.app/) application composed of three main parts:

1.  **Frontend (`larch-app/src`):** A [Svelte](https://svelte.dev/) + TypeScript single-page application that provides the user interface.
2.  **Backend (`larch-app/src-tauri`):** The core application logic written in Rust. It exposes functionality to the frontend via Tauri commands.
3.  **API Client (`larch-app/crates/taiga-client`):** A dedicated, independent Rust crate responsible for all communication with the external Taiga.io API.

## Backend Internal Architecture

To ensure our backend is idiomatic, secure, and maintainable, we follow these patterns:

-   **Dynamic Dependency Injection:** Core services like the `TaigaClient` are created dynamically. For instance, the `TaigaClient` is instantiated by the `login` command using the user-provided API URL. Upon successful authentication, the client is placed into Tauri's managed state. Subsequent commands access this shared client instance via the `tauri::State` guard.

-   **Model Separation (Anti-Corruption Layer):** We maintain a strict separation between API data structures and our application's domain models.
    -   **DTOs:** The `taiga-client` crate defines Data Transfer Objects (e.g., `TaigaProjectDto`) used for API serialization.
    -   **Domain Models:** The `larch-app` crate defines its own internal models (e.g., `Project`).
    -   **Mapping:** We use the `From` trait for clean, type-safe mapping from DTOs to Domain Models.

-   **Secure Credential Management:** The Taiga API token is a secret and is **never** stored in application state. We use the `keyring-rs` crate to securely store the token in the operating system's native credential manager. Tauri commands retrieve the token from the keyring on-demand for each API call.

## Project Organization

The project is organized as a Cargo Workspace to manage the different Rust crates.

- **ADR-0001:** For the decision to use a workspace and a separate API client crate, see [ADR-0001](./ADR/0001-workspace-and-api-client-crate.md).
- **ADR-0002:** For the decision to dynamically instantiate the API client, see [ADR-0002](./ADR/0002-dynamic-client-instantiation.md).
