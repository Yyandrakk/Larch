# Sentinel's Journal

## 2024-05-22 - Missing Content Security Policy

**Vulnerability:** The application was configured with `"csp": null` and `withGlobalTauri: true`.
**Learning:** This combination allows any XSS vulnerability to potentially escalate to Remote Code Execution (RCE) by accessing the injected `window.__TAURI__` API.
**Prevention:** Always define a strict CSP in `tauri.conf.json`, restricting `script-src` to `'self'` and blocking inline scripts where possible.
