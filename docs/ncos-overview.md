# NCOS Overview

NCOS is an Arch Linux–based distribution built around the COSMIC desktop
environment. It ships as an archiso-based live image and follows Arch's
rolling-release model, so packages update continuously rather than in fixed
version releases.

## Updating the system

NCOS uses pacman, the Arch Linux package manager. To update the entire system,
run:

```sh
sudo pacman -Syu
```

Reboot after kernel updates. Partial upgrades are unsupported: never run
`pacman -Sy` and then install packages without doing a full system upgrade
first.

## Desktop environment

NCOS runs the COSMIC desktop environment by System76. Settings are managed
through COSMIC Settings, including panel applets, theming, and the frosted-glass
appearance effect. The compositor is cosmic-comp, running on Wayland.

## Getting help

You can ask questions about NCOS directly to the NCOS Assistant applet in the
panel, which answers using this documentation.
