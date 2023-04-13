#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly EXAMPLE_NAME=simple_tankdrive
readonly TARGET_ARCH=armv7-unknown-linux-gnueabihf
readonly SOURCE_PATH=../../target/${TARGET_ARCH}/release/examples/${EXAMPLE_NAME}

# If necessary, change the following values.
readonly TARGET_HOST=pi@raspberrypi
readonly TARGET_BIN=/home/pi/robot_code/main

cross build --release --example ${EXAMPLE_NAME} --target=${TARGET_ARCH}
rsync -aP ${SOURCE_PATH} ${TARGET_HOST}:${TARGET_BIN}