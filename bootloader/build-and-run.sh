#!/bin/sh

set -e

cargo build
cp target/x86_64-unknown-uefi/debug/bootloader.efi target/x86_64-unknown-uefi/debug/BOOTX64.EFI
# cp target/x86_64-unknown-uefi/release/bootloader.efi target/x86_64-unknown-uefi/release/BOOTX64.EFI

dd if=/dev/zero of=fat.img bs=1k count=2880
mformat -i fat.img -f 2880 ::
mmd -i fat.img ::/EFI
mmd -i fat.img ::/EFI/BOOT
mcopy -i fat.img target/x86_64-unknown-uefi/debug/BOOTX64.EFI ::/EFI/BOOT
cd ../kernel && cargo build --release && cd -
mcopy -i fat.img ../kernel/target/x86_64/release/ros ::/

mkgpt -o hdimage.bin --image-size 8192 --part fat.img --type system
# sudo qemu-system-x86_64 -m 1G -L /usr/share/ovmf/x64 -pflash /usr/share/ovmf/x64/OVMF.fd -hda hdimage.bin -serial stdio -no-reboot
sudo qemu-system-x86_64 -m 1G -L /usr/share/ovmf/x64 -pflash /usr/share/ovmf/x64/OVMF.fd -hda hdimage.bin -nographic -no-reboot -smp 2
