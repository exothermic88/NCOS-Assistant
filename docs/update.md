# System Maintenance Guide

Maintaining NCOS keeps your system running smoothly. Each item below is written
as a plain-language statement with the exact command to run: updating packages,
cleaning up unused files, managing services, and checking logs and system
resources.

## Update the System

These statements cover updating NCOS packages:

- **Update all official packages:** The command to update all official NCOS packages is:

  ```bash
  sudo pacman -Syu
  ```

- **Update AUR packages only:** The command to update only AUR packages is:

  ```bash
  yay --aur
  ```

- **Full system upgrade including AUR:** The command for a full system upgrade including AUR packages is:

  ```bash
  yay
  ```

- **Reboot after kernel updates:** After an update that includes a new kernel, reboot NCOS so the new kernel takes effect.

## Clean Up the System and Free Disk Space

Cleaning up NCOS removes packages and cached files you no longer need, which
frees disk space. Always double-check the package list before deleting anything.

- **Remove unused (orphaned) packages:** Orphaned packages are dependencies no longer required by anything installed. The command to remove them is:

  ```bash
  sudo pacman -Rns $(pacman -Qdtq)
  ```

- **Clear the package cache:** The commands to free disk space by clearing the pacman cache are:

  ```bash
  sudo pacman -Sc   # Remove old package versions
  sudo pacman -Scc  # Remove all cached packages
  ```

## Manage Services

These statements cover managing systemd services on NCOS:

- **Check the status of a service:** The command to check a service's status is `systemctl status <service-name>`.
- **Enable a service at boot:** The command to enable a service so it starts at boot is `sudo systemctl enable <service-name>`.
- **Disable a service at boot:** The command to disable a service so it no longer starts at boot is `sudo systemctl disable <service-name>`.
- **Restart a running service:** The command to restart a running service is `sudo systemctl restart <service-name>`.

## Check System Logs

- **View recent system logs:** The command to view recent system logs for troubleshooting is `journalctl -xe`.
- **View logs for a specific service:** The command to view logs for one service is `journalctl -u <service-name> --no-pager`.

## Monitor System Resources

- **Check CPU and memory usage:** The command to monitor CPU and memory usage on NCOS is `htop`.
- **Check disk usage:** The command to check disk usage is `df -h`.
