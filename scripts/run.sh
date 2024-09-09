#!/bin/bash
set -euC

qemu-system-x86_64 -enable-kvm \
    -drive if=pflash,format=raw,readonly=on,file=./assets/ovmf/OVMF_CODE.fd \
    -drive if=pflash,format=raw,readonly=on,file=./assets/ovmf/OVMF_VARS.fd \
    -drive format=raw,file=fat:rw:esp
