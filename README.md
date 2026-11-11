![Skyward Recovery](assets/hero.jpg)

# Skyward for Mi8937

Skyward is a recovery written in Rust. This port uses the original unified
Mi-Thorium TWRP 4.19 kernel and recovery base.

**Version:** 0.2.0 · **Maintainer:** alpha269 · **Kernel:** 4.19.312-Mi8937v2

[Download](https://github.com/console-ramoops/skyward/releases/tag/v0.2.0-mi8937-4.19) ·
[Port source](https://github.com/console-ramoops/skyward/tree/mi8937) ·
[Build and device details](https://github.com/console-ramoops/skyward/blob/mi8937/docs/mi8937_4_19.md)

## Devices

The unified kernel includes these device trees. Device identity and display
size are detected at boot.

| Device | Codename |
| --- | --- |
| Redmi 3S / 3X / 3S Prime | `land` |
| Redmi 4 / 4X | `santoni` |
| Redmi 4 Standard | `prada` |
| Redmi 4A | `rolex` |
| Redmi 5A | `riva` |
| Redmi Note 5A | `ugglite` |
| Redmi Note 5A Prime | `ugg` |

Earlier builds have boot feedback from Redmi 4X. The current image and the
other models still need hardware testing.

## Download and install

Download
[Skyward-0.2.0-mi8937-4.19-20261111.img](https://github.com/console-ramoops/skyward/releases/download/v0.2.0-mi8937-4.19/Skyward-0.2.0-mi8937-4.19-20261111.img)
from the release. An unlocked bootloader is required.

SHA-256:

```text
95ef14eaca1415d676b8a9e2d177edf6f31b65331a72c5c0fc5eac61b1273f1c
```

Enter fastboot mode, connect the phone to a computer, and run:

```sh
fastboot flash recovery Skyward-0.2.0-mi8937-4.19-20261111.img
fastboot boot Skyward-0.2.0-mi8937-4.19-20261111.img
```

From an existing TWRP recovery, choose **Install Image**, select the IMG, select **Recovery**,
then **Reboot Recovery**.

## Port features

- File manager, terminal, backups, image flashing and script-based ROM ZIP installation.
- ADB and MTP, with 16 KiB MTP download requests for the Qualcomm USB controller.
- Back, Home and Menu hardware-key support.
- Automatic data opening on ordinary recovery boots; locked user files request their credential on the Data page.
- English by default, with saved language choices preserved.
- Maintainer name in the recovery version, IST and WIB time zones, and 16 dp status bar padding.
- Timed vibrator configuration and framebuffer startup adjustments.

Wi-Fi is not configured. A/B switching, A/B OTA payload installation and
init_boot dual boot are unavailable on these single-slot devices. Logical
partitions depend on the installed ROM's retrofit dynamic-partition metadata.
The latest USB, vibration, input and encrypted-data changes need device testing.

## Build

The port source is on the `mi8937` branch. Install the Rust/Android build
prerequisites and font tools described in
[the port guide](https://github.com/console-ramoops/skyward/blob/mi8937/docs/mi8937_4_19.md),
then pack using the original TWRP 4.19 recovery image:

```sh
git clone --branch mi8937 https://github.com/console-ramoops/skyward.git
cd skyward
tools/build-mi8937-4.19.sh /path/to/twrp-4.19/recovery.img out/skyward-mi8937_4_19.img
```

397 supported tests, Clippy and the release build passed. Packing checks
verify the original kernel and all seven device trees, base runtime files,
and the 64 MiB recovery partition and 20 MiB compressed ramdisk limits.

For debugging, collect `/skyward/run/recovery.log`, `/tmp/recovery.log`, and
`adb shell dmesg`. For MTP, transfer a file larger than 1 MiB in both directions
and compare SHA-256 on the phone and computer.

## Credits and license

Skyward and upstream contributors, juezhi6668 for legacy recovery support,
and Mi-Thorium / TWRP for the recovery base and unified device support.

[GPL-3.0-or-later](LICENSE). Imported recovery files retain their original licenses.
