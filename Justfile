@setup:
    @echo 'Adding Rust target'
    rustup target add x86_64-unknown-linux-musl
    @echo 'Setting up linux submodule & config'
    git submodule update --init --recursive --depth 1
    rm linux/linux/.config
    ln -s "$(pwd)/linux/.config" "$(pwd)/linux/linux/.config"
    @echo 'Creating build directory'
    mkdir -p .pep-os

@setup-arch:
    @echo 'Setting up Arch Linux'
    yay -Sy --needed qemu-full qemu-tools base-devel cdrkit syslinux xmlto kmod inetutils bc musl

@build-kernel:
    cd linux/linux && make --jobs 4

@build-crates:
    cargo build --release

[working-directory: '.pep-os']
@build-initramfs: build-crates
    @echo 'Creating directories'
    rm -rf initramfs
    mkdir -p initramfs initramfs/bin
    @echo 'Installing coreutils'
    cd initramfs/bin && \
        curl -Lo coreutils.tar.gz https://github.com/uutils/coreutils/releases/download/0.1.0/coreutils-0.1.0-x86_64-unknown-linux-musl.tar.gz && \
        tar -xzf coreutils.tar.gz --strip-components=1 coreutils-0.1.0-x86_64-unknown-linux-musl/coreutils && \
        rm coreutils.tar.gz
    cd initramfs/bin && for command in $(./coreutils --list); do ln -s /bin/coreutils $command; done
    @echo 'Installing initramfs program'
    cp rs/x86_64-unknown-linux-musl/release/initramfs initramfs/init
    echo 'Building initramfs.cpio'
    cd initramfs && find . | cpio -o -H newc > ../initramfs.cpio

@build: build-kernel build-crates build-initramfs

[working-directory: 'linux/linux']
@run: build
    make isoimage FDARGS="initrd=/initramfs.cpio" FDINITRD="../../.pep-os/initramfs.cpio"
    qemu-system-x86_64 -cdrom arch/x86/boot/image.iso
