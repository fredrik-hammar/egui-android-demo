# egui Android Demo

This is my attempt at getting latest version of the
[`egui`](https://github.com/emilk/egui) demo to work on Android.

Built using [`xbuild`](https://github.com/rust-mobile/xbuild)

## Building

    x build --arch arm64 --platform android

## Running

    x run --device <DEVICE>

### Example

    $ x devices
    host                                              Linux               linux x64           Arch Linux 6.6.2-arch1-1
    adb:d535946                                       OnePlus5T           android arm64       Android 10 (API 29)
    $ x run --device adb:d535946 
