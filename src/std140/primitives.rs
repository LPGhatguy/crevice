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

            unsafe impl Std140 for $name {
                const ALIGNMENT: usize = $align;
            }
        )+
    };
}

vectors! {
    #[doc = "Corresponds to GLSL's `vec2`."] align(8) Vec2<f32>(x, y)
    #[doc = "Corresponds to GLSL's `vec3`."] align(16) Vec3<f32>(x, y, z)
    #[doc = "Corresponds to GLSL's `vec4`."] align(16) Vec4<f32>(x, y, z, w)

    #[doc = "Corresponds to GLSL's `dvec2`."] align(16) DVec2<f64>(x, y)
    #[doc = "Corresponds to GLSL's `dvec3`."] align(32) DVec3<f64>(x, y, z)
    #[doc = "Corresponds to GLSL's `dvec4`."] align(32) DVec4<f64>(x, y, z, w)
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

            unsafe impl Std140 for $name {
                const ALIGNMENT: usize = $align;
            }
        )+
    };
}

matrices! {
    #[doc = "Corresponds to GLSL's `mat2`."]
    align(16)
    Mat2 {
        x: Vec2,
        _pad_y: [f32; 2],
        y: Vec2,
    }

    #[doc = "Corresponds to GLSL's `mat3`."]
    align(16)
    Mat3 {
        x: Vec3,
        _pad_y: f32,
        y: Vec3,
        _pad_z: f32,
        z: Vec3,
    }

    #[doc = "Corresponds to GLSL's `mat4`."]
    align(16)
    Mat4 {
        x: Vec4,
        y: Vec4,
        z: Vec4,
        w: Vec4,
    }

    #[doc = "Corresponds to GLSL's `dmat2`."]
    align(16)
    DMat2 {
        x: DVec2,
        y: DVec2,
    }

    #[doc = "Corresponds to GLSL's `dmat2`."]
    align(32)
    DMat3 {
        x: DVec3,
        _pad_x: f64,
        y: DVec3,
        _pad_y: f64,
        z: DVec3,
    }

    #[doc = "Corresponds to GLSL's `dmat3`."]
    align(32)
    DMat4 {
        x: DVec4,
        y: DVec4,
        z: DVec4,
        w: DVec4,
    }
}
