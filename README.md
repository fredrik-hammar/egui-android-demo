# egui Android Demo

This is my attempt at getting latest version of the
[`egui`](https://github.com/emilk/egui) demo to work on Android.

Optimized for size the resulting `egui-android-demo.aab` is 2.4M.

Built using [`xbuild`](https://github.com/rust-mobile/xbuild)

## Dependencies

`xbuild` requires additional tools that cannot be automatically installed
with `cargo install xbuild`. Consider installing with a system package
manager instead.  For instance, `pacman -Si xbuild` for Arch Linux.

You can use `x doctor` to check which tools are detected by `xbuild`.
Not all are required to build this project, but at least `lld` and
`llvm-readobj` is needed. 

Please check on how to install these tools with your preferred
distribution's documentation.


## Building

    x build --arch arm64 --platform android

## Running

    x run --device <DEVICE>

### Example

    $ x devices
    host                                              Linux               linux x64           Arch Linux 6.6.2-arch1-1
    adb:d535946                                       OnePlus5T           android arm64       Android 10 (API 29)
    $ x run --device adb:d535946 
