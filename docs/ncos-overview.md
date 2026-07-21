# NCOS Overview

NCOS is an Arch Linux–based distribution built around the COSMIC desktop
environment. Each statement below describes one aspect of NCOS: what it is
based on, how it updates, and what desktop it runs.

## What NCOS Is

- **Base distribution:** NCOS is based on Arch Linux and ships as an archiso-based live image.
- **Release model:** NCOS follows Arch's rolling-release model, so packages update continuously rather than in fixed version releases.

## Updating the System

- **Package manager:** NCOS uses pacman, the Arch Linux package manager.
- **Update everything:** The command to update the entire NCOS system is:

  ```sh
  sudo pacman -Syu
  ```

- **Reboot after kernel updates:** After an update that includes a new kernel, reboot NCOS so the new kernel takes effect.
- **Never do partial upgrades:** Partial upgrades are unsupported on NCOS — never run `pacman -Sy` and then install packages without doing a full system upgrade (`sudo pacman -Syu`) first.

## Desktop Environment

- **Desktop:** NCOS runs the COSMIC desktop environment by System76.
- **Settings:** On NCOS, panel applets, theming, and the frosted-glass appearance effect are all managed through COSMIC Settings.
- **Compositor:** The NCOS compositor is cosmic-comp, running on Wayland.

## Getting Help

- **Ask questions about NCOS:** You can ask questions about NCOS directly to the NCOS Assistant applet in the panel, which answers using this documentation.
