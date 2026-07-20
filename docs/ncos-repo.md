# Add the NCOS Repository

Follow these steps to add the NCOS package repository to your system so you can
install NCOS-specific packages with pacman.

## How to Add the NCOS Repository

1. **Open the pacman.conf file.** Open `/etc/pacman.conf` in a text editor:

   ```bash
   sudo nano /etc/pacman.conf
   ```

2. **Add the NCOS repository configuration.** Append the following lines to the
   end of the file:

   ```
   [ncos]
   SigLevel = Optional TrustAll
   Server = https://raw.githubusercontent.com/exothermic88/ncos-repo/main/$arch
   ```

3. **Synchronize the NCOS repository.** Run this command so pacman picks up the
   new repository. (For more about pacman package maintenance, see the Arch Wiki:
   https://wiki.archlinux.org/title/Pacman)

   ```bash
   sudo pacman -Syy
   ```

4. **Install an NCOS-specific package.** Use pacman to install any NCOS package,
   replacing `ncos-package_name` with the real package name:

   ```bash
   sudo pacman -S ncos-package_name
   ```

   For example, to install the NCOS hotfix package:

   ```bash
   sudo pacman -S ncos-hotfix
   ```

5. **Apply the package.** Run the package's command to apply it:

   ```bash
   ncos-package_name
   ```

   For example:

   ```bash
   ncos-hotfix
   ```

## Future Updates

When future updates are released for NCOS packages you have already installed,
they are downloaded automatically when you update your system with
`sudo pacman -Syu`.

To apply an update after it downloads, run the package command again:

```bash
ncos-package_name
```
