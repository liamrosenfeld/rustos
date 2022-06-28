ROOT := $(shell git rev-parse --show-toplevel)

KERN := boot
TARGET := target/aarch64-unknown-none/release/${KERN}

.PHONY: all bin elf qemu objdump nm check clean install test

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
