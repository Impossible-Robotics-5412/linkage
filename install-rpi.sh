#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly USER=linkage
readonly DEFAULT_PASSWORD=linkage
readonly LINKAGE_HOME=/home/${USER}
readonly LINKAGE_PATH=${LINKAGE_HOME}/linkage

readonly LINKAGE_SYSTEMD_SOCKET=${LINKAGE_PATH}/lib/linkage.socket
readonly LINKAGE_SYSTEMD_SOCKET_SERVICE=${LINKAGE_PATH}/lib/linkage@.service
readonly CARBURETOR_SERVICE=${LINKAGE_PATH}/carburetor/carburetor.service
readonly GAUGE_SERVICE=${LINKAGE_PATH}/gauge/gauge.service

readonly CARBURETOR_PATH=${LINKAGE_HOME}/carburetor
readonly GAUGE_PATH=${LINKAGE_HOME}/gauge

sudo useradd --create-home ${USER}
echo ${DEFAULT_PASSWORD}:${USER} | sudo chpasswd
sudo usermod -aG sudo ${USER}

# Install git
sudo apt update
sudo apt install git -y

# Make sure we can use PWM channels.
echo "dtoverlay=pwm-2chan" | sudo tee -a /boot/config.txt >/dev/null

# Clone the repo
sudo git clone https://github.com/Impossible-Robotics-5412/linkage.git ${LINKAGE_PATH}

sudo curl -oL ${CARBURETOR_PATH} https://github.com/Impossible-Robotics-5412/linkage/releases/latest/download/carburetor
sudo chmod +x ${CARBURETOR_PATH}
sudo curl -oL ${GAUGE_PATH} https://github.com/Impossible-Robotics-5412/linkage/releases/latest/download/gauge
sudo chmod +x ${GAUGE_PATH}

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