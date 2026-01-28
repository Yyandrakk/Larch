## 2025-05-15 - Missing Content Security Policy with Global Tauri

**Vulnerability:** The application enabled `withGlobalTauri: true` but had `"csp": null` in `tauri.conf.json`.
**Learning:** Enabling global Tauri access without a strict Content Security Policy (CSP) exposes the application to Remote Code Execution (RCE) via Cross-Site Scripting (XSS). If an attacker injects a script, they can invoke any Tauri command.
**Prevention:** Always enforce a strict CSP when `withGlobalTauri` is enabled. Restrict `script-src` to `'self'` and avoid `'unsafe-inline'` for scripts.

## 2025-05-15 - Unsanitized HTML Injection in Svelte Components

**Vulnerability:** The application rendered user-generated HTML content (issue descriptions and comments) using `{@html ...}` without client-side sanitization.
**Learning:** In a Tauri application with `withGlobalTauri: true`, any XSS vulnerability can escalate to Remote Code Execution (RCE) because injected scripts can access the Tauri API (`window.__TAURI__`). Trusting the backend to provide safe HTML is insufficient defense-in-depth.
**Prevention:** Always use a client-side sanitizer (like `dompurify`) when using `{@html}` to render content that originates from user input, even if it comes from a trusted API.
