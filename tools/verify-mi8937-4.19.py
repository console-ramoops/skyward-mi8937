#!/usr/bin/env python3
"""Check a packed port against its base without accessing a phone."""

import argparse
import hashlib
import lzma
import struct
from pathlib import Path


def image(path):
    data = path.read_bytes()
    assert data[:8] == b"ANDROID!", "not an Android boot image"
    kernel_size, _, ramdisk_size, _, second_size, _, _, page, version = struct.unpack_from("<9I", data, 8)
    assert version == 0 and page == 2048 and second_size == 0, "unexpected mi8937 boot layout"
    start = page + (kernel_size + page - 1) // page * page
    packed = data[start:start + ramdisk_size]
    assert len(packed) == ramdisk_size, "truncated ramdisk"
    return data, data[page:page + kernel_size], packed


def archive(packed):
    data = lzma.decompress(packed, format=lzma.FORMAT_ALONE)
    entries = {}
    pos = 0
    while data[pos:pos + 6] in (b"070701", b"070702"):
        fields = [int(data[pos + 6 + i * 8:pos + 14 + i * 8], 16) for i in range(13)]
        size, namesize = fields[6], fields[11]
        name = data[pos + 110:pos + 110 + namesize - 1].decode()
        pos = (pos + 110 + namesize + 3) // 4 * 4
        body = data[pos:pos + size]
        assert len(body) == size, f"truncated entry {name}"
        pos = (pos + size + 3) // 4 * 4
        if name == "TRAILER!!!":
            return entries
        assert name not in entries, f"duplicate entry {name}"
        # inode numbers change on repack; ownership, mode and device nodes
        # must stay, including the metadata of all the base's executables.
        entries[name] = (tuple(fields[1:6] + fields[7:11]), body)
    raise AssertionError("cpio trailer missing")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("base", type=Path)
    parser.add_argument("port", type=Path)
    args = parser.parse_args()
    base, original_kernel, original_packed = image(args.base)
    port, kernel, packed = image(args.port)
    assert kernel == original_kernel, "kernel or appended DTBs changed"
    assert port[:16] + port[20:1632] == base[:16] + base[20:1632], "boot addresses, command line or header fields changed"
    assert len(port) <= 64 << 20 and len(packed) <= 20 << 20, "partition or ramdisk limit exceeded"
    original, entries = archive(original_packed), archive(packed)
    for name in ("init", "system/bin/init", "system/bin/adbd", "system/bin/linker64", "system/bin/sh",
                 "system/lib64/libc.so", "system/lib64/libdl.so", "system/lib64/libm.so", "file_contexts.bin",
                 "system/lib64/hw/keystore.default.so", "system/lib64/hw/gatekeeper.default.so", "system/bin/qseecomd",
                 "system/etc/recovery.fstab.fbev2", "system/bin/merge-fstab.sh", "system/bin/determine-device.sh"):
        assert entries[name] == original[name], f"base file changed: {name}"
    assert not any(name == "twres" or name.startswith("twres/") for name in entries), "TWRP resources remain"
    assert b'mi8937_4_19' in entries["skyward/device.toml"][1], "unified configuration missing"
    wrapper = entries["system/bin/recovery"][1]
    assert wrapper.startswith(b"#!/system/bin/sh"), "startup wrapper missing"
    assert b"exec /system/bin/skyward recovery\n" in wrapper, "wrapper must select the recovery command"
    assert b"export TMPDIR=/tmp\n" in wrapper, "wrapper must provide the recovery shell's temporary directory"
    usb_rules = entries["init.recovery.usb.rc"][1]
    assert b"setprop sys.usb.ffs.mtp.ready 0" not in usb_rules, "base USB rules must not clear the MTP provider's readiness"
    mtp_rules = entries["system/etc/init/skyward-mtp.rc"][1]
    assert b"on post-fs && property:sys.usb.configfs=1\n" in mtp_rules, "MTP functionfs must be mounted before recovery starts"
    assert b"/idVendor 0x18D1\n" in mtp_rules and b"/idProduct 0x2D08\n" in mtp_rules, "MTP/ADB must retain the Mi-Thorium base composite IDs"
    assert entries["system/bin/skyward"][1].startswith(b"\x7fELF\x02"), "64-bit Skyward binary missing"
    for name in ("7za", "avbctl", "resetprop", "unzip", "zstd"):
        metadata, body = entries[f"system/bin/{name}"]
        assert metadata[0] & 0o170000 == 0o120000 and body == b"skyward", f"incorrect command link: {name}"
    print(f"Verified: unchanged kernel/DTBs and base runtime; {len(entries)} ramdisk entries")
    print(f"Image: {len(port)} / {64 << 20} bytes; LZMA ramdisk: {len(packed)} / {20 << 20} bytes")
    print(f"SHA-256: {hashlib.sha256(port).hexdigest()}")


if __name__ == "__main__":
    main()
