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

## 2025-05-18 - Sanitize Intermediate Markdown

**Vulnerability:** Markdown editors that generate HTML and render it dynamically using `{@html renderMarkdown(value)}` are vulnerable to Cross-Site Scripting (XSS). An attacker can inject malicious payloads within valid markdown links or images that bypass initial `escapeHtml` functions, e.g., `[click](javascript:alert(1))`.
**Learning:** Even if the initial raw text is escaped, the transformation process (regex matching) can introduce unsafe HTML structures. To properly prevent XSS in intermediate parsed markdown, the final transformed HTML string must be sanitized using `DOMPurify` (or `sanitizeHtml`) before rendering. Furthermore, when combining `sanitizeHtml` with custom image transformation logic (`transformImageUrls`), sanitization MUST occur beforehand to prevent standard DOMPurify rules from stripping out the valid custom domains and protocols.
**Prevention:** In `src/lib/utils/markdown.ts` (or equivalent parsing utilities), invoke `sanitizeHtml(html)` just prior to executing custom post-processing utilities (e.g. injecting `taiga-auth://` protocols). Then safely use `{@html renderMarkdown(value)}` alongside `<!-- eslint-disable-next-line svelte/no-at-html-tags -->`.
