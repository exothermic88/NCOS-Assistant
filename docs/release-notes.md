# Release Notes

What changed in each NCOS release. The latest release is NCOS 2.1.3.

## NCOS 2.1.3 New Features

- **Now Playing applet:** The new `ncos-now-playing-applet` shows the current track in the panel with album art and artist and title, for any MPRIS media player. It is in the panel by default.
- **Tiling Assistant:** The new `ncos-tiling-assistant` app manages which apps open floating instead of tiled. COSMIC Settings, Discord, and Spotify float by default.
- **Ollama included:** `ollama` now ships on the ISO, so the NCOS Assistant works out of the box.
- **grub-btrfs config:** Snapshot boot entries are named by date and description, and Docker and container subvolumes are ignored.
- **Accessibility zoom:** Zoom is preconfigured with mouse zoom shortcuts and 50% steps.
- **Installer slideshow:** The installer slideshow has new artwork and shortcut cards matching the current defaults.

## NCOS 2.1.3 Changes and Removals

- **Borderless panel:** The panel is borderless by default.
- **broadcom-wl removed:** The `broadcom-wl` driver is no longer on the ISO.
- **ncos-theming no longer installed:** Theming files now ship directly in `/usr`.
- **Config cleanup:** Leftover settings for applets NCOS does not ship were removed.

## NCOS 2.0 New Features

- **NCOS Assistant:** A local AI assistant applet in the panel that runs offline with Ollama and answers questions using the NCOS documentation.
- **First-party applets:** `ncos-package-update-applet` and `ncos-weather-applet` ship in the panel.
- **Theme system:** Themes restyle COSMIC, kitty, and VS Code together. Cycle themes with `Super+Shift+]` and `Super+Shift+[`.
- **Wallpaper picker:** `Super+Shift+W` opens a wallpaper grid, using the `awww` wallpaper daemon with panorama splitting for dual-monitor wallpapers.
- **btrfs by default:** The installer creates a btrfs filesystem with subvolumes and zstd compression.
- **Bootable snapshots:** The `auto-snapshot` command adds Timeshift snapshots to the GRUB menu with grub-btrfs.
- **Dual-boot toggle:** `multiboot-toggle enable` or `disable` switches GRUB between instant boot and a multiboot menu.
- **Redesigned installer:** New Calamares branding and slideshow.

## NCOS 2.0 New Default Apps

- **COSMIC Store:** Graphical app installation.
- **COSMIC Monitor:** System monitoring.
- **LocalSend:** Share files with nearby devices.
- **Popsicle:** Flash USB drives.
- **Android Messages:** Send text messages from your desktop.
- **rofi:** Powers the wallpaper picker.

## NCOS 2.0 Changes and Removals

- **Pamac removed:** Use `pacman`, `yay`, or COSMIC Store to install software instead.
- **KDE Connect not preinstalled:** Install `kdeconnect` yourself if you use it.
- **Legacy xf86-video drivers dropped:** The modern modesetting driver is used instead.
- **Slimmer system:** The Qt6 group was trimmed, saving over a gigabyte of disk space.
- **Faster ISO:** The ISO uses zstd compression for faster boot and install.
