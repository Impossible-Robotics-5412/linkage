#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly BUILD_FOLDER=build
readonly TARGET_PATH=/home/pi/linkage
readonly TARGET_HOST=pi@raspberrypi

npm run build
rsync -aP ${BUILD_FOLDER} ${TARGET_HOST}:${TARGET_PATH}/
ssh -t ${TARGET_HOST} \
    "cd ${TARGET_PATH} && \
    npm install @impossiblerobotics/linkage@latest"