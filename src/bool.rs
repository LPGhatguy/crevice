use bytemuck::{Pod, Zeroable};
use core::fmt::{Debug, Formatter};

#[derive(Eq, PartialEq, Clone, Copy)]
pub struct GlslBoolean(u32);

unsafe impl Zeroable for GlslBoolean {}
unsafe impl Pod for GlslBoolean {}

impl From<bool> for GlslBoolean {
    fn from(v: bool) -> Self {
        Self(v as u32)
    }
}

impl From<GlslBoolean> for bool {
    fn from(v: GlslBoolean) -> Self {
        v.0 != 0
    }
}

impl Debug for GlslBoolean {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "GlslBoolean({:?})", bool::from(*self))
    }
}
