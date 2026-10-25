hot:
    dx serve

run:
    dx run

build:
    dx build

build-win-release:
    cargo build --release --no-default-features --target x86_64-pc-windows-msvc

build-mac-arm-release:
    cargo packager --release --no-default-features --target aarch64-apple-darwin --format app

build-mac-x86-release:
    cargo packager --release --no-default-features --target x86_64-apple-darwin --format app

build-linux-release:
    cargo build --release --no-default-features --target x86_64-unknown-linux-gnu
