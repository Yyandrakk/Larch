## 2024-05-24 - Layout Header Accessibility Improvements

**Learning:** Icon-only and custom dropdown buttons in the application layout (like Header and Sidebar) lack necessary ARIA labels and state attributes out-of-the-box when using simple components/icons without a library wrapper like bits-ui.
**Action:** When adding new icon-only buttons or custom interactive UI elements, always remember to add `aria-label`, `title`, and state attributes like `aria-expanded` / `aria-haspopup` if it toggles a menu.
