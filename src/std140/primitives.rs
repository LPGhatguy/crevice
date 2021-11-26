use bytemuck::{Pod, Zeroable};

use crate::bool::Bool;
use crate::glsl::Glsl;
use crate::std140::{AsStd140, Std140};

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

unsafe impl Std140 for Bool {
    const ALIGNMENT: usize = 4;
}

impl AsStd140 for bool {
    type Output = Bool;

    fn as_std140(&self) -> Self::Output {
        (*self).into()
    }

    fn from_std140(val: Self::Output) -> Self {
        val.into()
    }
}

macro_rules! vectors {
    (
        $(
            #[$doc:meta] align($align:literal) $glsl_name:ident $name:ident <$prim:ident> ($($field:ident),+)
        )+
    ) => {
        $(
            #[$doc]
            #[allow(missing_docs)]
            #[derive(Debug, Clone, Copy, PartialEq)]
            #[repr(C)]
            pub struct $name {
                $(pub $field: $prim,)+
            }

            unsafe impl Zeroable for $name {}
            unsafe impl Pod for $name {}

            unsafe impl Std140 for $name {
                const ALIGNMENT: usize = $align;
            }

            unsafe impl Glsl for $name {
                const NAME: &'static str = stringify!($glsl_name);
            }
        )+
    };
}

vectors! {
    #[doc = "Corresponds to a GLSL `vec2` in std140 layout."] align(8) vec2 Vec2<f32>(x, y)
    #[doc = "Corresponds to a GLSL `vec3` in std140 layout."] align(16) vec3 Vec3<f32>(x, y, z)
    #[doc = "Corresponds to a GLSL `vec4` in std140 layout."] align(16) vec4 Vec4<f32>(x, y, z, w)

    #[doc = "Corresponds to a GLSL `ivec2` in std140 layout."] align(8) ivec2 IVec2<i32>(x, y)
    #[doc = "Corresponds to a GLSL `ivec3` in std140 layout."] align(16) ivec3 IVec3<i32>(x, y, z)
    #[doc = "Corresponds to a GLSL `ivec4` in std140 layout."] align(16) ivec4 IVec4<i32>(x, y, z, w)

    #[doc = "Corresponds to a GLSL `uvec2` in std140 layout."] align(8) uvec2 UVec2<u32>(x, y)
    #[doc = "Corresponds to a GLSL `uvec3` in std140 layout."] align(16) uvec3 UVec3<u32>(x, y, z)
    #[doc = "Corresponds to a GLSL `uvec4` in std140 layout."] align(16) uvec4 UVec4<u32>(x, y, z, w)

    #[doc = "Corresponds to a GLSL `bvec2` in std140 layout."] align(8) bvec2 BVec2<Bool>(x, y)
    #[doc = "Corresponds to a GLSL `bvec3` in std140 layout."] align(16) bvec3 BVec3<Bool>(x, y, z)
    #[doc = "Corresponds to a GLSL `bvec4` in std140 layout."] align(16) bvec4 BVec4<Bool>(x, y, z, w)

    #[doc = "Corresponds to a GLSL `dvec2` in std140 layout."] align(16) dvec2 DVec2<f64>(x, y)
    #[doc = "Corresponds to a GLSL `dvec3` in std140 layout."] align(32) dvec3 DVec3<f64>(x, y, z)
    #[doc = "Corresponds to a GLSL `dvec4` in std140 layout."] align(32) dvec4 DVec4<f64>(x, y, z, w)
}

macro_rules! matrices {
    (
        $(
            #[$doc:meta]
            align($align:literal)
            $glsl_name:ident $name:ident {
                $($field:ident: $field_ty:ty,)+
            }
        )+
    ) => {
        $(
            #[$doc]
            #[allow(missing_docs)]
            #[derive(Debug, Clone, Copy)]
            #[repr(C)]
            pub struct $name {
                $(pub $field: $field_ty,)+
            }

            unsafe impl Zeroable for $name {}
            unsafe impl Pod for $name {}

            unsafe impl Std140 for $name {
                const ALIGNMENT: usize = $align;
            }

            unsafe impl Glsl for $name {
                const NAME: &'static str = stringify!($glsl_name);
            }
        )+
    };
}

matrices! {
    #[doc = "Corresponds to a GLSL `mat2` in std140 layout."]
    align(16)
    mat2 Mat2 {
        x: Vec2,
        _pad_x: [f32; 2],
        y: Vec2,
        _pad_y: [f32; 2],
    }

    #[doc = "Corresponds to a GLSL `mat3` in std140 layout."]
    align(16)
    mat3 Mat3 {
        x: Vec3,
        _pad_x: f32,
        y: Vec3,
        _pad_y: f32,
        z: Vec3,
        _pad_z: f32,
    }

    #[doc = "Corresponds to a GLSL `mat4` in std140 layout."]
    align(16)
    mat4 Mat4 {
        x: Vec4,
        y: Vec4,
        z: Vec4,
        w: Vec4,
    }

    #[doc = "Corresponds to a GLSL `dmat2` in std140 layout."]
    align(16)
    dmat2 DMat2 {
        x: DVec2,
        y: DVec2,
    }

    #[doc = "Corresponds to a GLSL `dmat3` in std140 layout."]
    align(32)
    dmat3 DMat3 {
        x: DVec3,
        _pad_x: f64,
        y: DVec3,
        _pad_y: f64,
        z: DVec3,
        _pad_z: f64,
    }

    #[doc = "Corresponds to a GLSL `dmat3` in std140 layout."]
    align(32)
    dmat4 DMat4 {
        x: DVec4,
        y: DVec4,
        z: DVec4,
        w: DVec4,
    }
}
