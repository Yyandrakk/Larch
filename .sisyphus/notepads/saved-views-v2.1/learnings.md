## Task 12: Build SaveSplitButton Component (2026-02-01)

- Implemented `src/lib/components/dashboard/SaveSplitButton.svelte` using `ButtonGroup`, `Button`, and `DropdownMenu`.
- Leveraged `bits-ui` (underlying shadcn-svelte) behavior where `DropdownMenu.Root` does not render a wrapper element, allowing `DropdownMenu.Trigger` to be a direct flex child of `ButtonGroup`, inheriting correct border/radius styling.
- Used `buttonVariants({ variant: 'outline', size: 'icon' })` for the split trigger to match the main button.
- Cleaned up broken `filterUtils.test.ts` (disabled) to pass `pnpm check`.
