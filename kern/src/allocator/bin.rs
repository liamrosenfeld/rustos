use core::alloc::Layout;
use core::cmp::max;
use core::fmt;
use core::option::Option::{self, None, Some};
use core::ptr;
use core::write;

use crate::allocator::linked_list::{Cursor, LinkedList};
use crate::allocator::util::*;
use crate::allocator::LocalAlloc;

const BIN_SIZES: &[usize] = &[8, 16, 32, 64, 128, 256, 512, 1024, 2048];
const MIN_BIN: usize = BIN_SIZES[0];
const MAX_BIN: usize = *(BIN_SIZES.last().unwrap());

/// A simple allocator that allocates based on size classes.
///   - bin 0 (2^3 bytes)    : handles allocations in (0, 2^3]
///   - bin 1 (2^4 bytes)    : handles allocations in (2^3, 2^4]
///   - ...
///   - bin 8 (2^11 bytes)  : handles allocations in (2^10, 2^11]
///
/// It also uses a flexible linked list allocator for larger allocations.
/// The fallback is used because for large allocations, the amount of internal
/// fragmentation becomes unreasonable (because it can be up to 50%)
///   
pub struct Allocator {
    /// array of `NUM_BINS` linked lists
    /// `bins[i]` contains allocations of size 2^(i + SMALLEST_EXP)
    bins: [LinkedList; BIN_SIZES.len()],

    /// a linked list of freed chunks that are larger than the MAX_SIZE
    fallback: LinkedList,

    /// the line between memory that is either allocated or in a bin
    /// and memory that has never been touched
    current: usize,

    /// the final memory address
    end: usize,
}

impl Allocator {
    /// Creates a new bin allocator that will allocate memory from the region
    /// starting at address `start` and ending at address `end`.
    pub fn new(start: usize, end: usize) -> Allocator {
        const EMPTY_LINKED_LIST: LinkedList = LinkedList::new();
        Allocator {
            bins: [EMPTY_LINKED_LIST; BIN_SIZES.len()],
            fallback: LinkedList::new(),
            current: start,
            end,
        }
    }

    /* -------------  Bin Allocation ------------- */
    /// returns the smallest bin that would still fit `size`
    fn bin_for_size(size: usize) -> Option<usize> {
        if size > MAX_BIN {
            return None;
        }

        // effectively ceil(log2(size)) - SMALLEST_EXP (saturating)
        BIN_SIZES.iter().position(|&s| s >= size)
    }

    /// allocate new memory that is sized for a bin
    fn alloc_for_bin(&mut self, bin: usize, align: usize) -> *mut u8 {
        // TODO: spliting larger bins or pop from fallback
        let size = BIN_SIZES[bin];
        let align = max(size, align); // ensures it's aligned to the size of the bin
        self.alloc_new(Layout::from_size_align(size, align).unwrap())
    }

    /* -------------  Fallback Allocation ------------- */
    /// try to find a chunk of memory that fits the layout in the linked list
    fn find_fallback_chunk(&mut self, layout: Layout) -> Option<*mut u8> {
        for cursor in self.fallback.iter_mut() {
            unsafe { return self.alloc_in_chunk(cursor, layout) }
        }
        None
    }

    /// allocate inside of chunk of memory in the fallback linked list
    /// splits availiable extra memory off
    /// returns: pointer to the start of allocated memory
    unsafe fn alloc_in_chunk(&mut self, cursor: Cursor, layout: Layout) -> Option<*mut u8> {
        let chunk = &*cursor.value();

        // see if it fits in the chunk
        let (start, end) = Self::align_range(chunk.start_addr(), layout);
        if end > chunk.end_addr() {
            return None;
        }

        // save the gaps to reduce wasted memory
        let before_gap = start - chunk.start_addr();
        let after_gap = chunk.end_addr() - end;
        self.handle_gap(chunk.start_addr(), before_gap);
        self.handle_gap(end, after_gap);

        // remove the chunk from the list and return the start
        cursor.pop();
        Some(start as *mut u8)
    }

    /* ------------- Shared ------------- */
    /// Allocate new memory after `current` that fits a layout
    fn alloc_new(&mut self, layout: Layout) -> *mut u8 {
        // check if valid
        if layout.size() <= 0 || !is_power_of_two(layout.align()) {
            return ptr::null_mut();
        }

        // get range for allocation
        let (start, end) = Self::align_range(self.current, layout);

        // check that it fits in memory
        if end > self.end {
            return ptr::null_mut();
        }

        // if the gap before the new aligned allocation fits in a bin, add it there
        let gap = start - self.current;
        self.handle_gap(start, gap);

        // return pointer to start
        self.current = end;
        start as *mut u8
    }

    fn handle_gap(&mut self, start: usize, gap: usize) {
        match gap {
            0..MIN_BIN => {}
            MIN_BIN..=MAX_BIN => {
                // fits in a bin, so split and add there
                self.fit_in_largest_bin(start, gap);
            }
            _ => {
                // larger than a bin, so add to fallback
                unsafe {
                    self.fallback.push(start as *mut u8, gap);
                }
            }
        };
    }

    /// Creates a new bin entry for a chunk of memory. Uses the most memory possible.
    fn fit_in_largest_bin(&mut self, start: usize, available_size: usize) {
        if available_size < BIN_SIZES[0] {
            return;
        }
        for (bin, bin_size) in BIN_SIZES.iter().enumerate().rev() {
            let (new_start, end) = Self::align_range(
                start,
                Layout::from_size_align(*bin_size, *bin_size).unwrap(),
            );

            if end < (start + available_size) {
                let ptr = new_start as *mut u8;
                unsafe {
                    self.bins[bin].push(ptr, 0);
                }
                return;
            }
        }
    }

    fn align_range(start: usize, layout: Layout) -> (usize, usize) {
        let aligned_start = align_up(start, layout.align());
        let aligned_end = align_up(aligned_start.saturating_add(layout.size()), layout.align());
        (aligned_start, aligned_end)
    }
}

impl LocalAlloc for Allocator {
    unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
        // ensure that the bin_for_size accounts for aligment
        let size = max(layout.size(), layout.align());

        // allocate either as a bin or linked list fallback
        match Self::bin_for_size(size) {
            Some(bin) => {
                // check if there's an availiable slot in a bin and use that
                // (allocating new memory if not)
                match self.bins[bin].pop() {
                    Some(chunk) => chunk as *mut u8,
                    None => self.alloc_for_bin(bin, layout.align()),
                }
            }
            None => {
                // allocate using fallback
                // search the fallback linked list for a slot that fits
                match self.find_fallback_chunk(layout) {
                    Some(ptr) => ptr,
                    None => self.alloc_new(layout),
                }
            }
        }
    }

    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        // the max ensures it matches the alloc logic for sorting bins
        let size = max(layout.size(), layout.align());
        match Self::bin_for_size(size) {
            Some(bin) => self.bins[bin].push(ptr, 0),
            None => self.fallback.push(ptr, layout.size()),
        }
    }
}

impl fmt::Debug for Allocator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Current: {} End: {}\n", self.current, self.end)?;
        write!(f, "Bins:\n")?;
        for bin in self.bins {
            write!(f, "    {:?}\n", bin)?;
        }
        write!(f, "Fallback: {:?}", self.fallback)
    }
}
