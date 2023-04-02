#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly BUILD_FOLDER=build
readonly LINKAGE_LIB=../../../lib/linkage-node/
readonly LINKAGE_LIB_TARGET_PATH=/home/pi/linkage-node

readonly EXAMPLE_TARGET_PATH=/home/pi/robot_code

readonly TARGET_HOST=pi@raspberrypi

npm run build
# FIXME: build linkage-node
rsync -aP ${BUILD_FOLDER} ${TARGET_HOST}:${EXAMPLE_TARGET_PATH}/
rsync -aP --include 'dist' --exclude 'node_modules' ${LINKAGE_LIB} ${TARGET_HOST}:${LINKAGE_LIB_TARGET_PATH}/
ssh -t ${TARGET_HOST} \
    "cd ${LINKAGE_LIB_TARGET_PATH} && \
    npm install && \
    cd ${EXAMPLE_TARGET_PATH} && \
    npm install ${LINKAGE_LIB_TARGET_PATH}"