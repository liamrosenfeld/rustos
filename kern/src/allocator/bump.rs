use core::alloc::Layout;
use core::fmt::Debug;
use core::prelude::rust_2021::derive;
use core::ptr;

use crate::allocator::util::*;
use crate::allocator::LocalAlloc;

/// A "bump" allocator: allocates memory by bumping a pointer; never frees.
#[derive(Debug)]
pub struct Allocator {
    current: usize,
    end: usize,
}

impl Allocator {
    /// Creates a new bump allocator that will allocate memory from the region
    /// starting at address `start` and ending at address `end`.
    #[allow(dead_code)]
    pub fn new(start: usize, end: usize) -> Allocator {
        Allocator {
            current: start,
            end,
        }
    }
}

impl LocalAlloc for Allocator {
    unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
        // check if valid
        if layout.size() <= 0 || !is_power_of_two(layout.align()) {
            return ptr::null_mut();
        }

        // get range for allocation
        let start = align_up(self.current, layout.align());
        let end = align_up(start.saturating_add(layout.size()), layout.align());

        // return pointer to start
        if end > self.end {
            return ptr::null_mut();
        }
        self.current = end;
        start as *mut u8
    }

    unsafe fn dealloc(&mut self, _ptr: *mut u8, _layout: Layout) {
        // LEAKED
    }
}
