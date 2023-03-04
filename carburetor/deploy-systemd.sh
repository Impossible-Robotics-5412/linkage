#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly PROJECT_NAME=carburetor
readonly TARGET_ARCH=armv7-unknown-linux-gnueabihf
readonly SOURCE_PATH=target/${TARGET_ARCH}/release/${PROJECT_NAME}
readonly SOURCE_SERVICE=${PROJECT_NAME}.service

# If necessary, change the following values.
readonly TARGET_HOST=pi@raspberrypi
readonly TARGET_PATH=/home/pi/${PROJECT_NAME}
readonly TARGET_BIN=${TARGET_PATH}/${PROJECT_NAME}
readonly TARGET_SERVICE=${TARGET_PATH}/${SOURCE_SERVICE}

cross build --release --target=${TARGET_ARCH}
rsync -a ${SOURCE_PATH} ${SOURCE_SERVICE} ${TARGET_HOST}:${TARGET_PATH}/
ssh -t ${TARGET_HOST} \
    "sudo install ${TARGET_BIN} /usr/bin/${PROJECT_NAME} && \
    sudo install -Dm644 ${TARGET_SERVICE} /usr/lib/systemd/${SOURCE_SERVICE} && \
    sudo systemctl daemon-reload && \
    sudo systemctl restart ${SOURCE_SERVICE}"
