use bytemuck::{Pod, Zeroable};

use crate::std140::Std140;

unsafe impl Std140 for f32 {
    const ALIGNMENT: usize = 4;
}

unsafe impl Std140 for f64 {
    const ALIGNMENT: usize = 8;
}

unsafe impl Std140 for i32 {
    const ALIGNMENT: usize = 4;
}

unsafe impl Std140 for u32 {
    const ALIGNMENT: usize = 4;
}

/// Corresponds to GLSL's `vec2`.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

unsafe impl Zeroable for Vec2 {}
unsafe impl Pod for Vec2 {}

unsafe impl Std140 for Vec2 {
    const ALIGNMENT: usize = 8;
}

/// Corresponds to GLSL's `vec3`.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

unsafe impl Zeroable for Vec3 {}
unsafe impl Pod for Vec3 {}

unsafe impl Std140 for Vec3 {
    const ALIGNMENT: usize = 16;
}

/// Corresponds to GLSL's `vec4`.
#[allow(missing_docs)]
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
    const ALIGNMENT: usize = 16;
}

/// Corresponds to GLSL's `mat2`.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy)]
pub struct Mat2 {
    pub x: Vec2,
    pub _pad_y: [f32; 2],
    pub y: Vec2,
}

unsafe impl Zeroable for Mat2 {}
unsafe impl Pod for Mat2 {}

unsafe impl Std140 for Mat2 {
    const ALIGNMENT: usize = 16;
}

/// Corresponds to GLSL's `mat3`.
#[allow(missing_docs)]
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
    const ALIGNMENT: usize = 16;
}

/// Corresponds to GLSL's `mat4`.
#[allow(missing_docs)]
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
    const ALIGNMENT: usize = 16;
}
