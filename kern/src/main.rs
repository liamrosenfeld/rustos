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
use pi::timer;
use pi::gpio::{Gpio, Output};

fn blink(pin: &mut Gpio<Output>) {
    let pause = Duration::from_millis(200);
    pin.set();
    timer::spin_sleep(pause);
    pin.clear();
    timer::spin_sleep(pause);
}

unsafe fn kmain() -> ! {
    let mut pin = Gpio::new(16).into_output();
    blink(&mut pin);

    shell::shell("> ");

    loop {
        blink(&mut pin);
    }
}
