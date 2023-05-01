#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly EXAMPLE_NAME=$2
readonly TARGET_ARCH=armv7-unknown-linux-gnueabihf
readonly SOURCE_PATH=../../target/${TARGET_ARCH}/release/examples/${EXAMPLE_NAME}
readonly TARGET_HOST="linkage@$1"
readonly TARGET_FOLDER=/home/linkage/robot_code/
readonly TARGET_BIN=${TARGET_FOLDER}main

cross build --release --example "${EXAMPLE_NAME}" --target=${TARGET_ARCH}
rsync -aP --rsync-path="mkdir -p ${TARGET_FOLDER} && rsync" "${SOURCE_PATH}" "${TARGET_HOST}:${TARGET_BIN}"