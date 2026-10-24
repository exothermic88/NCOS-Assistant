# Themes and Wallpapers

NCOS has a theme system that restyles COSMIC, the kitty terminal, and VS Code
together and switches the wallpaper to match, all with one shortcut.

## NCOS Themes

NCOS includes four themes: Nord (the default), Red, Purple, and Green. Switching themes changes the COSMIC desktop colors, the kitty terminal colors, the VS Code colors, and the wallpaper at the same time. NCOS remembers your active theme across reboots. The active theme is stored in `~/.current_theme`.

## Change the Theme

- **Next theme:** Press `Super+Shift+]` to switch to the next theme. This runs `theme-select.sh`.
- **Previous theme:** Press `Super+Shift+[` to switch to the previous theme. This runs `theme-select-previous.sh`.
- **Light or dark mode:** Press `Super+Ctrl+]` to toggle between light and dark mode. This runs `toggle_light_dark`.

When you switch themes, your current COSMIC, kitty, and VS Code settings are saved into the active theme first, so changes you make to a theme are kept the next time you use it.

## Change the Wallpaper

Press `Super+Shift+W` to open the wallpaper picker, a grid of wallpaper thumbnails. Select a wallpaper and it is applied right away.

Picking a wallpaper also switches the theme. Each wallpaper belongs to a theme, so choosing a wallpaper from the green folder applies the Green theme too.

## Add Your Own Wallpapers

Wallpapers are stored in `~/.config/wallpapers/`, with one folder per theme: `nord`, `red`, `purple`, and `green`. To add your own wallpaper, copy the image into the folder of the theme you want it to use. It will appear in the wallpaper picker (`Super+Shift+W`), and picking it switches NCOS to that folder's theme.

## Span One Wallpaper Across Two Monitors (Panorama)

NCOS uses the `awww` wallpaper daemon, which can split one wide image across multiple monitors. Any wallpaper with `dual` in its file name is split across your monitors automatically. For example, name a wide image `mountains-dual.png`, put it in your theme's wallpaper folder, and select it with `Super+Shift+W`. NCOS handles each monitor's position and resolution.
