#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly TARGET_HOST=pi@haxophone.local
readonly TARGET_PATH=/home/pi/
readonly TARGET_ARCH=arm-unknown-linux-gnueabihf
readonly SOURCE_PATH=./target/${TARGET_ARCH}/release/pi_serial_test
scp ${SOURCE_PATH} ${TARGET_HOST}:${TARGET_PATH}

