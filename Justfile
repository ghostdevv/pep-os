@setup:
    @echo 'Adding Rust target'
    rustup target add x86_64-unknown-linux-musl
    @echo 'Setting up linux submodule & config'
    git submodule update --init --recursive --depth 1
    rm linux/linux/.config
    ln -s "$(pwd)/linux/.config" "$(pwd)/linux/linux/.config"

@setup-arch:
    @echo 'Setting up Arch Linux'
    yay -Sy --needed qemu-full qemu-tools base-devel cdrkit syslinux xmlto kmod inetutils bc musl

@build-kernel:
    cd linux/linux && make --jobs 4

@build-crates:
    cargo build --release

@build-initramfs: build-crates
    mkdir -p linux/initramfs-tmp
    mkdir -p linux/initramfs-tmp/bin
    cp target/x86_64-unknown-linux-musl/release/initramfs linux/initramfs-tmp/init
    cd linux/initramfs-tmp/bin && \
        curl -Lo coreutils.tar.gz https://github.com/uutils/coreutils/releases/download/0.1.0/coreutils-0.1.0-x86_64-unknown-linux-musl.tar.gz && \
        tar -xzf coreutils.tar.gz --strip-components=1 coreutils-0.1.0-x86_64-unknown-linux-musl/coreutils && \
        rm coreutils.tar.gz
    cd linux/initramfs-tmp/bin && for command in $(./coreutils --list); do ln -s /bin/coreutils $command; done
    cd linux/initramfs-tmp && find . | cpio -o -H newc > ../initramfs.cpio
    rm -rf linux/initramfs-tmp

@build: build-kernel build-crates build-initramfs

[working-directory: 'linux/linux']
@run: build
    make isoimage FDARGS="initrd=/initramfs.cpio" FDINITRD="../initramfs.cpio"
    qemu-system-x86_64 -cdrom arch/x86/boot/image.iso
