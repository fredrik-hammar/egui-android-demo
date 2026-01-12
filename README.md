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


## `wgpu` dependency debugging

`eframe` does not enable necessary `wgpu` backends on Android by default
so finicky overrides are done in `Cargo.toml` that only works if the dependency
resolver deduplicates `wpgu` to a single dependency.

This can be checked with `cargo tree` like this:
```
$ cargo tree --target=aarch64-linux-android --edges features --invert wgpu
wgpu v27.0.1
├── wgpu feature "gles"
│   └── egui-android-demo v0.1.0 (/home/frehamma/Code/egui-android-demo)
│       └── egui-android-demo feature "default" (command-line)
├── wgpu feature "std"
│   ├── eframe v0.33.3
│   │   ├── eframe feature "android-native-activity"
│   │   │   └── egui-android-demo v0.1.0 (/home/frehamma/Code/egui-android-demo) (*)
│   │   ├── eframe feature "default_fonts"
│   │   │   └── egui-android-demo v0.1.0 (/home/frehamma/Code/egui-android-demo) (*)
│   │   └── eframe feature "wgpu"
│   │       └── egui-android-demo v0.1.0 (/home/frehamma/Code/egui-android-demo) (*)
│   └── egui-wgpu v0.33.3
│       └── egui-wgpu feature "winit"
│           └── eframe v0.33.3 (*)
│           └── egui-wgpu feature "winit" (*)
└── wgpu feature "wgsl"
    └── egui-wgpu v0.33.3 (*)
```

Failure case will look like this:
```
$ cargo tree --target=aarch64-linux-android --edges features --invert wgpu
error: There are multiple `wgpu` packages in your project, and the specification `wgpu` is ambiguous.
Please re-run this command with one of the following specifications:
  wgpu@27.0.1
  wgpu@28.0.0
```
