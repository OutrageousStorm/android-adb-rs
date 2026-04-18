# ⚡ android-adb-rs

Fast, lightweight ADB CLI tool written in Rust.

## Features

- ⚡ Single binary, no dependencies
- 📦 Package management (list, install, uninstall)
- ⚙️ System settings (get/set)
- 📱 Device info
- 🔌 Multiple device support

## Install

```bash
cargo install --git https://github.com/OutrageousStorm/android-adb-rs
```

## Usage

```bash
# Device info
adb-rs info

# List packages
adb-rs packages list
adb-rs packages list --filter "com.example"

# Settings
adb-rs settings get secure location_mode
adb-rs settings put global limit_ad_tracking 1

# Multiple devices
adb-rs -s device_serial info
```
