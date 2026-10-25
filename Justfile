hot:
    dx serve

run:
    dx run

build:
    dx build

build-linux-release:
    cargo build --release --no-default-features --target x86_64-unknown-linux-gnu

build-win-release:
    cargo build --release --no-default-features --target x86_64-pc-windows-msvc

build-mac-arm-release:
    cargo packager --release --target aarch64-apple-darwin --formats app
