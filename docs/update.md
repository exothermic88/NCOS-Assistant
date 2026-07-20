# System Maintenance Guide

Maintaining NCOS keeps your system running smoothly. This guide covers updating
packages, cleaning up unused files, managing services, and checking logs and
system resources.

## Update the System

To update all official packages, run:

```bash
sudo pacman -Syu
```

To update AUR packages only, run:

```bash
yay --aur
```

For a full system upgrade including AUR packages, run:

```bash
yay
```

Reboot after kernel updates.

## Clean Up the System and Free Disk Space

Cleaning up NCOS removes packages and cached files you no longer need, which
frees disk space. This covers removing orphaned packages and clearing the pacman
package cache. Always double-check the package list before deleting anything.

**Remove unused (orphaned) packages.** Orphaned packages are dependencies that
are no longer required by anything installed. To remove them:

```bash
sudo pacman -Rns $(pacman -Qdtq)
```

**Clear the package cache.** To free up disk space by clearing the pacman cache:

```bash
sudo pacman -Sc   # Remove old package versions
sudo pacman -Scc  # Remove all cached packages
```

## Manage Services

Check the status of a service:

```bash
systemctl status <service-name>
```

Enable a service to start at boot:

```bash
sudo systemctl enable <service-name>
```

Disable a service so it no longer starts at boot:

```bash
sudo systemctl disable <service-name>
```

Restart a running service:

```bash
sudo systemctl restart <service-name>
```

## Check System Logs

View recent system logs for troubleshooting:

```bash
journalctl -xe
```

View logs for a specific service:

```bash
journalctl -u <service-name> --no-pager
```

## Monitor System Resources

Check CPU and memory usage:

```bash
htop
```

Check disk usage:

```bash
df -h
```
