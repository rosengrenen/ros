#!/bin/sh

cargo build --target x86_64-unknown-uefi
cp target/x86_64-unknown-uefi/debug/bootloader.efi target/x86_64-unknown-uefi/debug/BOOTX64.EFI

dd if=/dev/zero of=fat.img bs=1k count=1440
mformat -i fat.img -f 1440 ::
mmd -i fat.img ::/EFI
mmd -i fat.img ::/EFI/BOOT
mcopy -i fat.img target/x86_64-unknown-uefi/debug/BOOTX64.EFI ::/EFI/BOOT

mkgpt -o hdimage.bin --image-size 4096 --part fat.img --type system
sudo qemu-system-x86_64 -L /usr/share/ovmf/x64 -pflash /usr/share/ovmf/x64/OVMF.fd -hda hdimage.bin
