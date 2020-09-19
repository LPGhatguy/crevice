use bytemuck::{Pod, Zeroable};

use crate::alignment::{Align16, Align4, Align8};

pub use crevice_derive::AsStd140;

pub unsafe trait Std140: Copy + Zeroable + Pod {
    type Alignment;
}

pub trait AsStd140 {
    type Std140Type: Std140;

    fn as_std140(&self) -> Self::Std140Type;
}

impl<T> AsStd140 for T
where
    T: Std140,
{
    type Std140Type = Self;

    fn as_std140(&self) -> Self {
        *self
    }
}

unsafe impl Std140 for f32 {
    type Alignment = Align4;
}

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

unsafe impl Zeroable for Vec2 {}
unsafe impl Pod for Vec2 {}

unsafe impl Std140 for Vec2 {
    type Alignment = Align8;
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

unsafe impl Zeroable for Vec3 {}
unsafe impl Pod for Vec3 {}

unsafe impl Std140 for Vec3 {
    type Alignment = Align16;
}

#[derive(Debug, Clone, Copy)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

unsafe impl Zeroable for Vec4 {}
unsafe impl Pod for Vec4 {}

unsafe impl Std140 for Vec4 {
    type Alignment = Align16;
}

#[derive(Debug, Clone, Copy)]
pub struct Mat2 {
    pub x: Vec2,
    pub _pad_y: [f32; 2],
    pub y: Vec2,
}

unsafe impl Zeroable for Mat2 {}
unsafe impl Pod for Mat2 {}

unsafe impl Std140 for Mat2 {
    type Alignment = Align16;
}

#[derive(Debug, Clone, Copy)]
pub struct Mat3 {
    pub x: Vec3,
    pub _pad_y: f32,
    pub y: Vec3,
    pub _pad_z: f32,
    pub z: Vec3,
}

unsafe impl Zeroable for Mat3 {}
unsafe impl Pod for Mat3 {}

unsafe impl Std140 for Mat3 {
    type Alignment = Align16;
}

#[derive(Debug, Clone, Copy)]
pub struct Mat4 {
    pub x: Vec4,
    pub y: Vec4,
    pub z: Vec4,
    pub w: Vec4,
}

unsafe impl Zeroable for Mat4 {}
unsafe impl Pod for Mat4 {}

unsafe impl Std140 for Mat4 {
    type Alignment = Align16;
}
