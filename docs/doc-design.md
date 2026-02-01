# Larch - Design Document

> **Author:** Oscar Garcia de Lara Parreño
> **Version:** 2.0 (Redesign & UX Enhancement Phase)
> **Last Updated:** 2026-01-31
> **Project Name:** Larch
> **Baseline:** v1.1 (Tauri v2, Svelte 5, Testing suite, SeaORM)

---

## 1. Overview & Context (The "Why")

### 1.1. Problem Statement

Taiga.io is an effective project management tool, but its architecture isolates each project into a distinct silo. For users who manage or participate in multiple projects, this requires them to manually switch contexts—navigating into each project individually—to track, manage, and respond to issues. This process is time-consuming, inefficient, and increases the risk of missing critical updates or having slow response times, especially as the number of managed projects grows.

### 1.2. Vision & Goal

The vision for Larch is to create a centralized, multi-project issue management dashboard for Taiga.io that acts as a "single pane of glass" for issue triage and management.

**v2.0 Focus:** Full structural and visual overhaul. Transitioning from a page-based navigation to a modern, sidebar-driven interface using AI-generated layouts from **Google Stitch** as a development reference. This version focuses on high-frequency user actions: a more fluid sidebar-driven navigation, the ability to quickly share links, and seamless attachment handling via pasted screenshots to speed up reporting.

### 1.3. Key Terminology / Glossary

- **Taiga Instance:** A specific deployment of Taiga, either the official cloud version (`taiga.io`) or a self-hosted (on-premise) server.
- **Project:** A standard project workspace within a Taiga Instance.
- **Issue:** A ticket within the "Issues" module of a Taiga Project.
- **Issue Status:** A named state for issues configured per-project (e.g., "New", "In Progress", "Need info", "Closed", "Rejected").
- **is_closed:** A boolean flag on Issue Status indicating if this status represents a closed/resolved state.
- **Google Stitch:** An AI design tool used to generate HTML/CSS layouts that serve as a visual and structural reference for Svelte component development.
- **OpenCode:** Tool utilized alongside Stitch to implement and refine the logic for image pasting and other frontend functionalities.
- **Attachments:** Files and screenshots associated with an issue.

---

## 2. Goals & Non-Goals (The "Scope")

### 2.1. Goals (v2.0)

- [ ] **Stitch-based Redesign:** Use AI-generated layouts to implement a professional UI for login, navigation, and dashboards.
- [ ] **Sidebar Navigation:** Implement a persistent sidebar for switching between "Projects" and "Dashboard".
- [ ] **Overlay Issue Detail:** Replace the full-page view with a Sidebar Drawer that superimposes over the main table for faster context-switching.
- [ ] **Copy Taiga Link:** A quick action in the issue panel to copy the direct URL of the issue to the clipboard.
- [ ] **Clipboard Screenshot Support:** Allow users to paste images (Ctrl+V) directly into comments or descriptions, triggering an automatic upload to Taiga using **OpenCode**-refined logic.
- [ ] **Secure Credential Storage:** Maintain the use of the native OS keychain for auth/refresh tokens.
- [ ] **Desktop Application:** Standalone, cross-platform app built with Tauri v2.

### 2.2. Non-Goals

- **NO Advanced Collision Management:** Following user feedback, Git-like diff resolution is out of scope for v2.0 to optimize resources. Simple collision warnings will remain as per v1.1.
- **NO "Saved Filters" (Presets):** These remain a target for future versions (v2.1+).
- **NO support for other Taiga modules (User Stories, Tasks, Wiki, etc.)**.
- **NO administrative features (e.g., creating projects, managing users)**.
- **NO offline functionality or real-time auto-refresh**.

---

## 3. User Experience & Requirements (The "What")

### 3.1. User Persona

- **"Alex, the Multi-Project Manager"**: Oversees 10+ projects. Values efficiency and a modern interface that allows viewing details via an overlay without losing the context of the main aggregated list. Alex needs to report bugs quickly by pasting screenshots and share specific issues with the team via direct links.

### 3.2. User Journey / Flow (v2.0)

1. **Redesigned Login:** Entry point using layouts inspired by Google Stitch.
2. **Navigation Hub:** A persistent **Sidebar** allows Alex to switch between "Projects" (selection) and "Dashboard" (triage).
3. **The Dashboard (Triage):** An aggregated table with "Smart Default" filters.
4. **Overlay Interaction:** Clicking an issue opens the **Sidebar Drawer (Overlay)**. The table remains visible underneath, allowing Alex to keep the list in sight while editing.
5. **Issue Actions:** \* Alex clicks a "Link" icon to copy the direct Taiga URL for the current issue.

- Alex pastes a screenshot directly from the clipboard into the description or a comment; Larch uploads it and inserts the markdown.

6. **Data Feedback:** Every background action (like Refresh or Uploads) triggers a **Toast notification** with results.

### 3.3. User Stories

- **Auth & Nav:** - _As Alex,_ I want a modern, sidebar-driven navigation to switch instantly between project setup and my triage dashboard.
- **Issue Management:**
- _As Alex,_ I want to edit an issue in an overlay sidebar so I don't lose my place in the aggregated list.
- _As Alex,_ I want to paste screenshots directly into comments so I can report visual bugs much faster.
- _As Alex,_ I want to copy the direct Taiga link of an issue with one click to share it with my team.

---

## 4. System Architecture (The "How")

### 4.1. High-Level Diagram

`[ User's Desktop (Tauri v2) ]`
`|`
`+-- [ Frontend (Svelte 5 + TS) ]  <-- (Tauri Commands) --> [ Backend (Rust) ]`
`|       |      (Reference: Stitch & OpenCode)              |`
`|       +-- (Svelte Stores & Clipboard API)            +-- [ SQLite DB (SeaORM) ]`
`|                                                              |`
`|                                                      +-- [ OS Keychain (keyring-rs) ]`
`|                                                              |`
`|                                                      +-- [ Taiga API Service ]`

### 4.2. Technology Stack

- **Framework:** Tauri v2.
- **Backend:** Rust.
- **Frontend:** Svelte 5 + TypeScript.
- **Design Reference:** Google Stitch (AI-generated HTML/CSS as development source).
- **Implementation Support:** OpenCode for advanced frontend logic and feature refinement.
- **Styling:** Tailwind CSS.
- **Database:** SQLite via **SeaORM**.
- **Security:** `keyring-rs` for token management.

### 4.3. Attachment Upload Logic

1. **Frontend:** Intercept the `paste` event in the text editor.
2. **Processing:** Extract `Blob` data if the clipboard contains an image.
3. **Tauri Bridge:** Send the file data to the Rust backend via a dedicated command.
4. **API:** The Rust backend performs a `POST` to Taiga's attachment endpoint and returns the image URL.
5. **Completion:** The frontend appends the appropriate Markdown `![](url)` to the text area and triggers a Toast notification.

---

## 5. Implementation & Rollout Plan (v2.0)

### 5.1. Milestones

1. **M1: Login Redesign.** Apply Stitch-generated layout to the login screen.
2. **M2: Main Flow Redesign.**

- 2.1 **Sidebar Navigation:** Implement persistent navigation between projects and dashboard.
- 2.2 **Project Selector:** Redesign the project selection UI.
- 2.3 **Dashboard:** Redesign aggregated issue table and advanced filter UI.

3. **M3: Detail Sidebar (Overlay).** Implement the overlay sidebar component and the "Copy Link" functionality.
4. **M4: Attachment Management.** Implement the screenshot pasting logic and automatic upload to the Taiga API using **OpenCode** logic.

### 5.2. Testing Strategy

- **Rust Unit Tests:** Critical business logic, API service parsing for attachments, and link generation.
- **Integration Tests:** Tauri command chain from Svelte 5 to Rust, specifically for clipboard data handling.
- **Manual E2E:** Focus on sidebar navigation state, screenshot pasting across different OS, and link sharing accuracy.

---

## 6. Risks & Open Questions

- **Risk 1 (High):** Complexity of translating Stitch-generated HTML/CSS into reactive Svelte 5 components while maintaining consistency.
- **Risk 2 (Medium):** Handling different image formats from the clipboard across various operating systems (Windows vs. macOS).
- **Risk 3 (Medium):** Managing UI focus and scroll states when the sidebar overlay is open above the main table.
