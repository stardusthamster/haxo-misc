#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

export PATH="/home/user/Programme/raspi-zero-toolchain/x-tools/0.1.2/armv6-rpi-linux-gnueabihf/bin/:$PATH"

cargo clean
cargo build --release --target=arm-unknown-linux-gnueabihf  --config target.arm-unknown-linux-gnueabihf.linker=\"armv6-rpi-linux-gnueabihf-gcc\"

