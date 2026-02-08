# Larch Architecture

This document provides a high-level overview of the Larch application architecture. For detailed reasoning behind specific architectural decisions, please refer to the [Architecture Decision Records (ADRs)](./ADR/).

## Core Principles

Our architecture is guided by the principles of **Clean Architecture**:

- **Low Coupling:** Components should be independent and unaware of the implementation details of others.
- **High Cohesion:** Each component should have a single, well-defined responsibility.
- **Testability:** Components should be easily testable in isolation.

## High-Level Structure

Larch is a [Tauri](https://tauri.app/) application composed of three main parts:

1.  **Frontend (`src/`):** A [Svelte](https://svelte.dev/) + TypeScript single-page application that provides the user interface.
2.  **Backend (`src-tauri/`):** The core application logic written in Rust. It exposes functionality to the frontend via Tauri commands.
3.  **API Client (`crates/taiga-client/`):** A dedicated, independent Rust crate responsible for all communication with the external Taiga.io API.

## Frontend Structure

The frontend is organized into screens and reusable components, following Svelte 5 patterns.

### Screens

- **Login**: Handles user authentication and Taiga instance URL configuration.
- **Dashboard**: The main "single pane of glass" view containing the aggregated issue table.
- **ProjectConfig**: Allows users to select which projects to track and manage their local settings.

### Key Components

- **ViewSwitcher**: Header-level dropdown for navigating between Saved Views.
- **FilterBar**: Interactive bar for applying filters (Project, Status, Assignee) with "Dirty State" detection.
- **IssueDetailSheet**: An overlay (Sheet) displaying full issue details, descriptions, and comments.
- **IssueMetadataSidebar**: A sidebar within the detail view for managing issue attributes (Status, Priority, Severity).

## Data Layer

Larch uses SQLite for local persistence, managed via SeaORM.

| Entity        | Description                                                            |
| :------------ | :--------------------------------------------------------------------- |
| `config`      | Stores application-wide settings and the active Taiga instance URL.    |
| `drafts`      | Persists unsaved issue descriptions and comments to prevent data loss. |
| `saved_views` | Stores user-defined filter presets (projects, statuses, assignees).    |

## Authentication Flow

Larch implements a secure, transparent authentication flow:

1. **Login**: User provides Taiga URL and credentials.
2. **Token Acquisition**: Backend exchanges credentials for a JWT (Access + Refresh tokens).
3. **Secure Storage**: Tokens are stored in the OS Keyring via `keyring-rs`.
4. **Transparent Refresh**: The `taiga-client` middleware automatically handles JWT expiration by using the Refresh token to acquire a new Access token without user intervention.
5. **Session Management**: The `TaigaClient` is maintained in Tauri's managed state during the application lifecycle.

## Backend Internal Architecture

To ensure our backend is idiomatic, secure, and maintainable, we follow these patterns:

- **Dynamic Dependency Injection:** Core services like the `TaigaClient` are created dynamically. For instance, the `TaigaClient` is instantiated by the `login` command using the user-provided API URL. Upon successful authentication, the client is placed into Tauri's managed state. Subsequent commands access this shared client instance via the `tauri::State` guard.

- **Model Separation (Anti-Corruption Layer):** We maintain a strict separation between API data structures and our application's domain models.
  - **DTOs:** The `taiga-client` crate defines Data Transfer Objects (e.g., `TaigaProjectDto`) used for API serialization.
  - **Domain Models:** The `larch-app` crate defines its own internal models (e.g., `Project`).
  - **Mapping:** We use the `From` trait for clean, type-safe mapping from DTOs to Domain Models.

- **Data Access Layer (Repository Pattern):**
  - We use **SeaORM** as the ORM to interact with the local SQLite database.
  - **Repositories:** Data access is encapsulated in Repositories (e.g., `SqliteRepository`). This abstracts the specific database implementation from the rest of the application.
  - **Entities:** SeaORM entities (`src-tauri/src/entities`) define the database schema.

- **Secure Credential Management:** The Taiga API token is a secret and is **never** stored in application state. We use the `keyring-rs` crate to securely store the token in the operating system's native credential manager. Tauri commands retrieve the token from the keyring on-demand for each API call.

## Project Organization

The project is organized as a Cargo Workspace to manage the different Rust crates.

- **ADR-0001:** For the decision to use a workspace and a separate API client crate, see [ADR-0001](./ADR/0001-workspace-and-api-client-crate.md).
- **ADR-0002:** For the decision to dynamically instantiate the API client, see [ADR-0002](./ADR/0002-dynamic-client-instantiation.md).
