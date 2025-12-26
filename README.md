# Larch

**Larch** is a modern, cross-platform desktop client for [Taiga.io](https://taiga.io/), designed to streamline issue management across multiple projects. It acts as a "single pane of glass" for developers and managers who need to triage, track, and respond to issues without constantly switching context between different project URLs.

![Larch Dashboard](./docs/screenshots/dashboard.png)
_(Note: Add a screenshot of the dashboard here if available)_

## ✨ Key Features (v1.0)

- **Unified Dashboard**: View issues from multiple projects in a single, filterable table.
- **Intelligent Triage**: Automatically filters for actionable (Open) issues by default.
- **Advanced Filtering**: Filter by Status, Project, and Assignee with powerful "Include/Exclude" logic.
- **Offline Drafts**: Descriptions and comments are auto-saved locally. Never lose your work if the internet drops.
- **Conflict Resolution**: Detects if an issue has been modified by someone else before you save, preventing accidental overwrites.
- **Cross-Platform**: Native support for **Linux** (Debian/Ubuntu/Fedora), **Windows**, and **macOS**.
- **Secure**: API tokens are stored securely in your operating system's native keychain.

## 🚀 Installation

Download the latest release for your operating system from the [Releases Page](../../releases).

### Linux

- **Debian/Ubuntu**: Download `.deb` and run `sudo dpkg -i larch_*.deb`.
- **Fedora/RHEL**: Download `.rpm` and run `sudo rpm -i larch_*.rpm`.
- **Universal**: Download `.AppImage`, make it executable (`chmod +x`), and run.

### Windows

- Download and run the `.msi` or `.exe` installer.

### macOS

- Download the `.dmg`, open it, and drag Larch to your Applications folder.

## 🛠️ Development

Larch is built with **Tauri v2**, **Rust**, and **Svelte 5**.

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Node.js](https://nodejs.org/) (LTS) & pnpm
- System dependencies for Tauri (see [Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites))

### Setup

1.  Clone the repository:

    ```bash
    git clone https://github.com/Yyandrakk/Larch.git
    cd Larch
    ```

2.  Install frontend dependencies:

    ```bash
    pnpm install
    ```

3.  Run in development mode:

    ```bash
    pnpm tauri dev
    ```

## 🏗️ Architecture

- **Frontend**: Svelte 5 + TypeScript + TailwindCSS + shadcn-svelte.
- **Backend**: Rust (Tauri).
- **Storage**: SQLite (local data/drafts) + OS Keyring (secrets).
- **Communication**: Tauri Commands (IPC) & Context7 MCP.

See [docs/architecture.md](docs/architecture.md) for details.
