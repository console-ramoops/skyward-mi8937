![Skyward Recovery](assets/hero.jpg)

# Skyward for Mi8937

Skyward is a recovery written in Rust. This port uses the original unified
Mi-Thorium TWRP 4.19 kernel and recovery base.

**Release:** 0.2.0 (Mi8937_4_19) · **Updated:** 2026-11-12
**Maintainer:** alpha269 · **Kernel:** 4.19.312-Mi8937v2

[Download](https://github.com/console-ramoops/skyward-mi8937/releases/tag/v0.2.0-mi8937-4_19) ·
[Development branch](https://github.com/console-ramoops/skyward-mi8937/tree/mi8937) ·
[Build and device details](docs/mi8937_4_19.md) ·
[简体中文](README.zh-CN.md)

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

## Download and install

Download
[Skyward-0.2.0-Mi8937_4_19.img](https://github.com/console-ramoops/skyward-mi8937/releases/download/v0.2.0-mi8937-4_19/Skyward-0.2.0-Mi8937_4_19.img)
from the release. An unlocked bootloader is required.

SHA-256:

```text
d3e64c39bf93b8b5df9c544faa69621c2aa1f07691e0d3c1308e1c221924414f
```

Enter fastboot mode, connect the phone to a computer, and run:

```sh
fastboot flash recovery Skyward-0.2.0-Mi8937_4_19.img
fastboot boot Skyward-0.2.0-Mi8937_4_19.img
```

From an existing TWRP recovery, choose **Install Image**, select the IMG, select **Recovery**,
then **Reboot Recovery**.

## Port features

- File manager, terminal, backups, image flashing and script-based ROM ZIP installation.
- ADB and MTP for internal storage and the mounted microSD card, with 16 KiB MTP download requests for the Qualcomm USB controller.
- MicroSD appears as a separate MTP storage even while internal storage is locked; existing card mounts are reused after a recovery restart.
- 267 DPI and scrolling controls, with page rebuilding when updated readings change its height during background drawing.
- Single-slot ZIP installation without A/B metadata requirements, and native device-property detection.
- Back, Home and Menu hardware-key support.
- Automatic data opening on ordinary recovery boots; locked user files request their credential on the Data page.
- English by default, with saved language choices preserved.
- Maintainer name in the recovery version, IST and WIB time zones, and 16 dp status bar padding.
- Timed vibrator configuration and framebuffer startup adjustments.

Wi-Fi is not configured. A/B switching, A/B OTA payload installation and
init_boot dual boot are unavailable on these single-slot devices. Logical
partitions depend on the installed ROM's retrofit dynamic-partition metadata.

## File transfer

Enable **Settings → File transfer**, then reconnect the USB cable. The computer
should show internal storage when it is unlocked and the mounted microSD card
as a separate storage. With internal storage locked, the microSD remains available.

## Build

To build the current image, use the
[matching release source snapshot](https://github.com/console-ramoops/skyward-mi8937/releases/download/v0.2.0-mi8937-4_19/Skyward-0.2.0-mi8937-4_19-20261112-source.tar.gz).
Install the Rust/Android prerequisites and font tools in [the port guide](docs/mi8937_4_19.md),
then pack using the original TWRP 4.19 recovery image:

```sh
curl -fL -o skyward-source.tar.gz https://github.com/console-ramoops/skyward-mi8937/releases/download/v0.2.0-mi8937-4_19/Skyward-0.2.0-mi8937-4_19-20261112-source.tar.gz
tar -xzf skyward-source.tar.gz
cd skyward
CARGO_BUILD_JOBS=1 tools/build-mi8937-4.19.sh /path/to/twrp-4.19/recovery.img out/skyward-mi8937_4_19.img
```

The development branch is `mi8937`. On Termux, one compiler job limits memory
use; a larger host can set `CARGO_BUILD_JOBS` to its available CPU count.

Packing checks verify the original kernel and all seven device trees, base runtime files,
and the 64 MiB recovery partition and 20 MiB compressed ramdisk limits.

For debugging, collect `/skyward/run/recovery.log`, `/tmp/recovery.log`, and
`adb shell dmesg`. Rust panic text is also written to the kernel log.

## Credits and license

Skyward and upstream contributors, juezhi6668 for legacy recovery support,
and Mi-Thorium / TWRP for the recovery base and unified device support.

[GPL-3.0-or-later](LICENSE). Imported recovery files retain their original licenses.
