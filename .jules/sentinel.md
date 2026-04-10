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

## 2025-05-18 - Client-Side Markdown Rendering XSS

**Vulnerability:** The `renderMarkdown` function escaped HTML entities _before_ applying regex-based transformations for markdown (links, images, etc.). While this prevents basic XSS, the `href` and `src` attributes were not fully validated, allowing payload injections like `![alt](http://test"onerror="alert(1))`. When this markdown was converted to HTML, the malformed URL attributes could execute arbitrary JavaScript. The component rendering the markdown used Svelte's `{@html}` directly without running the output through a sanitizer.
**Learning:** Manual regex parsing for markdown is notoriously difficult to secure against XSS. Escaping initial HTML does not protect against injected attributes during the markdown-to-HTML conversion phase. Svelte's `{@html}` explicitly bypasses its built-in security, requiring manual sanitization.
**Prevention:** Always run user-generated or manually transformed HTML through a robust sanitizer like DOMPurify (e.g., our `sanitizeHtml` wrapper) _immediately before_ rendering it with `{@html}` or returning it from utility functions. This acts as a reliable defense-in-depth measure against flaws in parsing logic.
