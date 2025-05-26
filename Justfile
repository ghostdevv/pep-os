@setup:
    @echo 'Adding Rust target'
    rustup target add x86_64-unknown-linux-musl
    @echo 'Setting up linux submodule & config'
    git submodule update --init --recursive --depth 1
    rm linux/linux/.config
    ln -s "$(pwd)/linux/.config" "$(pwd)/linux/linux/.config"

@build-kernel:
    cd linux/linux && make --jobs 4

@build-crates:
    cargo build --release

@build-initrd: build-crates
    mkdir -p linux/initrd-tmp
    cp target/x86_64-unknown-linux-musl/release/initrd linux/initrd-tmp/init
    cd linux/initrd-tmp && find . | cpio -o -H newc > ../initrd.cpio
    rm -rf linux/initrd-tmp

@build: build-kernel build-crates build-initrd

[working-directory: 'linux/linux']
@run: build
    make isoimage FDARGS="initrd=/initrd.cpio" FDINITRD="../initrd.cpio"
    qemu-system-x86_64 -cdrom arch/x86/boot/image.iso
