## 2025-05-15 - Missing Content Security Policy with Global Tauri

**Vulnerability:** The application enabled `withGlobalTauri: true` but had `"csp": null` in `tauri.conf.json`.
**Learning:** Enabling global Tauri access without a strict Content Security Policy (CSP) exposes the application to Remote Code Execution (RCE) via Cross-Site Scripting (XSS). If an attacker injects a script, they can invoke any Tauri command.
**Prevention:** Always enforce a strict CSP when `withGlobalTauri` is enabled. Restrict `script-src` to `'self'` and avoid `'unsafe-inline'` for scripts.
