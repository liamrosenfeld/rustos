ROOT := $(shell git rev-parse --show-toplevel)

KERN := kernel
TARGET := target/aarch64-unknown-none/release/${KERN}
TTY_PATH := /dev/tty.usbserial-0001

.PHONY: all bin elf qemu transmit objdump nm check clean install test

all: bin

bin:
	@mkdir -p build
	cargo objcopy -- --strip-all -O binary build/$(KERN).bin

elf:
	@cargo build --release
	@mkdir -p build
	@cp -f $(TARGET) build/$(KERN).elf

check:
	@cargo check

qemu: bin
	./qemu.sh build/$(KERN).bin

transmit: bin
	@echo "+ Transmitting build/$(KERN).bin to $(TTY_PATH)"
	ttywrite -i build/$(KERN).bin $(TTY_PATH)

objdump:
	cargo objdump --release -- --disassemble --no-show-raw-insn

nm: elf
	cargo nm build/$(KERN).elf

clean:
	cargo clean
	rm -rf build

install: bin
	cp ./build/$(KERN).bin /Volumes/boot/kernel8.img
	diskutil eject disk2	

test:
	cargo test --target=$(shell $(ROOT)/bin/get-host-target.sh)
