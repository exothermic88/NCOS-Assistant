# NCOS Overview

NCOS is an Arch Linux-based distribution built around the COSMIC desktop
environment. It ships as an archiso-based live image and follows Arch's
rolling-release model.

> This is a seed document for testing the assistant's document index.
> Replace or extend the files in this folder with the real NCOS wiki.

## Updating the system

NCOS uses pacman, the Arch Linux package manager. To update the entire
system, run:

```sh
sudo pacman -Syu
```

Reboot after kernel updates. Partial upgrades are unsupported: never run
`pacman -Sy` followed by installing packages without a full upgrade.

## Desktop environment

NCOS runs the COSMIC desktop environment by System76. Settings are managed
through COSMIC Settings, including panel applets, theming, and the frosted
glass appearance effect. The compositor is cosmic-comp on Wayland.

## Getting help

Questions about NCOS can be asked directly to the NCOS Assistant applet in
the panel, which answers using this documentation.
