# Release Notes

This page lists what changed in each NCOS release. Each entry is written as a
plain-language statement describing the feature or fix and how to get it.

## Hotfix 0.5.1

To get the enhancements in Hotfix 0.5.1, install or update the `ncos-hotfix`
package. See the Add NCOS Repository guide (ncos-repo.md) for how to install
NCOS packages, and the System Maintenance Guide (update.md) for updating.

### Features in Hotfix 0.5.1

- **Custom Brave browser extension:** Hotfix 0.5.1 adds a custom extension for the Brave browser that lets you pull a specific tab out into its own new window.
- **How to load the Brave extension:** To load the extension, open `brave://extensions/`, enable Developer Mode, click "Load unpacked", and select the **BraveExtension** folder in `~/.config/`.
- **How to detach a tab:** With the extension loaded, the shortcut to detach the current Brave tab into a new window is `Ctrl + Shift + X` (or your chosen shortcut).

### Fixes in Hotfix 0.5.1

- **Discord streaming issue:** Hotfix 0.5.1 resolves an issue where Discord streaming failed to initialize properly.
- **Known limitation of the Discord fix:** For the Discord streaming fix to work, you currently need to run `xwaylandvideobridge` before streaming, due to an issue with running the application on autostart. This will be fixed in a future update.
