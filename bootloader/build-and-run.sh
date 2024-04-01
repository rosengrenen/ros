#!/bin/sh

set -e

UEFI_PROFILE=debug
KERNEL_PROFILE=debug

case $UEFI_PROFILE in
    debug)
        cargo build
        ;;
    release)
        cargo build --release
        ;;
    *)
        echo "Invalid uefi profile: $UEFI_PROFILE"
        exit 1
        ;;
esac
cp target/x86_64-unknown-uefi/$UEFI_PROFILE/bootloader.efi target/x86_64-unknown-uefi/$UEFI_PROFILE/BOOTX64.EFI

dd if=/dev/zero of=fat.img bs=1M count=64
mformat -i fat.img ::
mmd -i fat.img ::/EFI
mmd -i fat.img ::/EFI/BOOT
mcopy -i fat.img target/x86_64-unknown-uefi/$UEFI_PROFILE/BOOTX64.EFI ::/EFI/BOOT
case $KERNEL_PROFILE in
    debug)
        cd ../kernel/ros && cargo build && cd -
        ;;
    release)
        cd ../kernel/ros && cargo build --release && cd -
        ;;
    *)
        echo "Invalid kernel profile: $KERNEL_PROFILE"
        exit 1
        ;;
esac
mcopy -i fat.img ../kernel/ros/target/x86_64/$KERNEL_PROFILE/ros ::/

mkgpt -o hdimage.bin --part fat.img --type system
# sudo qemu-system-x86_64 -m 1G -L /usr/share/ovmf/x64 -pflash /usr/share/ovmf/x64/OVMF.fd -hda hdimage.bin -serial stdio -no-reboot
sudo qemu-system-x86_64 -m 1G -L /usr/share/ovmf/x64 -pflash /usr/share/ovmf/x64/OVMF.fd -hda hdimage.bin -nographic -no-reboot -smp 2 $@
