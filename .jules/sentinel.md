## 2025-05-15 - Missing Content Security Policy with Global Tauri

**Vulnerability:** The application enabled `withGlobalTauri: true` but had `"csp": null` in `tauri.conf.json`.
**Learning:** Enabling global Tauri access without a strict Content Security Policy (CSP) exposes the application to Remote Code Execution (RCE) via Cross-Site Scripting (XSS). If an attacker injects a script, they can invoke any Tauri command.
**Prevention:** Always enforce a strict CSP when `withGlobalTauri` is enabled. Restrict `script-src` to `'self'` and avoid `'unsafe-inline'` for scripts.

## 2025-05-16 - Secure External Links with DOMPurify

**Vulnerability:** DOMPurify strips `target="_blank"` by default. If allowed, it creates a "Reverse Tabnabbing" risk where the opened page can access `window.opener`.
**Learning:** To support opening links in new tabs securely, one must explicitly allow `target` in DOMPurify AND enforce `rel="noopener noreferrer"`. Since DOMPurify hooks are global/singleton, wrapping them in `try/finally` is crucial to avoid side effects.
**Prevention:** Use a centralized `sanitizeHtml` wrapper that adds the `afterSanitizeAttributes` hook temporarily.

## 2025-05-18 - Global DOMPurify Hooks

**Vulnerability:** The `sanitizeHtml` function was adding and removing a DOMPurify hook on every call. `DOMPurify.removeHook` removes _all_ hooks for a given entry point, potentially disabling other security hooks and causing race conditions.
**Learning:** DOMPurify hooks modify the global instance. Adding/removing them dynamically is unsafe in a modular application.
**Prevention:** Register DOMPurify hooks once at the module top-level (on import) to ensure they are permanently active and do not interfere with other components.

## 2025-05-20 - Sanitize Custom Protocol Transformations

**Vulnerability:** User-generated content passing through regex-based Markdown parsers without an explicit DOMPurify sanitization step can result in XSS, even if `escapeHtml` is used initially. Additionally, `DOMPurify` strips custom protocols like `taiga-auth://` by default.
**Learning:** We must apply `sanitizeHtml` to the raw generated HTML _before_ running any custom transformations (like `transformImageUrls`) that inject non-standard protocol handlers. Applying DOMPurify after the custom protocol is injected strips the custom scheme, breaking functionality, but omitting it entirely exposes the application to XSS.
**Prevention:** Always follow the pattern: `transformImageUrls(sanitizeHtml(regexParsedHtml))` when rendering user-generated content that relies on both DOMPurify and custom protocols.
