setup:
    rustup target add x86_64-unknown-linux-musl
    git submodule update --init --recursive --depth 1
    rm linux/linux/.config
    ln -s "$(pwd)/linux/.config" "$(pwd)/linux/linux/.config"

build:
    cargo build --release
