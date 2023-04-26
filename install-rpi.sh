#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly USER=linkage
readonly DEFAULT_PASSWORD=linkage
readonly HOME=/home/${USER}
readonly LINKAGE_PATH=${HOME}/linkage

readonly CARGO_PATH=$HOME/.cargo/bin/cargo

readonly CARBURETOR_BUILD_PATH=${LINKAGE_PATH}/target/release/carburetor
readonly GAUGE_BUILD_PATH=${LINKAGE_PATH}/target/release/gauge

readonly LINKAGE_SYSTEMD_SOCKET=${LINKAGE_PATH}/lib/linkage.socket
readonly LINKAGE_SYSTEMD_SOCKET_SERVICE=${LINKAGE_PATH}/lib/linkage@.service
readonly CARBURETOR_SERVICE=${LINKAGE_PATH}/carburetor/carburetor.service
readonly GAUGE_SERVICE=${LINKAGE_PATH}/gauge/gauge.service

sudo useradd --create-home ${USER}
echo ${DEFAULT_PASSWORD}:${USER} | sudo chpasswd
sudo usermod -aG sudo ${USER}

# Install git
sudo apt update
sudo apt install git -y

# Clone the repo
sudo git clone https://github.com/Impossible-Robotics-5412/linkage.git ${LINKAGE_PATH}
cd ${LINKAGE_PATH}

# Install Rust
# Yes, this is ugly but not sure how to set the -y flag in the script 
# when running `curl https://sh.rustup.rs -sSf | sh` any other way.
sudo curl https://sh.rustup.rs -sSf -o install-rust.sh
sudo chmod +x ./install-rust.sh
sudo ./install-rust.sh -y
sudo rm ./install-rust.sh

# Install Carburetor
${CARGO_PATH} build -p carburetor --release
sudo install ${CARBURETOR_BUILD_PATH} /usr/bin/carburetor

# Make sure we can use PWM channels.
echo "dtoverlay=pwm-2chan" | sudo tee -a /boot/config.txt >/dev/null

# Install Gauge
${CARGO_PATH} build -p gauge --release
sudo install ${GAUGE_BUILD_PATH} /usr/bin/gauge

# Setup services
sudo cp ${LINKAGE_SYSTEMD_SOCKET} /etc/systemd/system/
sudo cp ${LINKAGE_SYSTEMD_SOCKET_SERVICE} /etc/systemd/system/
sudo cp ${CARBURETOR_SERVICE} /etc/systemd/system/
sudo cp ${GAUGE_SERVICE} /etc/systemd/system/

sudo systemctl daemon-reload
sudo systemctl enable linkage.socket
sudo systemctl restart linkage.socket
sudo systemctl enable carburetor.service
sudo systemctl restart carburetor.service
sudo systemctl enable gauge.service
sudo systemctl restart gauge.service

echo "Please restart the Raspberry Pi!"