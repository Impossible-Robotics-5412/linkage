#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly CARGO_PATH=$HOME/.cargo/bin/cargo
readonly CARBURETOR_BUILD_PATH=./target/release/carburetor
readonly CONFIG_SOURCE_FILE=./examples/config/config.toml
readonly CONFIG_TARGET_FOLDER=$HOME/.config/linkage/
readonly LINKAGE_SYSTEMD_SOCKET=./lib/linkage.socket
readonly LINKAGE_SYSTEMD_SOCKET_SERVICE=./lib/linkage@.service
readonly CARBURETOR_SERVICE=./carburetor/carburetor.service

# Install NodeJS
curl -sL https://deb.nodesource.com/setup_lts.x | sudo bash -
sudo apt install nodejs

# Install Rust
# Yes, this is ugly but not sure how to set the -y flag in the script when running `curl https://sh.rustup.rs -sSf | sh` any other way.
curl https://sh.rustup.rs -sSf -o install-rust.sh
chmod +x ./install-rust.sh
./install-rust.sh -y
rm ./install-rust.sh

# Install Carburetor
${CARGO_PATH} build -p carburetor --release
sudo install ${CARBURETOR_BUILD_PATH} /usr/bin/carburetor


# Copy config file
mkdir -p ${CONFIG_TARGET_FOLDER}
cp ${CONFIG_SOURCE_FILE} ${CONFIG_TARGET_FOLDER}

# Setup services
sudo cp ${LINKAGE_SYSTEMD_SOCKET} /etc/systemd/system/
sudo cp ${LINKAGE_SYSTEMD_SOCKET_SERVICE} /etc/systemd/system/
sudo cp ${CARBURETOR_SERVICE} /etc/systemd/system/

sudo systemctl daemon-reload
sudo systemctl enable linkage.socket
sudo systemctl restart linkage.socket
sudo systemctl enable carburetor.service
sudo systemctl restart carburetor.service