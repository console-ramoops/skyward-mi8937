![Skyward Recovery](assets/hero.jpg)

# 适用于 Mi8937 的 Skyward

[English](README.md) | 简体中文

Skyward 是使用 Rust 编写的 recovery。本移植使用 Mi-Thorium TWRP 的统一
4.19 内核和 recovery 基础镜像。

**版本：** 0.2.0 (Mi8937_4_19) · **更新日期：** 2026-11-12
**维护者：** alpha269 · **内核：** 4.19.312-Mi8937v2

[发布页面](https://github.com/console-ramoops/skyward-mi8937/releases/tag/v0.2.0-mi8937-4_19) ·
[移植指南（英文）](docs/mi8937_4_19.md)

## 支持的设备

| 设备 | 代号 |
| --- | --- |
| Redmi 3S / 3X / 3S Prime | `land` |
| Redmi 4 / 4X | `santoni` |
| Redmi 4 Standard | `prada` |
| Redmi 4A | `rolex` |
| Redmi 5A | `riva` |
| Redmi Note 5A | `ugglite` |
| Redmi Note 5A Prime | `ugg` |

内核会在启动时识别设备和屏幕尺寸。

## 下载与安装

从发布页面的 Assets 下载
[Skyward-0.2.0-Mi8937_4_19.img](https://github.com/console-ramoops/skyward-mi8937/releases/download/v0.2.0-mi8937-4_19/Skyward-0.2.0-Mi8937_4_19.img)。
需要已解锁的 bootloader。

SHA-256：

```text
d3e64c39bf93b8b5df9c544faa69621c2aa1f07691e0d3c1308e1c221924414f
```

进入 fastboot 后连接电脑：

```sh
fastboot flash recovery Skyward-0.2.0-Mi8937_4_19.img
fastboot boot Skyward-0.2.0-Mi8937_4_19.img
```

也可以在已有的 TWRP 中选择 Install Image，选中 IMG 和 Recovery 分区，
然后重启到 recovery。

## 功能与此次修复

- 文件管理、终端、备份、镜像刷写和基于脚本的 ROM ZIP 安装。
- ADB 与 MTP；已挂载的 microSD 作为独立存储显示，内部存储锁定时仍可使用。
- microSD 挂载到 `/sdcard1`，安装页面可直接打开它；recovery 进程重启后复用已有挂载。
- 267 DPI，内容超出屏幕时可以滚动；设备信息更新导致页面高度改变时重新绘制页面，避免页面尺寸不一致导致的绘制崩溃。
- 默认英语并保留已保存的语言选择，支持 Back、Home、Menu 按键及振动设置。
- 普通 recovery 启动时在后台打开 data；受锁屏密钥保护的文件仍需要对应凭据。

Wi-Fi 尚未配置。这些单槽设备不提供 A/B 槽位切换、A/B OTA payload 安装或
基于 init_boot 的双系统功能。逻辑分区取决于已安装 ROM 的动态分区布局。

## 文件传输

开启 **Settings → File transfer**，再重新连接 USB。内部存储解锁后与已挂载的
microSD 作为独立存储显示；内部存储锁定时仍可使用 microSD。

## 构建与日志

使用发布页面附带的
[对应源码快照](https://github.com/console-ramoops/skyward-mi8937/releases/download/v0.2.0-mi8937-4_19/Skyward-0.2.0-mi8937-4_19-20261112-source.tar.gz)
构建当前镜像。依赖和步骤见[移植指南](docs/mi8937_4_19.md)。
Termux 可设置 `CARGO_BUILD_JOBS=1` 降低内存使用；较大的构建主机可提高该值。

日志位于 `/skyward/run/recovery.log` 和 `/tmp/recovery.log`；同时收集
`adb shell dmesg`。Rust panic 文本也会写入内核日志。

## 致谢与许可证

感谢 Skyward 上游及贡献者、提供旧设备支持的 juezhi6668，以及提供 recovery
基础镜像与统一设备支持的 Mi-Thorium / TWRP。

[GPL-3.0-or-later](LICENSE)。导入的 recovery 文件保留各自的原始许可证。
