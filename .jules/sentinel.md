## 2026-01-22 - Svelte {@html} XSS Risk

**Vulnerability:** The application renders HTML content from the backend using Svelte's `{@html}` tag without client-side sanitization. This occurs in `CommentList.svelte` (comments) and `IssueDetailSheet.svelte` (issue descriptions). While the backend (Taiga) is expected to sanitize content, trusting external input blindly violates defense-in-depth principles.
**Learning:** Svelte's `{@html}` is a raw HTML injection sink. If the backend is compromised or a vulnerability exists in the backend's sanitizer, the client is exposed to XSS.
**Prevention:**

1. Enable strict Content Security Policy (CSP) to block inline scripts and limit script sources.
2. If possible, use a client-side sanitizer (e.g., DOMPurify) before rendering HTML.
3. Use safer alternatives like `textContent` where HTML is not strictly required.
