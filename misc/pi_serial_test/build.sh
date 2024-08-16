#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

export PATH="/home/user/Programme/raspi-zero-toolchain/x-tools/0.1.2/armv6-rpi-linux-gnueabihf/bin/:$PATH"

readonly TARGET_ARCH=arm-unknown-linux-gnueabihf
cargo clean
cargo build --release --target=${TARGET_ARCH}

