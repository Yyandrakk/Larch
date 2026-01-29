## 2025-05-15 - Missing Content Security Policy with Global Tauri

**Vulnerability:** The application enabled `withGlobalTauri: true` but had `"csp": null` in `tauri.conf.json`.
**Learning:** Enabling global Tauri access without a strict Content Security Policy (CSP) exposes the application to Remote Code Execution (RCE) via Cross-Site Scripting (XSS). If an attacker injects a script, they can invoke any Tauri command.
**Prevention:** Always enforce a strict CSP when `withGlobalTauri` is enabled. Restrict `script-src` to `'self'` and avoid `'unsafe-inline'` for scripts.

## 2025-05-15 - Unsafe Usage of Backend-Generated HTML

**Vulnerability:** The application rendered `description_html` and `comment_html` fields directly from the backend using `{@html}`.
**Learning:** Relying on external APIs (like Taiga) for HTML sanitization is risky, especially in a Tauri context where XSS leads to RCE. The backend might change its sanitization rules or be compromised.
**Prevention:** Prefer rendering the raw content (Markdown) on the client side using a secure renderer that escapes HTML by default. If using `{@html}`, always sanitize the input on the client side first.
