use core::fmt::{Debug, Formatter};

use bytemuck::{Pod, Zeroable};

/// GLSL's `bool` type.
///
/// Boolean values in GLSL are 32 bits, in contrast with Rust's 8 bit bools.
#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(transparent)]
pub struct Bool(u32);

unsafe impl Zeroable for Bool {}
unsafe impl Pod for Bool {}

impl From<bool> for Bool {
    fn from(v: bool) -> Self {
        Self(v as u32)
    }
}

impl From<Bool> for bool {
    fn from(v: Bool) -> Self {
        v.0 != 0
    }
}

impl Debug for Bool {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Bool({:?})", bool::from(*self))
    }
}
