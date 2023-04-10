#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly PROJECT_NAME=gauge
readonly TARGET_ARCH=armv7-unknown-linux-gnueabihf
readonly SOURCE_PATH=../target/${TARGET_ARCH}/release/${PROJECT_NAME}

# If necessary, change the following values.
readonly TARGET_HOST=pi@raspberrypi
readonly TARGET_BIN=/home/pi/${PROJECT_NAME}

cross build --release --target=${TARGET_ARCH}
rsync -aP ${SOURCE_PATH} ${TARGET_HOST}:${TARGET_BIN}
ssh -t ${TARGET_HOST} \
    "sudo install ${TARGET_BIN} /usr/bin/${PROJECT_NAME} && \
     sudo systemctl daemon-reload && \
     sudo systemctl restart ${PROJECT_NAME}.service && \
     rm -rf ${TARGET_BIN}"
