# Larch - Design Document

> **Status:** ✅ Implemented
> **Author:** Oscar Garcia de Lara Parreño
> **Version:** 2.1 (Implemented)
> **Last Updated:** 2026-02-08
> **Project Name:** Larch
> **Baseline:** v2.0 (Stitch Redesign, Svelte 5, SeaORM)

---

## 1. Overview & Context (The "Why")

### 1.1. Problem Statement

Taiga.io is an effective project management tool, but its architecture isolates each project into a distinct silo. For users who manage or participate in multiple projects, this requires them to manually switch contexts—navigating into each project individually—to track, manage, and respond to issues. This process is time-consuming, inefficient, and increases the risk of missing critical updates. While Larch v2.0 introduced an aggregated "single pane of glass" view, users still face friction when manually re-applying complex filter combinations for different work contexts (e.g., "Critical Bugs" vs. "Client X Review").

### 1.2. Vision & Goal

The vision for Larch is to create a centralized, multi-project issue management dashboard for Taiga.io that acts as a "single pane of glass" for issue triage and management.

**v2.1 Focus:** Evolve the platform from a "visualization tool" to a "personalized management hub" by implementing **Saved Views**. This version aims to allow users to define, persist, and switch between complex filter contexts instantly, eliminating manual configuration and enabling priority-driven navigation.

### 1.3. Key Terminology / Glossary

- **Taiga Instance:** A specific deployment of Taiga (official cloud or self-hosted).
- **Active Triage:** The default, system-level view that filters out closed issues to reduce cognitive load.
- **Saved View (Preset):** A persistent configuration of filters (projects, statuses, assignees) with a user-defined name.
- **Dirty State:** A visual indicator signaling that the current active filters have been modified and differ from the version persisted in the database.
- **View Switcher:** A header-level component used to navigate between different saved views.
- **Google Stitch:** An AI design tool used to generate HTML/CSS layouts as a development reference.
- **is_closed:** A boolean flag on Issue Status indicating if this status represents a closed/resolved state.

---

## 2. Goals & Non-Goals (The "Scope")

### 2.1. Goals (v2.1)

- [x] **View Persistence:** Implement local storage for custom filters in SQLite using SeaORM.
- [x] **Dropdown View Switcher:** Replace the static title with an interactive selector to switch and delete views.
- [x] **Contextual Save Management:** Implement a "Split Button" in the filter bar to save changes or create new views without leaving the triage flow.
- [x] **Integrity Validation:** Backend logic to detect and sanitize orphan project or status IDs within saved views.
- [x] **Dirty State Indicator:** A subtle visual notification (asterisk or dot) to alert users of unsaved changes in the current view.
- [x] **Functional Continuity:** Maintain all v2.0 features, including sidebar navigation, overlay details, link copying, and clipboard screenshot support.

### 2.2. Non-Goals

- **NO Cloud Synchronization:** Larch remains a local-first application; no external backend for view synchronization will be implemented.
- **NO Bulk Editing:** Managing multiple issues simultaneously remains out of scope.
- **NO View Sharing:** Saved views are strictly local to the user's device.
- **NO Real-time Auto-refresh:** Manual refresh with Toast feedback remains the standard.

---

## 3. User Experience & Requirements (The "What")

### 3.1. User Persona

- **"Alex, the Multi-Project Manager"**: Oversees 10+ projects. Alex needs to switch between different "battlefronts" (e.g., "Urgent", "Backend", "Frontend") multiple times a day and expects the application to remember exactly how they prefer to view each data segment.

### 3.2. User Journey / Flow (v2.1)

1. **Navigation:** Alex opens Larch and lands on the default **"Active Triage"** view.
2. **Configuration:** Alex applies specific filters for a client and two specific issue statuses.
3. **Creation:** Upon detecting changes, the filter bar displays the **"Save as new view"** button. Alex names it "Client X - Pending".
4. **Daily Use:** Alex uses the **View Switcher** in the header to jump instantly between "Active Triage" and "Client X - Pending".
5. **Modification:** If Alex changes a filter in "Client X", the **Dirty State** indicator (an asterisk next to the name) activates. Alex clicks the primary **"Save"** button to update the persisted view.
6. **Cleanup:** Alex hovers over an old view in the switcher and clicks the **trash icon** to remove it.

### 3.3. UX Acceptance Criteria

- **View Switcher:** Must appear as a clean dropdown menu; system-level views (Active Triage) must be locked and non-deletable.
- **Split Button:** Only visible in the filter bar when changes are pending or a customizable view is active.
- **Naming Modal:** Minimalist dialog that validates the name input is not empty.
- **Dirty State:** Resets immediately upon saving or when switching to a different view.
- **Ordering:** Views are ordered by `last_used` timestamp in the switcher for quick access.

---

## 4. System Architecture (The "How")

### 4.1. High-Level Diagram

`[ User's Desktop (Tauri v2) ]`
`|`
`+-- [ Frontend (Svelte 5 + TS) ]  <-- (Tauri Commands) --> [ Backend (Rust) ]`
`|       |      (Reference: Stitch & OpenCode)              |`
`|       +-- (Local State Store: Active View)           +-- [ SQLite DB (SeaORM) ]`
`|                                                              |   |-- Table: saved_views`
`|                                                      +-- [ Taiga API Service ]`

### 4.2. Technology Stack

- **Framework:** Tauri v2.
- **Backend:** Rust.
- **Frontend:** Svelte 5 + TypeScript.
- **Design Reference:** Google Stitch (AI-generated HTML/CSS).
- **Styling:** Tailwind CSS.
- **Database:** SQLite via **SeaORM**.
- **Security:** `keyring-rs` for token management.

### 4.3. Data Schema (SeaORM Entities)

The `SavedView` entity is added to the SQLite database:

- `id`: UUID (Primary Key).
- `name`: String (User-defined name).
- `filter_data`: Json (Object containing `project_ids`, `status_ids`, `assignees`, and inclusion/exclusion logic).
- `is_default`: Boolean (Indicates the startup view).
- `last_used`: DateTime (Timestamp for ordering).

### 4.4. Backend Logic & Validation

1. **Sanitization:** Before returning a saved view to the frontend, the Rust backend verifies that persisted project and status IDs still exist in the local configuration.
2. **Dirty Detection:** Deep comparison of the filter state received from the frontend against the persisted version to toggle the modification flag.
3. **CRUD Operations:** Atomic operations to create, update, and delete views in SQLite.

---

## 5. Implementation & Rollout Plan (v2.1)

### 5.1. Milestones

1. **M1: Persistence Layer (Backend):** Create the SeaORM entity and Tauri commands for `saved_views` management.
2. **M2: Header View Switcher:** Implement the interactive dropdown in the header based on Stitch layouts.
3. **M3: Filter Bar Evolution:** Implement the dynamic Split Button and Dirty State logic in Svelte 5.
4. **M4: Modal & UX Polish:** Minimalist naming dialog and Toast feedback for save/delete operations.

### 5.2. Testing Strategy

- **Rust Unit Tests:** Serialization of filter JSON and ID validation logic.
- **Component Testing (Svelte):** Verify Dirty State reactivity to filter changes.
- **Manual E2E:** Test "Save as new" duplicates, deletion of views, and persistence after application restarts.

---

## 6. Risks & Open Questions

- **Risk 1 (High):** Complexity of translating Stitch-generated HTML/CSS into reactive Svelte 5 components while maintaining consistency.
- **Risk 2 (Medium):** Data inconsistency if Taiga modifies project or status IDs. _Mitigation:_ Implement backend sanitization as described in section 4.4.
- **Risk 3 (Medium):** UI real estate for the Split Button on small window sizes. _Mitigation:_ Design the button to collapse text and show only icons if space is restricted.
