#!/bin/sh
set -eu

if [ "$#" -ne 2 ]; then
    echo "Usage: $0 BASE_RECOVERY.img OUTPUT.img" >&2
    exit 2
fi
port_root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
base_image=$(realpath "$1")
output_image=$(realpath -m "$2")
test "$base_image" != "$output_image" || { echo "Output must differ from the base image" >&2; exit 2; }
cd "$port_root"
test -r "$base_image"
port_native=false

if [ -z "${SKYWARD_BINARY:-}" ] || [ -z "${SKYWARD_PACKER:-}" ]; then
    port_host=$(rustc -vV | sed -n 's/^host: //p')
    case "$port_host" in
        aarch64-linux-android)
            command -v patchelf >/dev/null || { echo "Native Android builds require patchelf" >&2; exit 2; }
            port_native=true
            cargo build --locked --release -p skyward -p skyward-pack
            SKYWARD_BINARY="$port_root/target/release/skyward"
            SKYWARD_PACKER="$port_root/target/release/skyward-pack"
            ;;
        *)
            cargo ndk -t arm64-v8a --platform 30 build --locked --release -p skyward
            cargo build --locked --release -p skyward-pack
            SKYWARD_BINARY="$port_root/target/aarch64-linux-android/release/skyward"
            SKYWARD_PACKER="$port_root/target/release/skyward-pack"
            ;;
    esac
fi

mkdir -p target/mi8937_4_19 "$(dirname -- "$output_image")"
if [ "$port_native" = true ]; then
    cp "$SKYWARD_BINARY" target/mi8937_4_19/skyward
    patchelf --remove-rpath target/mi8937_4_19/skyward
    SKYWARD_BINARY="$port_root/target/mi8937_4_19/skyward"
fi
python3 tools/subset-cjk.py --out target/mi8937_4_19/MiSansVF.ttf
"$SKYWARD_PACKER" recovery \
    --stock "$base_image" \
    --binary "$SKYWARD_BINARY" \
    --config devices/mi8937_4_19.toml \
    --wrapper devices/mi8937_4_19/recovery.sh \
    --font target/mi8937_4_19/MiSansVF.ttf \
    --mono resources/fonts/KirkMono-Regular.ttf \
    --mono-license resources/fonts/KirkMono-OFL.txt \
    --drop twres \
    --partition-size 67108864 \
    --ramdisk-limit 20971520 \
    --out "$output_image"
python3 tools/verify-mi8937-4.19.py "$base_image" "$output_image"
sha256sum "$output_image"
