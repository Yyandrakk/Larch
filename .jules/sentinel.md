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

## 2025-05-20 - Broadening Reverse Tabnabbing Protection

**Vulnerability:** The DOMPurify `afterSanitizeAttributes` hook only added `rel="noopener noreferrer"` when the `target` attribute was exactly `_blank`. It missed case-insensitive variations (like `_BLANK`) and custom window names (like `target="external"`), which also open new browsing contexts and can be exploited for reverse tabnabbing.
**Learning:** Checking for an exact match of `target="_blank"` is insufficient. Any target that opens a new browsing context (anything other than `_self`, `_parent`, `_top`) exposes `window.opener` in older or misconfigured browsers.
**Prevention:** Enforce `rel="noopener noreferrer"` on all `<a>` tags with a `target` attribute that doesn't explicitly target the current browsing context (e.g., filter out `_self`, `_parent`, `_top`).
