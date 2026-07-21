# NCOS Installation Guide

This guide explains how to install NCOS from the live environment after you have
created a bootable USB (see the Pre-Installation Guide). Each step below is
written as a plain-language statement so you can find exactly what to do at each
stage: boot the live environment, set your region and keyboard, partition the
drive, create a user, and begin the installation.

## Boot into the Live Environment

These statements cover starting the NCOS installer:

- **Start the NCOS installer:** To start installing NCOS, insert the NCOS installation media (USB or ISO) and boot the computer from it.
- **The installer opens automatically:** When the NCOS live environment loads, the installer launches automatically — you do not need to start it yourself.

## Select Region, Timezone, and System Language

- **Set the region and timezone:** In the NCOS installer, choose your preferred region and timezone from the list.
- **Set the system language:** In the NCOS installer, select the system language you want your installation to use.

## Configure the Keyboard Layout

- **Choose a keyboard layout:** In the NCOS installer, pick your keyboard layout from the available options.
- **Verify the keyboard layout:** Use the installer's test field to type and confirm the layout is correct before continuing.

## Partition the Drive

In the NCOS installer, choose the target drive for installation, then select one
of the four partitioning methods:

- **Install alongside an existing OS:** The "Install alongside" option keeps your current partitions and installs NCOS next to them (dual boot).
- **Replace a partition:** The "Replace a partition" option overwrites one existing partition with NCOS and leaves the others untouched.
- **Erase the disk:** The "Erase disk" option deletes all partitions on the drive and installs NCOS on a clean disk. This removes all existing data on that drive.
- **Manual partitioning:** The "Manual partitioning" option lets advanced users create and assign custom partitions themselves.

## Create a User Account

- **Create your user:** In the NCOS installer, enter a username and password for your account.
- **Choose the login behavior:** You can optionally enable automatic login, or require the password at startup.

## Begin the Installation

- **Start the installation:** After reviewing your settings, click "Install" to begin installing NCOS.
- **Finish and reboot:** Wait for the installation to complete, then reboot the computer into your new NCOS system.
