# Larch - Design Document

> **Author:** Oscar Garcia de Lara Parreño
> **Version:** 1.0
> **Last Updated:** 2025-10-25
> **Project Name:** Larch

## 1. Overview & Context (The "Why")

### 1.1. Problem Statement

Taiga.io is an effective project management tool, but its architecture isolates each project into a distinct silo. For users who manage or participate in multiple projects, this requires them to manually switch contexts—navigating into each project individually—to track, manage, and respond to issues. This process is time-consuming, inefficient, and increases the risk of missing critical updates or having slow response times, especially as the number of managed projects grows.

### 1.2. Vision & Goal

The vision for Larch is to create a centralized, multi-project issue management dashboard for Taiga.io that acts as a "single pane of glass" for issue triage and management.

The primary goal is to provide a user with a single, actionable interface where they can quickly view, filter, and perform core actions (like commenting and changing status) on issues from all their accessible Taiga projects, eliminating the need for constant context-switching. The key success metric is a dramatic reduction in the time required for issue management and triage, ensuring no critical updates are missed.

### 1.3. Key Terminology / Glossary

- **Taiga Instance:** A specific deployment of Taiga, either the official cloud version (`taiga.io`) or a self-hosted (on-premise) server.
- **Project:** A standard project workspace within a Taiga Instance.
- **Issue:** A ticket within the "Issues" module of a Taiga Project.
- **Issue Status:** A named state for issues configured per-project (e.g., "New", "In Progress", "Need info", "Closed", "Rejected"). Each status has:
  - A `name` (user-defined)
  - A `color` for visual identification
  - An `is_closed` boolean flag indicating if this status represents a closed/resolved state
- **is_closed:** A boolean flag on Issue Status that indicates whether the status represents a "closed" state. Multiple statuses can be "open" (e.g., New, In Progress) or "closed" (e.g., Closed, Rejected). This is distinct from the status name itself.
- **Issue Type:** A classification category (e.g., "Bug", "Enhancement", "Question") with a name and color, configured per-project.
- **Severity:** The impact level of an issue (e.g., "Critical", "Normal", "Minor") with a name and color, configured per-project.
- **Priority:** The importance ranking of an issue (e.g., "High", "Normal", "Low") with a name and color, configured per-project.
- **Attachments:** Files attached to an issue, including images embedded in descriptions. Each has a URL, name, size, and optional thumbnail.

---

## 2. Goals & Non-Goals (The "Scope")

### 2.1. Goals (Must-Haves for v1.0)

- [ ] **Multi-Instance Support:** The application must allow a user to specify the API endpoint URL for their Taiga instance.
- [ ] **Username/Password Authentication:** The initial connection will be established via username and password.
- [ ] **Secure Credential Storage:** The application will securely store the user's auth/refresh tokens on the local machine using the native OS keychain.
- [ ] **Unified Issue Dashboard:** The core feature is a single, filterable table view that aggregates issues from multiple, user-selected projects.
- [ ] **Intelligent Default "Triage View":** The dashboard will load with smart default filters (e.g., `Status IS NOT 'Closed'`, `Assigned To: Me or Unassigned`) to show actionable issues immediately.
- [ ] **Advanced Filtering (Include/Exclude):** Users must be able to filter by attributes (Project, Status, Assignee) using both inclusion and exclusion logic.
- [ ] **Core Issue Interactions:** Users must be able to: view issue details, add a comment, and change its status.
- [ ] **Robust Auto-Save:** In-progress descriptions and comments will be auto-saved locally to prevent data loss, with explicit save to API on user action.
- [ ] **Desktop Application:** The final product will be a standalone, cross-platform desktop application built with Tauri.

### 2.2. Non-Goals (What this project is NOT)

- **NO** support for other Taiga modules (User Stories, Tasks, Wiki, etc.). The focus is exclusively on **Issues**.
- **NO** alternative authentication methods (e.g., SSO, GitHub, etc.).
- **NO** web-based or hosted version. This is a desktop-only application.
- **NO** administrative features (e.g., creating projects, managing users).
- **NO** offline functionality. An active internet connection to the Taiga instance is required.
- **NO** support for custom fields on issues.
- **NO** real-time auto-refresh. Data is refreshed on app start and via a manual "Refresh" button.
- **NO** "saved filters" (presets). This is a target for v1.1.

---

## 3. User Experience & Requirements (The "What")

### 3.1. User Persona

- **"Alex, the Multi-Project Manager"**: Alex is a technically proficient developer or manager who oversees multiple software projects in Taiga. They value efficiency, speed, and clarity above all else. Their primary frustration is the time wasted context-switching between projects to stay on top of new issues, comments, and status changes. They need a tool that gives them a high-level, actionable overview quickly.

### 3.2. User Journey / Flow

1.  **First-Time Setup:**
    - User launches the app for the first time.
    - They are presented with a clean login screen. The Taiga Cloud API URL is pre-filled.
    - A discreet settings icon allows them to change the API URL for a self-hosted instance.
    - They enter their username and password and log in.
    - Upon successful login, they are navigated to the Project Configuration screen.
2.  **Project Configuration:**
    - The app fetches and displays a list of all projects Alex has access to.
    - A search bar allows Alex to quickly filter the project list.
    - Each project has a toggle switch to "Include in Dashboard".
    - Alex enables the projects he wants to track. This selection is saved locally and persists across sessions.
    - There is a clear navigation link to return to the main "Dashboard".
3.  **The Main Dashboard (Daily Use) - "Triage View":**
    - The main view is a filterable table of all issues from the selected projects.
    - **Intelligent Default Filters:** The dashboard loads with pre-applied filters (e.g., `Status IS NOT 'Closed'`, `Assigned To: Me or Unassigned`) to present an actionable "Triage View".
    - **Filtering UI:** A prominent filter bar is available with clear indicators of active filters (e.g., `Estado: Abiertos`, `Excluyendo 2`).
      - Clicking a filter button opens a **Popover** (as sketched by Clara) for `Projects`, `Status`, and `Assigned To`.
      - Each filter within the Popover supports both **inclusion and exclusion logic** via checkboxes and a dropdown/segmented control.
      - A free-text search bar searches issue titles/subjects.
    - The table displays columns: `Subject`, `Status`, `Project`, `Assigned To`.
    - Filter selections are remembered during the session.
4.  **Issue Interaction:**
    - Clicking on an issue in the table navigates the user to a dedicated **Issue Detail View**.
    - This view shows all issue details: subject, description, status, assignee, reporter, tags, and comments.
    - The user can perform actions: change status, re-assign, add comments.
    - **Robust Auto-Save:** The description and new comment fields trigger auto-save functionality to a **local SQLite draft** while the user types. The UI will provide subtle feedback (e.g., "Borrador guardado").
    - **API Commit:** Changes are committed to the Taiga API only upon explicit user action (e.g., navigating back to the dashboard, attempting to close the app).
    - **Concurrency Conflict (412):** If an API commit fails due to a version conflict (`412 Precondition Failed`), the app will block navigation/closure and present a clear modal to the user, offering options to "Discard my draft and see changes" or "Overwrite changes with my version."
5.  **Data Refresh:**
    - A "Refresh" button in the Dashboard triggers an asynchronous background data fetch.
    - The button itself will show a spinner, but the UI will remain interactive.
    - Upon completion, a **toast notification** will provide feedback (e.g., "Refresco completado. (3 issues nuevos, 5 actualizados)" or "Error al refrescar.").

### 3.3. User Stories

- **Authentication & Configuration:**
  - _As Alex,_ I want to log in with my username and password to my specified Taiga instance securely.
  - _As Alex,_ I want to easily switch between Taiga Cloud and a self-hosted instance URL.
  - _As Alex,_ I want to see a list of all my projects and select which ones I want to monitor, so my dashboard isn't cluttered.
  - _As Alex,_ I want the application to remember my project selection across sessions.
- **Dashboard & Filtering:**
  - _As Alex,_ I want the app to open directly to an intelligent "Triage View" showing me actionable issues, so I can start working immediately.
  - _As Alex,_ I want to see all issues from my selected projects in a single, unified table view.
  - _As Alex,_ I want to filter the issues by one or more Projects, Statuses, and Assignees using both inclusion and exclusion logic (e.g., 'Status IS NOT Closed') to precisely narrow down my view.
  - _As Alex,_ I want the filter controls to clearly indicate what filters are currently active.
  - _As Alex,_ I want the Status and Assignee filters to dynamically update based on the projects I've selected for viewing.
  - _As Alex,_ I want to manually refresh the issue list in the background without blocking the UI, and receive a notification when it's done.
- **Issue Management:**
  - _As Alex,_ I want to click an issue and see its full details, including description and all comments, in a clear, dedicated view.
  - _As Alex,_ I want to be able to change the status and assignee of an issue directly from the app.
  - _As Alex,_ I want to add new comments to an issue.
  - _As Alex,_ I want my description and comment fields to be saved as a draft locally while I type, so I never lose my work.
  - _As Alex,_ if someone else changes an issue while I'm editing it, I want to be informed of the conflict and choose how to resolve it before saving my changes.

---

## 4. System Architecture (The "How")

### 4.1. High-Level Diagram

[ User's Desktop (Tauri) ] | +-- [ Frontend (Svelte + TS) ] <-- (Tauri Commands) --> [ Backend (Rust) ] | | | | +-- (Svelte Stores) +-- [ SQLite DB (config, drafts) ] | | | +-- [ OS Keychain (tokens) ] | | | +-- [ Taiga API Service Module ] <--- (HTTPS) ---> [ Taiga Instance API ] | +-- (OS Events: e.g., Close Window)

### 4.2. Technology Stack

- **Application Framework:** **Tauri v2** (Rust Backend + Webview Frontend).
- **Backend Logic:** **Rust**.
- **Frontend:** **Svelte 5 + TypeScript**.
  - _Justification:_ Chosen to learn a new, modern paradigm focused on performance and developer experience (DX). Its compiler-based nature aligns perfectly with Tauri's lightweight philosophy.
- **Styling:** **Tailwind CSS**.
  - _Justification:_ Utility-first CSS for implementing a custom design (like one inspired by Google's Stitch) with maximum control.
- **Secure Credential Storage:** **`keyring-rs` crate**.
- **Local Configuration & Data:** **SQLite** (via **`sea-orm`** ORM).

### 4.3. Local Storage Strategy (SQLite via SeaORM)

- **`config` entity:** Key-value store for `taiga_api_url`, `selected_project_ids`, etc.
- **`drafts` entity:** To store auto-saved `content` (TEXT) related to a `draft_type` (e.g., 'comment') and `related_id` (e.g., `issue_id`).

### 4.4. Internal API (Tauri Commands - Frontend to Backend)

- **Auth:** `login(url, username, password)`, `logout()`
- **Config:** `get_projects()`, `get_selected_projects()`, `save_selected_projects(ids)`
- **Issue Data:** `get_aggregated_issues(filters: FilterObject)` (This command will handle fetching from N projects based on user-defined filters).
- **Issue Actions (Granular):**
  - `save_local_draft(issue_id, content)`: Writes content directly to local SQLite `drafts` table. No API call.
  - `change_issue_status(issue_id, status_id, etag)`: Commits status change to Taiga API.
  - `change_issue_assignee(issue_id, assignee_id, etag)`: Commits assignee change to Taiga API.
  - `add_comment(issue_id, content)`: Adds comment to Taiga API.
  - `commit_issue_description(issue_id, etag)`: Reads the latest draft from SQLite and commits the description change to Taiga API.
- **Refresh:** `refresh_issues()`: Triggers a new `get_aggregated_issues()` call.

### 4.5. Taiga API Service (Rust Module - Backend to Taiga API)

This Rust module encapsulates _all_ communication with the external Taiga API.

- **Authentication:** Handles attaching bearer tokens and refreshing them via the `refresh_token` on `401 Unauthorized`.
- **Rate Limiting:** Implements **exponential backoff** on `429 Too Many Requests`.
- **Concurrency:** Manages **Optimistic Locking** using **ETags**. All write operations (excluding `add_comment`) must send an `If-Match` header. Will correctly handle `412 Precondition Failed` by returning a specific `Error::Conflict` to the frontend, indicating a version mismatch.
- **Aggregated Fetching:** The `get_aggregated_issues()` logic will iterate through all selected projects, making individual API calls (`GET /projects/{project_id}/issues`) for each. These calls will include API-level filters (e.g., `status__not_in=X,Y`) derived from the user's current filters to minimize data payload. If some project fetches fail, the service will return partial data with clear indications of failures for specific projects.

### 4.6. Auto-Save & Graceful Shutdown Logic

- **Auto-Save (Frontend Driven):**
  1.  User types in a field (e.g., issue description).
  2.  Frontend (Svelte) updates its local state.
  3.  A `useDebounce` (e.g., 2-3 seconds) triggers a call to the Rust command `save_local_draft(issue_id, current_content)`.
  4.  Rust then writes the draft directly to the local SQLite `drafts` table. This is an immediate, offline operation.
- **Graceful Shutdown & Explicit Commit:**
  1.  When the user navigates back from the "Issue Detail" view, or attempts to close the application:
  2.  The Svelte frontend calls the Rust command `commit_issue_description(issue_id, etag)`.
  3.  Rust reads the latest draft from SQLite and attempts to commit it to the Taiga API using the ETag.
  4.  If `412 Precondition Failed` (conflict), Rust returns `Error::Conflict`. Svelte intercepts this, blocks navigation/closure, and displays the conflict resolution modal.
  5.  If successful, Rust deletes the local draft from SQLite.

### 4.7. Key Libraries & State Management

- **State Management:** **Svelte Native Stores** (`writable`, `derived`). Simple, powerful, and sufficient for managing application state.
- **UI Components:** **`shadcn-svelte`**. Provides accessible, headless components styled with Tailwind, offering full design control.
- **Data Table:** Custom implementation using Shadcn/Tailwind. (Originally planned `svelte-headless-table`, but moved to simpler custom implementation for flexibility).

---

## 5. Implementation & Rollout Plan (The "When")

### 5.1. Milestones /Phases

- **M1: "Core Auth & Read"**
  - Goal: Establish a secure connection and prove data can be fetched.
  - Tasks: Build Tauri skeleton. Build basic Svelte login screen with `shadcn-svelte` forms. Implement Rust `login()` command, Keychain storage, `get_projects()` command.
  - Success: User can log in, select Taiga instance URL, and see a list of their projects (e.g., in the project configuration UI).
- **M2: "Read-Only Dashboard & Advanced Filters"**
  - Goal: Create the main unified issue view with robust filtering.
  - Tasks: Build Svelte UI (Dashboard, Project Selection screen) with Tailwind and `shadcn-svelte` components (especially the table and the advanced filter Popover). Implement Rust commands `get_selected_projects()`, `save_selected_projects()`, and `get_aggregated_issues()` (ensuring API-level filtering).
  - Success: User can select projects, apply complex include/exclude filters, and see a read-only, filterable table of their issues. Performance of initial load is acceptable.
- **M3: "Write Interaction & Concurrency"**
  - Goal: Enable full issue management.
  - Tasks: Build Svelte "Issue Detail" view. Implement all explicit write-action Rust commands (`change_status`, `add_comment`, `commit_issue_description`). Crucially, implement the full **ETag/Optimistic Locking** logic (`412 Precondition Failed` handling) in Rust and the corresponding conflict resolution modal in Svelte.
  - Success: User can open, comment on, change the status of an issue. Conflicts are handled gracefully with user choice.
- **M4: "Quality of Life & Robustness"**
  - Goal: Polish the application and make it resilient.
  - Tasks: Implement the Svelte debounce logic for `save_local_draft` to SQLite. Implement the "graceful shutdown" listener in Rust. Implement the **toast notification system** for refresh feedback.
  - Success: In-progress work is never lost. User receives clear feedback on background operations.

### 5.2. Testing Strategy

- **Rust Unit Tests:** For critical business logic (e.g., state transitions, Taiga API service parsing, ETag handling, local SQLite operations).
- **Integration Tests (Tauri):** Test the full Frontend (Svelte) to Backend (Rust Command) communication chain.
- **Manual E2E Testing:** For v1.0, the main user flows (Login -> Select Projects -> Apply Filters -> Edit Issue -> Refresh) will be tested manually. Particular attention will be paid to conflict resolution scenarios.

---

## 6. Risks & Open Questions (The "Unknowns")

### 6.1. Potential Risks

- **Risk 1 (High): Learning Curve of the Stack (Tauri/Rust/Svelte)**
  - _Description:_ The developer is learning three new core technologies simultaneously. This will impact initial development velocity.
  - _Mitigation:_ Acknowledge that Milestones 1 & 2 will take longer than usual. The focus is on learning and correct implementation, not raw speed. The phased implementation plan helps isolate learning challenges.
- **Risk 2 (Medium): Taiga API Variability & Performance**
  - _Description:_ Different Taiga instances (cloud vs. self-hosted, different versions) may have subtle API differences. Aggregating data from N projects, even with filters, may still be slow if Taiga's API latency is high or if many issues match the filters.
  - _Mitigation:_ Develop against the official cloud API first. Document assumptions about Taiga API behavior. Implement aggressive API-level filtering as default. If performance becomes an issue, consider a "Non-Goal" for v1.1 like client-side pagination or virtual scrolling.
- **Risk 3 (Low): Svelte/shadcn-svelte Ecosystem Gaps for Specific UI Needs**
  - _Description:_ A highly specific UI component (beyond the table and basic inputs/popovers) might be missing from the Svelte ecosystem or `shadcn-svelte`.
  - _Mitigation:_ This risk has been largely mitigated by selecting `shadcn-svelte` and `svelte-headless-table`, which cover the most complex components. Custom components will be built as needed with Tailwind.

### 6.2. Open Questions

- _How will app updates be distributed and managed?_ (Tauri has a built-in updater. This will be investigated post-M4.)
- _Will specific "Project Roles" (e.g., Admin vs. Member in Taiga) affect which issues/actions are visible or allowed?_ (For v1.0, assume the API key grants
