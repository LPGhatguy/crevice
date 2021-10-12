use insta::assert_yaml_snapshot;
use type_layout::TypeLayout;

use crevice::std140::{AsStd140, DVec4, Std140, Vec3};
use crevice::std430::AsStd430;

#[derive(AsStd140)]
struct PrimitiveF32 {
    x: f32,
    y: f32,
}

#[test]
fn primitive_f32() {
    assert_yaml_snapshot!(<<PrimitiveF32 as AsStd140>::Std140Type as TypeLayout>::type_layout());

    assert_eq!(<PrimitiveF32 as AsStd140>::Std140Type::ALIGNMENT, 16);

    let value = PrimitiveF32 { x: 1.0, y: 2.0 };
    let _value_std140 = value.as_std140();
}

#[derive(AsStd140)]
struct TestVec3 {
    pos: Vec3,
    velocity: Vec3,
}

#[test]
fn test_vec3() {
    assert_yaml_snapshot!(<<TestVec3 as AsStd140>::Std140Type as TypeLayout>::type_layout());

    assert_eq!(<TestVec3 as AsStd140>::Std140Type::ALIGNMENT, 16);

    let value = TestVec3 {
        pos: Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        },
        velocity: Vec3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        },
    };
    let _value_std140 = value.as_std140();
}

#[derive(AsStd140)]
struct UsingVec3Padding {
    pos: Vec3,
    brightness: f32,
}

#[test]
fn using_vec3_padding() {
    assert_yaml_snapshot!(
        <<UsingVec3Padding as AsStd140>::Std140Type as TypeLayout>::type_layout()
    );

    assert_eq!(<UsingVec3Padding as AsStd140>::Std140Type::ALIGNMENT, 16);

    let value = UsingVec3Padding {
        pos: Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        },
        brightness: 4.0,
    };
    let _value_std140 = value.as_std140();
}

#[derive(AsStd140)]
struct PointLight {
    position: Vec3,
    diffuse: Vec3,
    specular: Vec3,
    brightness: f32,
}

#[test]
fn point_light() {
    assert_yaml_snapshot!(<<PointLight as AsStd140>::Std140Type as TypeLayout>::type_layout());

    assert_eq!(<PointLight as AsStd140>::Std140Type::ALIGNMENT, 16);

    let value = PointLight {
        position: Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        },
        diffuse: Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        },
        specular: Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        },
        brightness: 4.0,
    };
    let _value_std140 = value.as_std140();
}

#[derive(AsStd140)]
struct MoreThan16Alignment {
    doubles: DVec4,
}

#[test]
fn more_than_16_alignment() {
    assert_yaml_snapshot!(
        <<MoreThan16Alignment as AsStd140>::Std140Type as TypeLayout>::type_layout()
    );

    assert_eq!(<MoreThan16Alignment as AsStd140>::Std140Type::ALIGNMENT, 32);
}

#[derive(AsStd140)]
struct PaddingAtEnd {
    base_value: PrimitiveF32,
    small_field: f32,
}

#[test]
fn padding_at_end() {
    assert_yaml_snapshot!(<<PaddingAtEnd as AsStd140>::Std140Type as TypeLayout>::type_layout());
}

#[derive(AsStd140, AsStd430)]
struct MatrixUniform {
    e: mint::ColumnMatrix3<f32>,
    f: f32,
}

#[test]
fn matrix_uniform_std140() {
    assert_yaml_snapshot!(<<MatrixUniform as AsStd140>::Std140Type as TypeLayout>::type_layout());
}

#[test]
fn matrix_uniform_std430() {
    assert_yaml_snapshot!(<<MatrixUniform as AsStd430>::Std430Type as TypeLayout>::type_layout())
}

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

#[test]
fn proper_offset_calculations_for_differing_member_sizes() {
    assert_yaml_snapshot!(
        <<ProperlyChecksForUnderlyingTypeSize as AsStd140>::Std140Type as TypeLayout>::type_layout(
        )
    )
}

#[derive(AsStd140, Debug, PartialEq)]
struct ThereAndBackAgain {
    view: mint::ColumnMatrix3<f32>,
    origin: mint::Vector3<f32>,
}

#[test]
fn there_and_back_again() {
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
