## 2024-05-14 - Layout Navigation Accessibility

**Learning:** Icon-only buttons in navigation (like Header Notifications) and stateful dropdowns (like the Header user menu) frequently lack crucial ARIA properties that define their purpose and state to screen readers, especially when labels are visually hidden on smaller breakpoints. Sidebar navigation also misses explicit active state indicators.
**Action:** Always ensure icon-only buttons include `aria-label` and `title`. For stateful dropdowns, add `aria-expanded` and `aria-haspopup`. For navigation menus, use `aria-current="page"` to indicate the currently active screen.
