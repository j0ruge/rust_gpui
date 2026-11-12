#!/usr/bin/env bash
set -euo pipefail

# Preparação do Ubuntu 24.04 para compilar este aplicativo.
if (( EUID != 0 )); then
    echo 'Execute este script com sudo no seu terminal.' >&2
    exit 1
fi

packages=(
    build-essential pkg-config clang cmake
    libfontconfig-dev libfreetype-dev libwayland-dev libx11-xcb-dev
    libxkbcommon-dev libxkbcommon-x11-dev libxcb1-dev libssl-dev libzstd-dev
)

apt-get update
apt-get install --no-install-recommends "${packages[@]}"
