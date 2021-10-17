use memoffset::offset_of;

use crevice::glsl::GlslStruct;
use crevice::std140::{AsStd140, DVec4, Std140, Vec3};
use crevice::std430::AsStd430;

macro_rules! assert_std140_offsets {
    ((size = $size:literal, align = $align:literal) $struct:ident {
        $( $field:ident: $offset:literal, )*
    }) => {
        type Target = <$struct as AsStd140>::Output;

        assert_eq!(std::mem::size_of::<Target>(), $size);
        assert_eq!(Target::ALIGNMENT, $align);
        $( assert_eq!(offset_of!(Target, $field), $offset); )*
    };
}

#[test]
fn two_f32() {
    #[derive(AsStd140)]
    struct TwoF32 {
        x: f32,
        y: f32,
    }

    assert_std140_offsets!((size = 8, align = 16) TwoF32 {
        x: 0,
        y: 4,
    });
}

#[test]
fn two_vec3() {
    #[derive(AsStd140)]
    struct TwoVec3 {
        pos: Vec3,
        velocity: Vec3,
    }

    assert_std140_offsets!((size = 32, align = 16) TwoVec3 {
        pos: 0,
        velocity: 16,
    });
}

#[test]
fn vec3_then_f32() {
    #[derive(AsStd140)]
    struct Vec3ThenF32 {
        pos: Vec3,
        brightness: f32,
    }

    assert_std140_offsets!((size = 20, align = 16) Vec3ThenF32 {
        pos: 0,
        brightness: 16,
    });
}

#[test]
fn point_light() {
    #[derive(AsStd140)]
    struct PointLight {
        position: Vec3,
        diffuse: Vec3,
        specular: Vec3,
        brightness: f32,
    }

    assert_std140_offsets!((size = 52, align = 16) PointLight {
        position: 0,
        diffuse: 16,
        specular: 32,
        brightness: 48,
    });
}

#[test]
fn dvec4() {
    #[derive(AsStd140)]
    struct UsingDVec4 {
        doubles: DVec4,
    }

    assert_std140_offsets!((size = 32, align = 32) UsingDVec4 {
        doubles: 0,
    });
}

#[test]
fn padding_after_struct() {
    #[derive(AsStd140)]
    struct TwoF32 {
        x: f32,
        y: f32,
    }

    #[derive(AsStd140)]
    struct PaddingAfterStruct {
        base_value: TwoF32,
        small_field: f32,
    }

    assert_std140_offsets!((size = 20, align = 16) PaddingAfterStruct {
        base_value: 0,
        small_field: 16,
    });
}

#[test]
fn mat3_padding() {
    #[derive(AsStd140, AsStd430)]
    struct Mat3Padding {
        e: mint::ColumnMatrix3<f32>,
        f: f32,
    }

    assert_std140_offsets!((size = 40, align = 16) Mat3Padding {
        e: 0,
        f: 36,
    });
}

#[test]
fn proper_offset_calculations_for_differing_member_sizes() {
    /// Rust size: 4, align: 4
    /// Std140 size: 4, align: 16
    #[derive(AsStd140)]
    struct PaddedByStdButNotRust {
        x: f32,
    }

    /// Rust size: 8, align: 4
    /// Std140 size: 20, align: 16
    #[derive(AsStd140)]
    struct BaseSizeAndStdSizeAreDifferent {
        first: PaddedByStdButNotRust,
        second: PaddedByStdButNotRust,
    }

    /// If checking for base struct size, produces layout:
    /// (padding 0) (field 20) (padding 8) (field 4)
    /// which does not properly align the second member.
    #[derive(AsStd140)]
    struct ProperlyChecksForUnderlyingTypeSize {
        leading: BaseSizeAndStdSizeAreDifferent,
        trailing: PaddedByStdButNotRust,
    }

    assert_std140_offsets!((size = 36, align = 16) ProperlyChecksForUnderlyingTypeSize {
        leading: 0,
        trailing: 32,
    });
}

#[test]
fn there_and_back_again() {
    #[derive(AsStd140, Debug, PartialEq)]
    struct ThereAndBackAgain {
        view: mint::ColumnMatrix3<f32>,
        origin: mint::Vector3<f32>,
    }

    let x = ThereAndBackAgain {
        view: mint::ColumnMatrix3 {
            x: mint::Vector3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            y: mint::Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            z: mint::Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        },
        origin: mint::Vector3 {
            x: 0.0,
            y: 1.0,
            z: 2.0,
        },
    };
    let x_as = x.as_std140();
    assert_eq!(<ThereAndBackAgain as AsStd140>::from_std140(x_as), x);
}

#[test]
fn generate_struct_glsl() {
    #[allow(dead_code)]
    #[derive(GlslStruct)]
    struct TestGlsl {
        foo: mint::Vector3<f32>,
        bar: mint::ColumnMatrix2<f32>,
    }

    insta::assert_display_snapshot!(TestGlsl::glsl_definition());
}
