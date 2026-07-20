# Release Notes

## Hotfix 0.5.1

Install or update the `ncos-hotfix` package to get the enhancements below. See
the Add NCOS Repository guide (ncos-repo.md) and the System Maintenance Guide
(update.md) for how to install it.

### Features

**Custom Brave Browser Extension.** Added a custom extension for the Brave
browser that lets you pull a specific tab out into its own new window.

To apply the Brave update:

1. **Load the extension.** Open `brave://extensions/`, enable **Developer
   Mode**, click **Load unpacked**, and select the **BraveExtension** folder in
   `~/.config/`.
2. **Use the shortcut.** Press `Ctrl + Shift + X` (or your chosen shortcut) to
   detach the current tab into a new window.

### Fixes

**Discord streaming issue.** Resolved an issue where Discord streaming failed to
initialize properly.

Note: for this fix to work, you currently need to run `xwaylandvideobridge`
before streaming, due to an issue with running the application on autostart.
This will be fixed in a future update.
