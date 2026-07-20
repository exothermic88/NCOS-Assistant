# Pre-Installation Guide

Before installing NCOS, follow these steps to prepare your system: download the
ISO, flash it to a USB drive, and configure your BIOS to boot from that USB.

## Download the NCOS ISO

To download NCOS, visit the NCOS official website at
https://exothermic88.github.io and download the latest NCOS ISO file.

## Flash the ISO to a USB Drive

To install NCOS you need a bootable USB drive created from the NCOS ISO. Use one
of the two recommended tools below, depending on your current operating system.

### Creating a Bootable USB on Windows (using Rufus)

1. Download and install Rufus (https://rufus.ie/) on your Windows PC.
2. Open Rufus.
3. Select your USB drive from the "Device" dropdown menu.
4. Click "Select" and choose the NCOS ISO file.
5. Ensure the "Partition scheme" is set to **GPT**.
6. Leave the file system settings as default (usually FAT32).
7. Click "Start" to begin creating the bootable USB.
8. Once complete, safely eject the USB drive.

### Creating a Bootable USB on Linux (using Etcher)

1. Download and install Etcher (https://www.balena.io/etcher/) on your system.
2. Open Etcher.
3. Click "Flash from file" and select the NCOS ISO file.
4. Insert your USB drive, and Etcher will automatically detect it.
5. Click "Flash" to begin writing the ISO to the USB drive.
6. Wait for the process to complete.
7. Safely remove the USB drive from your system.

## Configure Your BIOS and Boot from USB

To boot into the NCOS live environment, configure your BIOS or UEFI as follows:

1. **Access the BIOS/UEFI:** Restart your computer and enter the BIOS/UEFI
   settings. The key to access the BIOS varies by manufacturer (commonly `F2`,
   `F10`, `Del`, or `Esc`). Consult your system's manual for details.
2. **Disable Secure Boot:** Locate the **Secure Boot** option in your BIOS
   settings and disable it to allow booting from the NCOS USB.
3. **Set USB as boot priority:** Navigate to the **Boot Menu** in your BIOS and
   set the USB drive as the first boot device.
4. **Save and exit:** Save your changes and exit the BIOS.

## Enter the Live Environment

1. Insert your bootable USB drive into the target machine.
2. Restart the system. It should boot directly into the NCOS live environment.
3. If it does not boot, ensure the USB is set as the first boot device in the
   BIOS settings.

Once you reach the live environment, you are ready to proceed with the
Installation Process (see Installation.md).
