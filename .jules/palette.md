## 2024-05-24 - Missing ARIA Labels on Layout Navigation Buttons

**Learning:** This app's layout components (such as Header and Sidebar) frequently use icon-only buttons for actions like notifications or dropdown menus without providing standard ARIA attributes.
**Action:** When adding or modifying global navigation or layout buttons, always ensure proper `aria-label`, `title`, and appropriate state attributes like `aria-expanded` and `aria-haspopup` are included to support screen readers and tooltips.
