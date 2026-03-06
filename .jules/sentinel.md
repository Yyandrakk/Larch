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

## 2025-05-18 - Missing DOMPurify in Markdown Rendering

**Vulnerability:** The application rendered markdown to HTML using regex parsing and dynamically injected the result into the DOM via `{@html}`. This bypassed DOMPurify sanitization. If regex parsing had any flaws, a Cross-Site Scripting (XSS) payload could be executed, leading to Remote Code Execution (RCE) via Tauri global APIs.
**Learning:** Even if HTML is escaped before markdown parsing, the subsequent regex replacements can create unsafe HTML or be manipulated to execute scripts. All dynamically generated or transformed HTML, regardless of whether the source was raw user input or an intermediate markdown parser, MUST be passed through DOMPurify before rendering with Svelte's `{@html}`.
**Prevention:** Ensured `renderMarkdown` in `src/lib/utils/markdown.ts` wraps its final parsed output with `sanitizeHtml` from `$lib/sanitize`. Suppressed the Svelte warning with `<!-- eslint-disable-next-line svelte/no-at-html-tags -->` only after sanitization was guaranteed.
