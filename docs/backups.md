# Backups and Snapshots

NCOS installs on btrfs by default, so system snapshots are fast and use little
space. With Timeshift and grub-btrfs you can roll back a broken update by booting a
snapshot from the boot menu.

## Set Up Backups with Timeshift

Timeshift is preinstalled on NCOS and takes snapshots of your system so you can restore it if something breaks:

1. Open Timeshift from the App Library (`Super+A`).
2. On first run, choose the BTRFS snapshot type. BTRFS is recommended on NCOS.
3. Choose a schedule, such as daily or weekly, or take snapshots by hand with the Create button.

## Take a Snapshot Before an Update

Take a manual snapshot before big changes like major updates, kernel upgrades, or editing system config files.

Create a Timeshift snapshot from the terminal with a description:

```bash
sudo timeshift --create --comments "before update"
```

## Make Snapshots Bootable from GRUB (auto-snapshot)

The `auto-snapshot` command adds every Timeshift snapshot to the GRUB boot menu as a bootable entry. It turns on the grub-btrfs daemon, which watches for new snapshots and adds them to GRUB automatically. Snapshot boot entries are named by date and description, and Docker and container subvolumes are ignored.

Run it once to turn on bootable snapshots:

```bash
auto-snapshot
```

`auto-snapshot` only works on a btrfs root filesystem. New NCOS installs use btrfs by default, but if you picked a different filesystem during manual partitioning, this feature is not available.

## Restore NCOS After a Bad Update

If an update breaks your system, roll back to a snapshot:

1. Reboot and open the GRUB menu. The menu is hidden by default: hold `Shift` (BIOS) or press `Esc` repeatedly (UEFI) during boot.
2. Open the snapshots submenu and boot a snapshot from before the problem.
3. Once booted, open Timeshift and click Restore on that snapshot to make the rollback permanent.
