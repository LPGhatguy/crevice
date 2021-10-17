use bytemuck::Zeroable;

use crate::glsl::Glsl;
use crate::std140::{self, AsStd140};
use crate::std430::{self, AsStd430};

macro_rules! mint_vectors {
    ( $( $mint_ty:ty, $glsl_name:ident, $std_name:ident, ( $($field:ident),* ), )* ) => {
        $(
            impl AsStd140 for $mint_ty {
                type Output = std140::$std_name;

                fn as_std140(&self) -> Self::Output {
                    std140::$std_name {
                        $(
                            $field: self.$field,
                        )*
                    }
                }

                fn from_std140(value: Self::Output) -> Self {
                    Self {
                        $(
                            $field: value.$field,
                        )*
                    }
                }
            }

            impl AsStd430 for $mint_ty {
                type Output = std430::$std_name;

                fn as_std430(&self) -> Self::Output {
                    std430::$std_name {
                        $(
                            $field: self.$field,
                        )*
                    }
                }

                fn from_std430(value: Self::Output) -> Self {
                    Self {
                        $(
                            $field: value.$field,
                        )*
                    }
                }
            }

            unsafe impl Glsl for $mint_ty {
                const NAME: &'static str = stringify!($glsl_name);
            }
        )*
    };
}

mint_vectors! {
    mint::Vector2<f32>, vec2, Vec2, (x, y),
    mint::Vector3<f32>, vec3, Vec3, (x, y, z),
    mint::Vector4<f32>, vec4, Vec4, (x, y, z, w),

    mint::Vector2<i32>, ivec2, IVec2, (x, y),
    mint::Vector3<i32>, ivec3, IVec3, (x, y, z),
    mint::Vector4<i32>, ivec4, IVec4, (x, y, z, w),

    mint::Vector2<u32>, uvec2, UVec2, (x, y),
    mint::Vector3<u32>, uvec3, UVec3, (x, y, z),
    mint::Vector4<u32>, uvec4, UVec4, (x, y, z, w),

    // bool vectors are disabled due to https://github.com/LPGhatguy/crevice/issues/36

    // mint::Vector2<bool>, bvec2, BVec2, (x, y),
    // mint::Vector3<bool>, bvec3, BVec3, (x, y, z),
    // mint::Vector4<bool>, bvec4, BVec4, (x, y, z, w),

    mint::Vector2<f64>, dvec2, DVec2, (x, y),
    mint::Vector3<f64>, dvec3, DVec3, (x, y, z),
    mint::Vector4<f64>, dvec4, DVec4, (x, y, z, w),
}

macro_rules! mint_matrices {
    ( $( $mint_ty:ty, $glsl_name:ident, $std_name:ident, ( $($field:ident),* ), )* ) => {
        $(
            impl AsStd140 for $mint_ty {
                type Output = std140::$std_name;

                fn as_std140(&self) -> Self::Output {
                    std140::$std_name {
                        $(
                            $field: self.$field.as_std140(),
                        )*
                        ..Zeroable::zeroed()
                    }
                }

                fn from_std140(value: Self::Output) -> Self {
                    Self {
                        $(
                            $field: <_ as AsStd140>::from_std140(value.$field),
                        )*
                    }
                }
            }

            impl AsStd430 for $mint_ty {
                type Output = std430::$std_name;

                fn as_std430(&self) -> Self::Output {
                    std430::$std_name {
                        $(
                            $field: self.$field.as_std430(),
                        )*
                        ..Zeroable::zeroed()
                    }
                }

                fn from_std430(value: Self::Output) -> Self {
                    Self {
                        $(
                            $field: <_ as AsStd430>::from_std430(value.$field),
                        )*
                    }
                }
            }

            unsafe impl Glsl for $mint_ty {
                const NAME: &'static str = stringify!($glsl_name);
            }
        )*
    };
}

mint_matrices! {
    mint::ColumnMatrix2<f32>, mat2, Mat2, (x, y),
    mint::ColumnMatrix3<f32>, mat3, Mat3, (x, y, z),
    mint::ColumnMatrix4<f32>, mat4, Mat4, (x, y, z, w),

    mint::ColumnMatrix2<f64>, dmat2, DMat2, (x, y),
    mint::ColumnMatrix3<f64>, dmat3, DMat3, (x, y, z),
    mint::ColumnMatrix4<f64>, dmat4, DMat4, (x, y, z, w),
}
