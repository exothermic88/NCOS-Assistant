# Pre-Installation Guide

This guide explains how to prepare for installing NCOS. Each step below is
written as a plain-language statement so you can find exactly what to do:
download the ISO, flash it to a USB drive, and configure your BIOS to boot from
that USB.

## Download the NCOS ISO

- **Where to download NCOS:** The place to download the latest NCOS ISO file is the NCOS official website at https://exothermic88.github.io.

## Flash the ISO to a USB Drive

To install NCOS you need a bootable USB drive created from the NCOS ISO. The
recommended tool depends on your current operating system: use Rufus on Windows,
or Etcher on Linux.

### Creating a Bootable USB on Windows (using Rufus)

To create an NCOS bootable USB on Windows, use Rufus:

- **Get Rufus:** Download and install Rufus from https://rufus.ie/ on your Windows PC, then open it.
- **Pick the USB drive:** In Rufus, select your USB drive from the "Device" dropdown menu.
- **Pick the ISO:** In Rufus, click "Select" and choose the NCOS ISO file.
- **Set the partition scheme:** In Rufus, ensure the "Partition scheme" is set to GPT. Leave the file system settings as default (usually FAT32).
- **Write the USB:** In Rufus, click "Start" to begin creating the bootable USB, and safely eject the drive once it completes.

### Creating a Bootable USB on Linux (using Etcher)

To create an NCOS bootable USB on Linux, use Etcher:

- **Get Etcher:** Download and install Etcher from https://www.balena.io/etcher/ on your system, then open it.
- **Pick the ISO:** In Etcher, click "Flash from file" and select the NCOS ISO file.
- **Pick the USB drive:** Insert your USB drive and Etcher will detect it automatically.
- **Write the USB:** In Etcher, click "Flash" to begin writing the ISO, wait for it to complete, then safely remove the USB drive.

## Configure Your BIOS and Boot from USB

To boot into the NCOS live environment, configure your BIOS or UEFI as follows:

- **Access the BIOS/UEFI:** To enter the BIOS/UEFI settings, restart your computer and press the manufacturer's setup key — commonly `F2`, `F10`, `Del`, or `Esc`. Consult your system's manual for the exact key.
- **Disable Secure Boot:** In the BIOS settings, locate the Secure Boot option and disable it — Secure Boot must be off to boot the NCOS USB.
- **Set USB as boot priority:** In the BIOS Boot Menu, set the USB drive as the first boot device.
- **Save and exit:** Save your BIOS changes and exit.

## Enter the Live Environment

- **Boot the live environment:** Insert the bootable USB into the target machine and restart it — the system should boot directly into the NCOS live environment.
- **If it does not boot:** If the machine does not boot into the NCOS live environment, check that the USB is set as the first boot device in the BIOS settings.
- **Next step:** Once you reach the live environment, continue with the Installation Guide (Installation.md).
