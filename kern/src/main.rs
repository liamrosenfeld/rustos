#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
// features
#![feature(alloc_error_handler)]
#![feature(decl_macro)]
#![feature(auto_traits)]
#![feature(negative_impls)]
#![feature(panic_info_message)]
#![feature(exclusive_range_pattern)]
#![feature(const_mut_refs)]
#![feature(const_option)]

#[cfg(not(test))]
mod init;

extern crate alloc;

pub mod allocator;
pub mod console;
// pub mod fs;
pub mod mutex;
pub mod shell;

use console::kprintln;
use core::time::Duration;
use pi::atags::Atags;
// use pi::gpio::{Gpio, Output};
use alloc::string::String;
use pi::timer;

use allocator::Allocator;
// use fs::FileSystem;

#[cfg_attr(not(test), global_allocator)]
pub static ALLOCATOR: Allocator = Allocator::uninitialized();
// pub static FILESYSTEM: FileSystem = FileSystem::uninitialized();

fn kmain() -> ! {
    timer::spin_sleep(Duration::from_millis(3000));
    let atags = Atags::get();
    for atag in atags {
        kprintln!("{:#?}", atag);
    }
    unsafe {
        ALLOCATOR.initialize();
        // FILESYSTEM.initialize();
    }
    let test = String::from("string on the heap");
    kprintln!("your string is: {}", test);

    use alloc::vec::Vec;

    let mut v = Vec::new();
    for i in 0..25 {
        v.push(i);
        kprintln!("{:?}", v);
    }
    kprintln!("{:?}", ALLOCATOR);

    shell::shell("> ");
}
