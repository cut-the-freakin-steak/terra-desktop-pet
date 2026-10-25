hot:
    dx serve --features "bevy_debug"

run:
    dx run --features "bevy_debug"

build:
    dx build --features "bevy_debug"

build-linux-release:
    cargo build --release --target x86_64-unknown-linux-gnu

build-win-release:
    cargo build --release --target x86_64-pc-windows-msvc

build-mac-arm-release:
    cargo packager --release --target aarch64-apple-darwin --formats app
