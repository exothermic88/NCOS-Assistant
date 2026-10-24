# How-To Guides

Short step-by-step guides for everyday tasks on NCOS: mirroring or casting your
screen, sharing files, and flashing a USB drive.

## Mirror Your Screen to a Second Monitor or TV

To show the same picture on a second monitor or TV connected with HDMI or DisplayPort:

1. Open Settings with `Super+;`.
2. Go to Displays.
3. Select the second display and turn on mirroring. When mirroring is off, the displays extend instead.
4. Choose a resolution and refresh rate, then click Apply.

If the second display does not show up, check the cable first, then open Displays again. Displays are usually detected automatically.

## Cast Your Screen Wirelessly to a TV (Miracast)

NCOS includes GNOME Network Displays for wireless screen casting with Miracast:

1. Open Network Displays from the App Library (`Super+A`).
2. Turn on the TV or display and make it discoverable. Most smart TVs need Screen Mirroring or Miracast mode turned on in their settings.
3. Select the display from the list and click Connect.

To stop casting, open Network Displays again and click Disconnect. Wireless casting needs a Wi-Fi adapter that supports Wi-Fi Direct. Most laptop Wi-Fi cards do, but some USB adapters do not.

## Share Files with a Phone or Another Computer (LocalSend)

NCOS includes LocalSend, an open-source AirDrop alternative that sends files over your local network to Linux, Windows, macOS, Android, and iPhone:

1. Open LocalSend from the App Library (`Super+A`).
2. Install LocalSend on the other device from https://localsend.org/.
3. When both devices are on the same Wi-Fi network they find each other automatically. Pick a file and send it.

## Flash an ISO to a USB Drive (Popsicle)

To write an ISO image to a USB drive, for example to make a bootable installer, use Popsicle, which comes with NCOS:

1. Open Popsicle from the App Library (`Super+A`).
2. Choose the ISO image.
3. Select the USB drive and click Flash.

Flashing erases everything on the USB drive.
