#[repr(align(1))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Align1;

#[repr(align(2))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Align2;

#[repr(align(4))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Align4;

#[repr(align(8))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Align8;

#[repr(align(16))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Align16;
