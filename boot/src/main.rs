#![no_std]
#![no_main]

mod init;

use core::arch::asm;
use core::fmt::Write;
use core::result::Result::{Err, Ok};
use core::time::Duration;
use pi::uart::{BaudRate, MiniUart};
use shim::io;
use xmodem::Xmodem;

/// Start address of the binary to load and of the bootloader.
const BINARY_START_ADDR: usize = 0x80000;
const BOOTLOADER_START_ADDR: usize = 0x4000000;

/// Pointer to where the loaded binary expects to be loaded.
const BINARY_START: *mut u8 = BINARY_START_ADDR as *mut u8;

/// Free space between the bootloader and the loaded binary's start address.
const MAX_BINARY_SIZE: usize = BOOTLOADER_START_ADDR - BINARY_START_ADDR;

/// Branches to the address `addr` unconditionally.
unsafe fn jump_to(addr: *mut u8) -> ! {
    asm!(
        "br {x}",
        x = in(reg) (addr as usize)
    );
    loop {
        asm!("wfe");
    }
}

fn kmain() -> ! {
    // setup uart to recieve kernel
    // using a low baudrate because I found that to be the most consistent
    let mut uart = MiniUart::new(BaudRate::Baud38400);
    uart.set_read_timeout(Duration::from_millis(750));

    let mut binary = unsafe { core::slice::from_raw_parts_mut(BINARY_START, MAX_BINARY_SIZE) };

    // flush the nonsense UART starts with
    uart.clear();

    // repeatedly initiate a transfer
    loop {
        match Xmodem::receive(&mut uart, &mut binary) {
            Ok(_) => unsafe { jump_to(BINARY_START) },
            Err(err) => match err.kind() {
                io::ErrorKind::TimedOut => (), // just keep trying to receive
                _ => uart.write_str("error receiving over uart\n").unwrap(),
            },
        }
    }
}
