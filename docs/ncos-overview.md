# NCOS Overview

NCOS is a lightweight, rolling-release Linux distribution based on Arch Linux with
the COSMIC desktop. It comes ready to use with tiling, themes, snapshots, and an
offline AI assistant already set up.

## What NCOS Is

- **Base distribution:** NCOS is based on Arch Linux.
- **Release model:** NCOS is a rolling release, so packages update continuously instead of in fixed version releases.
- **Desktop:** NCOS runs the COSMIC desktop environment by System76, on Wayland. The compositor is cosmic-comp.
- **Filesystem:** NCOS installs on btrfs by default, with zstd compression and subvolumes `@`, `@home`, `@root`, `@srv`, `@cache`, `@log`, and `@tmp`.
- **Package manager:** NCOS uses `pacman`, with `yay` for AUR packages and COSMIC Store for graphical app installs.
- **Current version:** The latest NCOS release is NCOS 2.1.3.

## What Is New in NCOS 2.1.3

- **Now Playing applet:** NCOS 2.1.3 adds a panel applet that shows the current song with album art.
- **Tiling Assistant:** NCOS 2.1.3 adds the Tiling Assistant, an app for choosing which apps open floating instead of tiled.
- **Ollama included:** Ollama now ships on the NCOS ISO, so the NCOS Assistant works out of the box.
- **Snapshots:** NCOS 2.1.3 tunes grub-btrfs so snapshot boot entries are named by date and description.

## Default Apps on NCOS

These apps come preinstalled on NCOS:

- **COSMIC Store:** Install apps with a graphical store.
- **COSMIC Monitor:** Monitor CPU, memory, and processes.
- **Timeshift:** Take system backups and snapshots.
- **LocalSend:** Send files to nearby phones and computers.
- **Popsicle:** Flash ISO images to USB drives.
- **GNOME Network Displays:** Cast your screen wirelessly to a TV.
- **Android Messages:** Send text messages from your desktop.
- **kitty:** The default NCOS terminal.
- **NCOS Assistant:** The offline AI assistant in the panel.

## Report a Bug or Contribute to NCOS

- **Report a bug:** To report a bug or suggest an idea for NCOS, open an issue on the NCOS GitHub repository at https://github.com/exothermic88/NCOS.
- **Contribute:** Patches and documentation fixes for NCOS are welcome on the same GitHub repository.
- **Download NCOS:** The latest NCOS ISO is on the NCOS website at https://exothermic88.github.io/.
- **Ask the Assistant:** You can ask questions about NCOS in the NCOS Assistant applet in the panel, which answers using this documentation.
