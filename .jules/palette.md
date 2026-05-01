## 2024-11-20 - Layout Navigation Button Accessibility

**Learning:** Layout navigation elements, like Header icon buttons or Sidebar navigation links, often lack explicit accessibility states for custom interactions and current page indication. Svelte handles boolean attributes smoothly when evaluated to `undefined`, making conditional rendering of ARIA states like `aria-current={currentScreen === 'dashboard' ? 'page' : undefined}` very clean and spec-compliant for absent states.
**Action:** When implementing or modifying layout navigation components, ensure that custom dropdown toggles have `aria-expanded` and `aria-haspopup` attributes, that icon-only buttons have descriptive `aria-label`s, and that active page tabs/buttons use `aria-current="page"`.
