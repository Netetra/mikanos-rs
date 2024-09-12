#!/bin/bash
set -euC

function build() {
  cd $1 && cargo build
  cd ..
}

build bootloader
build kernel

mkdir -p ./esp/efi/boot

cp ./bootloader/target/x86_64-unknown-uefi/debug/bootloader.efi esp/efi/boot/bootx64.efi
cp ./kernel/target/kernel/kernel.elf esp/kernel.elf
