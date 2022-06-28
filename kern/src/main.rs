#![feature(alloc_error_handler)]
#![feature(decl_macro)]
#![feature(auto_traits)]
#![feature(negative_impls)]
#![feature(optin_builtin_traits)]
#![feature(raw_vec_internals)]

#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(not(test))]
mod init;

extern crate alloc;

pub mod allocator;
pub mod console;
pub mod fs;
pub mod mutex;
pub mod shell;

use core::time::Duration;
use pi::gpio::{Gpio, Output};
use pi::timer;

fn blink(pin: &mut Gpio<Output>) {
    let pause = Duration::from_millis(200);
    pin.set();
    timer::spin_sleep(pause);
    pin.clear();
    timer::spin_sleep(pause);
}

use allocator::Allocator;
use fs::FileSystem;

#[cfg_attr(not(test), global_allocator)]
pub static ALLOCATOR: Allocator = Allocator::uninitialized();
pub static FILESYSTEM: FileSystem = FileSystem::uninitialized();

fn kmain() -> ! {
    unsafe {
        ALLOCATOR.initialize();
        FILESYSTEM.initialize();
    }

    kprintln!("Welcome to cs3210!");
    shell::shell("> ");
}
