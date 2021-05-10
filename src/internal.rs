//! This module is internal to crevice but used by its derive macro. No
//! guarantees are made about its contents.

pub use bytemuck;

/// Align the given struct offset up to the given alignment.
pub const fn align_offset(offset: usize, alignment: usize) -> usize {
    if offset % alignment == 0 {
        0
    } else {
        alignment - offset % alignment
    }
}

/// Max of two `usize`. Implemented because the `max` method from `Ord` cannot
/// be used in const fns.
pub const fn max(a: usize, b: usize) -> usize {
    if a > b {
        a
    } else {
        b
    }
}

pub const fn pad_at_end(size: usize, alignment: usize, do_pad: bool) -> usize {
    if do_pad {
        align_offset(size, alignment)
    }
    else {
        0
    }
}