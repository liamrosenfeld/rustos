use core::panic;

/// Align `addr` downwards to the nearest multiple of `align`.
///
/// The returned usize is always <= `addr.`
///
/// # Panics
///
/// Panics if `align` is not a power of 2.
pub fn align_down(addr: usize, align: usize) -> usize {
    if !is_power_of_two(align) {
        panic!("Alignment of {} is not power of 2", align);
    }
    addr - (addr % align)
}

/// Align `addr` upwards to the nearest multiple of `align`.
///
/// The returned `usize` is always >= `addr.`
///
/// # Panics
///
/// Panics if `align` is not a power of 2
/// or aligning up overflows the address.
pub fn align_up(addr: usize, align: usize) -> usize {
    if !is_power_of_two(align) {
        panic!("Alignment of {} is not power of 2", align);
    }
    if addr % align == 0 {
        addr
    } else {
        (addr + align) - (addr % align)
    }
}

#[inline(always)]
pub fn is_power_of_two(num: usize) -> bool {
    // if a number is a power of two, it'll only have
    // a single 1 in the binary representation and the
    // number below will be entirely 1s below the 1 in
    // the original
    (num & (num - 1)) == 0
}
