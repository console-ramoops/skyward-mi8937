#!/system/bin/sh
# init runs this in place of TWRP. The base already selects its device and
# FBE-v2 table before starting the recovery service.
export PATH=/system/bin:/system/xbin:/sbin
export LD_LIBRARY_PATH=/system/lib64:/system/lib:/vendor/lib64:/vendor/lib
mkdir -p /tmp
export TMPDIR=/tmp
exec >> /tmp/recovery.log 2>&1
echo "Starting Skyward unified mi8937_4_19"

codename=$(cat /sys/xiaomi-msm8937-mach/codename 2>/dev/null)
model=$(cat /sys/xiaomi-msm8937-mach/product_name 2>/dev/null)
# Only put plain device names in TOML; the base's identity properties stay
# available even if an unexpected kernel gives a different spelling.
case "$codename" in
    ''|*[!a-zA-Z0-9_-]*) ;;
    *) sed -i "s/^codename = .*/codename = \"$codename\"/" /skyward/device.toml ;;
esac
case "$model" in
    ''|*[!a-zA-Z0-9\ ._/\(\)+-]*) ;;
    *) sed -i "s|^model = .*|model = \"$model\"|" /skyward/device.toml ;;
esac

# On a non-dynamic ROM the base only supplies TWRP flags for these two
# partitions. Skyward reads Android's five-column fstab instead.
if ! /system/bin/grep -q ',logical' /system/etc/recovery.fstab; then
    printf '%s\n' \
        '/dev/block/bootdevice/by-name/system /system_root ext4 ro,discard wait,check,formattable' \
        '/dev/block/bootdevice/by-name/system /system_root erofs ro wait' \
        '/dev/block/bootdevice/by-name/cust /vendor ext4 ro,discard wait,check,formattable' \
        >> /system/etc/recovery.fstab
fi

exec /system/bin/skyward recovery
