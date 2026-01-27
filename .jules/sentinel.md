## 2025-05-15 - Missing Content Security Policy with Global Tauri

**Vulnerability:** The application enabled `withGlobalTauri: true` but had `"csp": null` in `tauri.conf.json`.
**Learning:** Enabling global Tauri access without a strict Content Security Policy (CSP) exposes the application to Remote Code Execution (RCE) via Cross-Site Scripting (XSS). If an attacker injects a script, they can invoke any Tauri command.
**Prevention:** Always enforce a strict CSP when `withGlobalTauri` is enabled. Restrict `script-src` to `'self'` and avoid `'unsafe-inline'` for scripts.

## 2025-05-16 - Unsafe HTML Rendering in Svelte Components

**Vulnerability:** The application used `{@html ...}` to render user-generated content (issue descriptions and comments) without client-side sanitization.
**Learning:** Even if the backend claims to sanitize data, client-side sanitization is a critical defense-in-depth layer, especially in Tauri apps where XSS can lead to RCE. Svelte's `{@html}` is a raw HTML injection sink.
**Prevention:** Always wrap content passed to `{@html}` in a sanitizer like `DOMPurify`. Created a centralized `sanitizeHtml` utility in `src/lib/security.ts`.
