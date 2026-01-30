## 2025-05-15 - Missing Content Security Policy with Global Tauri

**Vulnerability:** The application enabled `withGlobalTauri: true` but had `"csp": null` in `tauri.conf.json`.
**Learning:** Enabling global Tauri access without a strict Content Security Policy (CSP) exposes the application to Remote Code Execution (RCE) via Cross-Site Scripting (XSS). If an attacker injects a script, they can invoke any Tauri command.
**Prevention:** Always enforce a strict CSP when `withGlobalTauri` is enabled. Restrict `script-src` to `'self'` and avoid `'unsafe-inline'` for scripts.

## 2025-05-15 - Unsafe Usage of `{@html}`
**Vulnerability:** The application rendered `issue.description_html` and `comment.comment_html` directly using `{@html}` without sanitization, exposing the app to Stored XSS.
**Learning:** Svelte's `{@html}` is raw and dangerous. Even "trusted" backend data should be sanitized on the client because Tauri context makes XSS more dangerous.
**Prevention:** Use `isomorphic-dompurify` to sanitize all HTML content before rendering. I created `src/lib/sanitize.ts` to centralize this logic.
