FROM rust:1.93

LABEL org.opencontainers.image.source \
      https://github.com/fredrik-hammar/egui-android-demo

# Tools needed by xbuild
RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,target=/var/lib/apt,sharing=locked \
    apt-get update && apt-get --no-install-recommends install -y \
        clang \
        lld \
        llvm

RUN --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo install xbuild --version 0.2.0

RUN rustup target add aarch64-linux-android

WORKDIR /egui-android-demo
COPY . .

# Diagnostics for available tools.
RUN x doctor

RUN --mount=type=cache,target=/egui-android-demo/target \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/root/.cache/x \
    <<-EOF
    set -ex
    x build --arch arm64 --platform android

    echo "wgpu dependency deduplicated check"
    checks/wgpu-dedup.sh --verbose
EOF
