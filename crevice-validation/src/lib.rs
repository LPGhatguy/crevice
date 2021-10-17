#![cfg(test)]

mod gpu;

#[macro_use]
mod util;

use crevice::glsl::GlslStruct;
use crevice::std140::AsStd140;
use mint::{ColumnMatrix3, Vector2, Vector3, Vector4};

use gpu::assert_round_trip;

#[test]
fn two_f32() {
    #[derive(Debug, PartialEq, AsStd140, GlslStruct)]
    struct TwoF32 {
        x: f32,
        y: f32,
    }

    assert_std140_offsets!((size = 8, align = 16) TwoF32 {
        x: 0,
        y: 4,
    });

    assert_round_trip(TwoF32 { x: 5.0, y: 7.0 });
}

#[test]
fn vec2() {
    #[derive(Debug, PartialEq, AsStd140, GlslStruct)]
    struct UseVec2 {
        one: Vector2<f32>,
    }

    assert_std140_offsets!((size = 8, align = 16) UseVec2 {
        one: 0,
    });

    assert_round_trip(UseVec2 {
        one: [1.0, 2.0].into(),
    });
}

#[test]
fn mat3() {
    #[derive(Debug, PartialEq, AsStd140, GlslStruct)]
    struct TestData {
        one: ColumnMatrix3<f32>,
    }

    assert_round_trip(TestData {
        one: [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]].into(),
    });
}

#[test]
fn dvec4() {
    #[derive(Debug, PartialEq, AsStd140, GlslStruct)]
    struct UsingDVec4 {
        doubles: Vector4<f64>,
    }

    assert_std140_offsets!((size = 32, align = 32) UsingDVec4 {
        doubles: 0,
    });

    assert_round_trip(UsingDVec4 {
        doubles: [1.0, 2.0, 3.0, 4.0].into(),
    });
}

#[test]
fn two_vec3() {
    #[derive(Debug, PartialEq, AsStd140, GlslStruct)]
    struct TwoVec3 {
        one: Vector3<f32>,
        two: Vector3<f32>,
    }

    assert_std140_offsets!((size = 32, align = 16) TwoVec3 {
        one: 0,
        two: 16,
    });

    assert_round_trip(TwoVec3 {
        one: [1.0, 2.0, 3.0].into(),
        two: [4.0, 5.0, 6.0].into(),
    });
}

#[test]
fn two_vec4() {
    #[derive(Debug, PartialEq, AsStd140, GlslStruct)]
    struct TestData {
        one: Vector4<f32>,
        two: Vector4<f32>,
    }

    assert_round_trip(TestData {
        one: [1.0, 2.0, 3.0, 4.0].into(),
        two: [5.0, 6.0, 7.0, 8.0].into(),
    });
}

#[test]
fn vec3_then_f32() {
    #[derive(Debug, PartialEq, AsStd140, GlslStruct)]
    struct Vec3ThenF32 {
        one: Vector3<f32>,
        two: f32,
    }

    assert_std140_offsets!((size = 20, align = 16) Vec3ThenF32 {
        one: 0,
        two: 16,
    });

    assert_round_trip(Vec3ThenF32 {
        one: [1.0, 2.0, 3.0].into(),
        two: 4.0,
    });
}

#[test]
fn mat3_padding() {
    #[derive(Debug, PartialEq, AsStd140, GlslStruct)]
    struct Mat3Padding {
        one: mint::ColumnMatrix3<f32>,
        two: f32,
    }

    assert_std140_offsets!((size = 40, align = 16) Mat3Padding {
        one: 0,
        two: 36,
    });

    assert_round_trip(Mat3Padding {
        one: [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]].into(),
        two: 10.0,
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
