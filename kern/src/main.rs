#![feature(alloc_error_handler)]
#![feature(decl_macro)]
#![feature(auto_traits)]
#![feature(negative_impls)]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(not(test))]
mod init;

pub mod console;
pub mod mutex;
pub mod shell;

use core::time::Duration;
use console::kprintln;
use pi::timer;
use pi::gpio::Gpio;

unsafe fn kmain() -> ! {
    // Set GPIO Pin 16 as output.
    let mut pin16 = Gpio::new(16).into_output();
    let mut pin21 = Gpio::new(21).into_output();

    // Continuously set and clear GPIO 16.
    let pause = Duration::from_millis(100);
    loop {
        pin16.set();
        timer::spin_sleep(pause);
        pin21.set();
        timer::spin_sleep(pause);
        pin16.clear();
        timer::spin_sleep(pause);
        pin21.clear();
        timer::spin_sleep(pause);
    }
}
