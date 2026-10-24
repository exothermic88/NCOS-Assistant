# NCOS Package Repository

NCOS has its own pacman repository, `[ncos]`, with first-party NCOS apps and panel
applets. It is already enabled on NCOS 2.0 and later.

## NCOS Packages

These packages are in the `[ncos]` repository:

- **ncos-assistant:** The NCOS Assistant, an offline AI assistant applet for the panel.
- **ncos-package-update-applet:** A panel applet that shows pending package updates and firmware updates.
- **ncos-weather-applet:** A panel applet that shows the weather.
- **ncos-now-playing-applet:** A panel applet that shows the song currently playing, with album art. It works with any MPRIS media player.
- **ncos-tiling-assistant:** The Tiling Assistant, an app for choosing which apps open floating instead of tiled.

## Install an NCOS Package

On NCOS 2.0 and later the `[ncos]` repository is already enabled, so you can install any NCOS package with `sudo pacman -S` followed by the package name.

Install the NCOS Assistant:

```bash
sudo pacman -S ncos-assistant
```

Install the NCOS weather applet and the Now Playing applet:

```bash
sudo pacman -S ncos-weather-applet ncos-now-playing-applet
```

## Update NCOS Packages

NCOS packages update together with the rest of the system. No extra command is needed:

```bash
sudo pacman -Syu
```

## Add the NCOS Repository on Older Installs

Systems installed before NCOS 2.0, or other Arch-based systems, need to add the repository by hand.

1. Open `/etc/pacman.conf` in a text editor as root.
2. Add these lines to the end of the file:

```
[ncos]
SigLevel = Optional TrustAll
Server = https://raw.githubusercontent.com/exothermic88/ncos-repo/main/$arch
```

3. Sync the package databases:

```bash
sudo pacman -Syy
```
