#!/bin/bash

set -e

TOP=$(git rev-parse --show-toplevel)
BIN=$TOP/bin
DEP=$TOP/.dep
PROJ_PKG=(build-essential
     python3
     socat
     wget
     curl
     tar
     screen
     clang
     lld
     linux-image-extra-virtual
     qemu-system-arm)

# install pkgs
if [[ $($BIN/get-dist) == "ubuntu" ]]; then
    echo "[!] Installing packages"

    sudo apt update
    sudo apt install -y ${PROJ_PKG[*]}
fi

# install rustup
if ! [ -x "$(command -v rustup)" ]; then
    echo "[!] Installing rustup"

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

    export PATH=$HOME/.cargo/bin:$PATH
fi

# install rust nightly
echo "[!] Installing rust nightly"
rustup toolchain install nightly
rustup default nightly
rustup component add rust-src clippy

# install cargo binutils
mkdir -p $DEP
pushd $DEP
if ! [ -e cargo-objcopy ]; then
    echo "[!] Installing cargo binutils"
    cargo install cargo-binutils
    rustup component add llvm-tools-preview
fi
popd
