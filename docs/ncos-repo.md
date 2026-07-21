# Add the NCOS Repository

This page explains how to add the NCOS package repository to your system so you
can install NCOS-specific packages with pacman. Each step below is written as a
plain-language statement with the exact command to run.

## How to Add the NCOS Repository

- **Step 1 — Open the pacman configuration file:** To add the NCOS repository, first open `/etc/pacman.conf` in a text editor:

  ```bash
  sudo nano /etc/pacman.conf
  ```

- **Step 2 — Add the NCOS repository entry:** Append the following lines to the end of `/etc/pacman.conf`:

  ```
  [ncos]
  SigLevel = Optional TrustAll
  Server = https://raw.githubusercontent.com/exothermic88/ncos-repo/main/$arch
  ```

- **Step 3 — Synchronize the repository:** The command to make pacman pick up the new NCOS repository is:

  ```bash
  sudo pacman -Syy
  ```

  For more about pacman package maintenance, see the Arch Wiki:
  https://wiki.archlinux.org/title/Pacman

## How to Install an NCOS Package

- **Install a package:** The command to install an NCOS-specific package is `sudo pacman -S ncos-package_name`, replacing `ncos-package_name` with the real package name. For example, to install the NCOS hotfix package:

  ```bash
  sudo pacman -S ncos-hotfix
  ```

- **Apply the package:** After installing an NCOS package, run the package's own command to apply it. For example, to apply the hotfix package:

  ```bash
  ncos-hotfix
  ```

## Future Updates to NCOS Packages

- **Updates download automatically:** When updates are released for NCOS packages you have installed, they are downloaded automatically when you update your system with `sudo pacman -Syu`.
- **Updates must be applied manually:** To apply an NCOS package update after it downloads, run the package command again (for example, `ncos-hotfix`).
