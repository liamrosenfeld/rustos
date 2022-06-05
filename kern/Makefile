ROOT := $(shell git rev-parse --show-toplevel)

KERN := kernel
TARGET := target/aarch64-unknown-none/release/${KERN}
OBJCPY := cargo objcopy --release -- --strip-all -O binary
TTY_PATH := /dev/ttyUSB0

.PHONY: all build qemu transmit objdump nm check clean install test

all: build

build:
	@echo "+ Building build/$(KERN).elf [build/$@]"
	@cargo build --release
	@mkdir -p build
	@cp -f $(TARGET) build/$(KERN).elf

	@echo "+ Building build/$(KERN).bin [objcopy]"
	@$(OBJCPY) build/$(KERN).bin

check:
	@cargo check

qemu: build
	./qemu.sh build/$(KERN).bin

transmit: build
	@echo "+ Transmitting build/$(KERN).bin to $(TTY_PATH)"
	ttywrite -i build/$(KERN).bin $(TTY_PATH)

objdump:
	cargo objdump --release -- --disassemble --no-show-raw-insn

nm: build
	cargo nm build/$(KERN).elf

clean:
	cargo clean
	rm -rf build

install: build
	cp ./build/kernel.bin /media/liam/boot/kernel8.img

test:
	cargo test --target=$(shell $(ROOT)/bin/get-host-target.sh)
