# Issues - v2-release

## Clipboard & Paste Issues

- **Share Button**: The share button was failing with `NotAllowedError` because it awaited a Tauri command (`CMD_GET_TAIGA_BASE_URL`) before calling `navigator.clipboard.writeText`. This broke the "user gesture" requirement.
  - **Fix**: Pre-fetch `taigaBaseUrl` when loading issue data and cache it in state. Use the cached URL synchronously in the click handler.
- **Paste-to-Upload**: Pasting images was unreliable because `for...of` loop over `DataTransferItemList` (clipboard items) is not consistently supported or behaves unexpectedly in some WebViews/environments.
  - **Fix**: Replaced `for (const item of items)` with standard `for (let i = 0; i < items.length; i++)` loop in both `MarkdownEditor.svelte` and `IssueDetailSheet.svelte`.
