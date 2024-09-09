#!/bin/bash
set -euC

cargo build --target x86_64-unknown-uefi

mkdir -p ./esp/efi/boot
cp ./target/x86_64-unknown-uefi/debug/bootloader.efi esp/efi/boot/bootx64.efi
