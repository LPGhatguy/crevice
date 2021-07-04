use bytemuck::{Pod, Zeroable};

use crate::std430::{Std430, Std430Padded};

use crate::internal::align_offset;
use core::mem::size_of;

unsafe impl Std430 for f32 {
    const ALIGNMENT: usize = 4;
    type Padded = Self;
}

unsafe impl Std430 for f64 {
    const ALIGNMENT: usize = 8;
    type Padded = Self;
}

unsafe impl Std430 for i32 {
    const ALIGNMENT: usize = 4;
    type Padded = Self;
}

unsafe impl Std430 for u32 {
    const ALIGNMENT: usize = 4;
    type Padded = Self;
}

macro_rules! vectors {
    (
        $(
            #[$doc:meta] align($align:literal) $name:ident <$prim:ident> ($($field:ident),+)
        )+
    ) => {
        $(
            #[$doc]
            #[allow(missing_docs)]
            #[derive(Debug, Clone, Copy)]
            pub struct $name {
                $(pub $field: $prim,)+
            }

            unsafe impl Zeroable for $name {}
            unsafe impl Pod for $name {}

            unsafe impl Std430 for $name {
                const ALIGNMENT: usize = $align;
                type Padded = Std430Padded<Self, {align_offset(size_of::<$name>(), $align)}>;
            }
        )+
    };
}

vectors! {
    #[doc = "Corresponds to a GLSL `vec2` in std430 layout."] align(8) Vec2<f32>(x, y)
    #[doc = "Corresponds to a GLSL `vec3` in std430 layout."] align(16) Vec3<f32>(x, y, z)
    #[doc = "Corresponds to a GLSL `vec4` in std430 layout."] align(16) Vec4<f32>(x, y, z, w)

    #[doc = "Corresponds to a GLSL `ivec2` in std140 layout."] align(8) IVec2<i32>(x, y)
    #[doc = "Corresponds to a GLSL `ivec3` in std140 layout."] align(16) IVec3<i32>(x, y, z)
    #[doc = "Corresponds to a GLSL `ivec4` in std140 layout."] align(16) IVec4<i32>(x, y, z, w)

    #[doc = "Corresponds to a GLSL `uvec2` in std140 layout."] align(8) UVec2<u32>(x, y)
    #[doc = "Corresponds to a GLSL `uvec3` in std140 layout."] align(16) UVec3<u32>(x, y, z)
    #[doc = "Corresponds to a GLSL `uvec4` in std140 layout."] align(16) UVec4<u32>(x, y, z, w)

    #[doc = "Corresponds to a GLSL `bvec2` in std140 layout."] align(8) BVec2<bool>(x, y)
    #[doc = "Corresponds to a GLSL `bvec3` in std140 layout."] align(16) BVec3<bool>(x, y, z)
    #[doc = "Corresponds to a GLSL `bvec4` in std140 layout."] align(16) BVec4<bool>(x, y, z, w)

    #[doc = "Corresponds to a GLSL `dvec2` in std430 layout."] align(16) DVec2<f64>(x, y)
    #[doc = "Corresponds to a GLSL `dvec3` in std430 layout."] align(32) DVec3<f64>(x, y, z)
    #[doc = "Corresponds to a GLSL `dvec4` in std430 layout."] align(32) DVec4<f64>(x, y, z, w)
}

macro_rules! matrices {
    (
        $(
            #[$doc:meta]
            align($align:literal)
            $name:ident {
                $($field:ident: $field_ty:ty,)+
            }
        )+
    ) => {
        $(
            #[$doc]
            #[allow(missing_docs)]
            #[derive(Debug, Clone, Copy)]
            pub struct $name {
                $(pub $field: $field_ty,)+
            }

            unsafe impl Zeroable for $name {}
            unsafe impl Pod for $name {}

            unsafe impl Std430 for $name {
                const ALIGNMENT: usize = $align;
                /// Matrices are technically arrays of primitives, and as such require pad at end.
                const PAD_AT_END: bool = true;
                type Padded = Std430Padded<Self, {align_offset(size_of::<$name>(), $align)}>;
            }
        )+
    };
}

matrices! {
    #[doc = "Corresponds to a GLSL `mat2` in std430 layout."]
    align(16)
    Mat2 {
        x: Vec2,
        y: Vec2,
    }

    #[doc = "Corresponds to a GLSL `mat3` in std430 layout."]
    align(16)
    Mat3 {
        x: Vec3,
        y: Vec3,
        z: Vec3,
    }

    #[doc = "Corresponds to a GLSL `mat4` in std430 layout."]
    align(16)
    Mat4 {
        x: Vec4,
        y: Vec4,
        z: Vec4,
        w: Vec4,
    }

    #[doc = "Corresponds to a GLSL `dmat2` in std430 layout."]
    align(16)
    DMat2 {
        x: DVec2,
        y: DVec2,
    }

    #[doc = "Corresponds to a GLSL `dmat3` in std430 layout."]
    align(32)
    DMat3 {
        x: DVec3,
        y: DVec3,
        z: DVec3,
    }

    #[doc = "Corresponds to a GLSL `dmat3` in std430 layout."]
    align(32)
    DMat4 {
        x: DVec4,
        y: DVec4,
        z: DVec4,
        w: DVec4,
    }
}
