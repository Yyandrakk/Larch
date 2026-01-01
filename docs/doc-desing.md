# Larch - Design Document

> **Author:** Oscar Garcia de Lara Parreño
> **Version:** 2.0 (Redesign & Enhanced Collision Phase)
> **Last Updated:** 2026-01-01
> **Project Name:** Larch
> **Baseline:** v1.1 (Tauri v2, Svelte 5, Testing suite, SeaORM)

---

## 1. Overview & Context (The "Why")

### 1.1. Problem Statement
Taiga.io is an effective project management tool, but its architecture isolates each project into a distinct silo. For users who manage or participate in multiple projects, this requires them to manually switch contexts—navigating into each project individually—to track, manage, and respond to issues. This process is time-consuming, inefficient, and increases the risk of missing critical updates or having slow response times, especially as the number of managed projects grows.

### 1.2. Vision & Goal
The vision for Larch is to create a centralized, multi-project issue management dashboard for Taiga.io that acts as a "single pane of glass" for issue triage and management.

**v2.0 Focus:** Full structural and visual overhaul. Transitioning from a page-based navigation to a modern, sidebar-driven interface using AI-generated layouts (Google Stitch) as a development reference. This version also introduces advanced Git-like conflict resolution to improve the reliability of multi-user editing.

### 1.3. Key Terminology / Glossary
- **Taiga Instance:** A specific deployment of Taiga, either the official cloud version (`taiga.io`) or a self-hosted (on-premise) server.
- **Project:** A standard project workspace within a Taiga Instance.
- **Issue:** A ticket within the "Issues" module of a Taiga Project.
- **Issue Status:** A named state for issues configured per-project (e.g., "New", "In Progress", "Need info", "Closed", "Rejected").
- **is_closed:** A boolean flag on Issue Status indicating if this status represents a closed/resolved state.
- **Google Stitch:** An AI design tool used to generate HTML/CSS layouts that serve as a visual and structural reference for Svelte component development.
- **Collision (Conflict):** A concurrency conflict occurring when two users edit the same issue simultaneously (HTTP 412).
- **Advanced Conflict Resolution:** A Git-like interface allowing users to compare local changes side-by-side with server data and pick the desired state for each field.

---

## 2. Goals & Non-Goals (The "Scope")

### 2.1. Goals (v2.0)
- **Stitch-based Redesign:** Use AI-generated layouts to implement a professional UI for login, navigation, and dashboards.
- **Sidebar Navigation:** Implement a persistent sidebar for switching between "Projects" and "Dashboard".
- **Overlay Issue Detail:** Replace the full-page view with a Sidebar Drawer that superimposes over the main table for faster context-switching.
- **Advanced Collision Management:** A new modal that allows diffing and merging local vs. server changes field by field.
- **Secure Credential Storage:** Maintain the use of the native OS keychain for auth/refresh tokens.
- **Desktop Application:** Standalone, cross-platform app built with Tauri v2.

### 2.2. Non-Goals
- **NO** "Saved Filters" (Presets): These remain a target for future versions (v2.1+).
- **NO** support for other Taiga modules (User Stories, Tasks, Wiki, etc.).
- **NO** administrative features (e.g., creating projects, managing users).
- **NO** offline functionality or real-time auto-refresh.

---

## 3. User Experience & Requirements (The "What")

### 3.1. User Persona
- **"Alex, the Multi-Project Manager"**: Oversees 10+ projects. Values efficiency and a modern interface that allows viewing details via an overlay without losing the context of the main aggregated list.

### 3.2. User Journey / Flow (v2.0)
1. **Redesigned Login:** Redesigned entry point using layouts inspired by Google Stitch.
2. **Navigation Hub:** A persistent **Sidebar** allows Alex to switch between "Projects" (selection) and "Dashboard" (triage).
3. **The Dashboard (Triage):** An aggregated table with "Smart Default" filters.
4. **Overlay Interaction:** Clicking an issue opens the **Sidebar Drawer (Overlay)**. The table remains visible underneath, allowing Alex to keep the list in sight while editing.
5. **Conflict Resolution:** If a collision occurs upon saving, the **Advanced Collision Modal** appears, showing a side-by-side comparison (Local vs. Server) to resolve differences like a Git merge.
6. **Data Feedback:** Every background action (like Refresh) triggers a **Toast notification** with results.

### 3.3. User Stories
- **Auth & Nav:** - *As Alex,* I want a modern, sidebar-driven navigation to switch instantly between project setup and my triage dashboard.
  - **Acceptance Criteria:**
    - Login page uses Stitch-generated layout.
    - Sidebar loads in < 500ms.
    - Navigation between "Projects" and "Dashboard" does not reload the page.
- **Dashboard & Filtering:**
  - *As Alex,* I want to filter issues by inclusion/exclusion logic and have the app remember my "Triage View" defaults.
  - **Acceptance Criteria:**
    - Filters persist across sessions in SQLite.
    - Triage view loads pre-filtered issues on app launch.
- **Issue Management:** 
  - *As Alex,* I want to edit an issue in an overlay sidebar so I don't lose my place in the aggregated list.
  - *As Alex,* if a collision occurs, I want to see exactly what changed on the server vs. my local draft and choose which changes to keep.
---

## 4. System Architecture (The "How")

### 4.1. High-Level Diagram
`[ User's Desktop (Tauri v2) ]`
  `|`
  `+-- [ Frontend (Svelte 5 + TS) ]  <-- (Tauri Commands) --> [ Backend (Rust) ]`
  `|       |      (Reference: Stitch)                         |`
  `|       +-- (Svelte Stores)                            +-- [ SQLite DB (SeaORM) ]`
  `|                                                              |`
  `|                                                      +-- [ OS Keychain (keyring-rs) ]`
  `|                                                              |`
  `|                                                      +-- [ Taiga API Service ]`

### 4.2. Technology Stack
- **Framework:** Tauri v2.
- **Backend:** Rust.
- **Frontend:** Svelte 5 + TypeScript.
- **Design Reference:** Google Stitch (AI-generated HTML/CSS as development source).
- **Styling:** Tailwind CSS.
- **Database:** SQLite via **SeaORM**.
- **Security:** `keyring-rs` for token management.

### 4.3. Advanced Collision Logic
The v2.0 collision system improves upon the v1.1 warning:
1.  **Detection:** Catch `412 Precondition Failed` error during API commit.
2.  **Comparison:** Backend fetches the latest server state of the issue.
3.  **UI Resolution:** Modal opens a side-by-side diff view.
4.  **Selection:** User picks field-by-field which data to keep (Local vs. Server) or "Keep All Local" / "Keep All Server".

---

## 5. Implementation & Rollout Plan (v2.0)

### 5.1. Milestones
1.  **M1: Login Redesign.** Apply Stitch-generated layout to the login screen.
2.  **M2: Main Flow Redesign.**
    - 2.1 **Sidebar Navigation:** Implement persistent navigation between projects and dashboard.
    - 2.2 **Project Selector:** Redesign the project selection UI.
    - 2.3 **Dashboard:** Redesign aggregated issue table and advanced filter UI.
3.  **M3: Detail Sidebar (Overlay).** Implement the overlay sidebar component for issue detail viewing.
4.  **M4: Collision Resolution Modal.** Implement advanced conflict resolution system (Git-like diff).

### 5.2. Testing Strategy
- **Rust Unit Tests:** Critical business logic, API service parsing, and ETag handling.
- **Integration Tests:** Tauri command chain from Svelte 5 to Rust.
- **Manual E2E:** Focus on the new collision resolution modal and sidebar navigation state.

---

## 6. Risks & Open Questions

- **Risk 1 (High):** Complexity of translating Stitch-generated HTML/CSS into reactive Svelte 5 components while maintaining consistency.
- **Risk 2 (Medium):** Managing UI focus and scroll states when the sidebar overlay is open above the main table.
- **Risk 3 (Medium):** Performance impact of the diffing logic on the client-side for very large issue descriptions. Mitigation: implement lazy diffing or pagination for large text fields.