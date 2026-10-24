# System Maintenance

How to keep NCOS updated and running well: updating packages, cleaning up disk
space, managing services, and reading logs.

## Update the System

Update all official packages (and NCOS packages) with pacman:

```bash
sudo pacman -Syu
```

Update everything, including AUR packages, with yay:

```bash
yay
```

Update only AUR packages:

```bash
yay --aur
```

## Update Rules: No Partial Upgrades and Reboot After Kernel Updates

- **No partial upgrades:** Partial upgrades are not supported on NCOS. Never run `sudo pacman -Sy` and then install a package without doing a full upgrade first. Always use `sudo pacman -Syu`.
- **Reboot after kernel updates:** After an update that includes a new Linux kernel, reboot so the new kernel is used.
- **Snapshot first:** Take a Timeshift snapshot before big updates so you can roll back if something breaks.

## Remove Unused (Orphaned) Packages

Orphaned packages were installed as dependencies but are no longer needed. Always check the package list before confirming.

Remove orphaned packages:

```bash
sudo pacman -Rns $(pacman -Qdtq)
```

## Clear the Package Cache to Free Disk Space

Remove old versions of cached packages:

```bash
sudo pacman -Sc
```

Remove all cached packages:

```bash
sudo pacman -Scc
```

## Manage Services with systemctl

Replace `<service-name>` with the name of the service, for example `ollama`.

Check whether a service is running:

```bash
systemctl status <service-name>
```

Start a service at boot:

```bash
sudo systemctl enable <service-name>
```

Stop a service from starting at boot:

```bash
sudo systemctl disable <service-name>
```

Restart a service:

```bash
sudo systemctl restart <service-name>
```

## Check System Logs with journalctl

View recent system logs with explanations, useful for troubleshooting:

```bash
journalctl -xe
```

View the logs of one service:

```bash
journalctl -u <service-name> --no-pager
```

## Check CPU, Memory, and Disk Usage

See CPU and memory usage and running processes in the terminal:

```bash
htop
```

See how much disk space is used and free:

```bash
df -h
```

You can also open COSMIC Monitor from the App Library (`Super+A`) for a graphical view.
