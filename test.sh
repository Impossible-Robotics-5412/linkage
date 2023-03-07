#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly PWD=$(pwd)
readonly CFE=$PWD/cockpit/frontend/web

cargo build

alacritty --hold --working-directory=$PWD --command cargo run --bin carburetor      &
alacritty --hold --working-directory=$PWD --command cargo run --bin runtime         &
alacritty --hold --working-directory=$PWD --command cargo run --bin cockpit-backend &
alacritty --hold --working-directory=$CFE --command yarn dev                        &

trap 'kill $(jobs -p)' SIGINT SIGTERM
wait $(jobs -p)
