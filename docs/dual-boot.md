# Dual Boot

NCOS boots instantly by default because the GRUB menu is hidden with a zero-second
timeout. If you dual boot with Windows or another Linux, use `multiboot-toggle` to
show a boot menu that finds your other operating systems.

## Dual Boot with Windows or Another OS (multiboot-toggle enable)

Turn on the multiboot menu:

```bash
sudo multiboot-toggle enable
```

This switches GRUB to the multiboot profile and regenerates the boot configuration with os-prober turned on. Other installed operating systems, like Windows, are detected and added to the GRUB menu automatically. On the next reboot you will see a menu listing every OS. If you install another OS after enabling multiboot, run `sudo multiboot-toggle enable` again to detect it.

## Go Back to Instant Boot (multiboot-toggle disable)

Turn off the multiboot menu and restore the hidden, zero-timeout GRUB menu for the fastest boot into NCOS:

```bash
sudo multiboot-toggle disable
```

## Open the Hidden GRUB Menu

Even when the GRUB menu is hidden, you can still open it during boot. Hold `Shift` on BIOS systems, or press `Esc` repeatedly on UEFI systems. This is how you reach the snapshot boot entries without turning on the full multiboot menu.
