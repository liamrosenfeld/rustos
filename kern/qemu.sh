#!/bin/sh

qemu-system-aarch64 \
    -nographic \
    -M raspi3b \
    -serial null -serial mon:stdio \
    -kernel \
    "$@"
