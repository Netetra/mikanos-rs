#!/bin/bash
set -euC

mkdir -p ./assets/ovmf
cp /usr/share/OVMF/x64/OVMF_CODE.4m.fd ./assets/ovmf/OVMF_CODE.fd
cp /usr/share/OVMF/x64/OVMF_VARS.4m.fd ./assets/ovmf/OVMF_VARS.fd
