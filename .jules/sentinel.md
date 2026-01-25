# Sentinel's Journal

## 2026-01-25 - Missing Content Security Policy

**Vulnerability:** The application was configured with `"csp": null` and `withGlobalTauri: true`.
**Learning:** Enabling global Tauri API access without a strict CSP creates a high risk of RCE if XSS occurs.
**Prevention:** Always enforce a strict CSP (`default-src 'self'`) in Tauri applications, especially when `withGlobalTauri` is enabled.
