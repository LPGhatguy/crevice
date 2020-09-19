use bytemuck::Zeroable;

use crate::std140::{self, AsStd140};

impl AsStd140 for mint::Vector2<f32> {
    type Std140Type = std140::Vec2;

    fn as_std140(&self) -> Self::Std140Type {
        std140::Vec2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl AsStd140 for mint::Vector3<f32> {
    type Std140Type = std140::Vec3;

    fn as_std140(&self) -> Self::Std140Type {
        std140::Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl AsStd140 for mint::Vector4<f32> {
    type Std140Type = std140::Vec4;

    fn as_std140(&self) -> Self::Std140Type {
        std140::Vec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.w,
        }
    }
}

impl AsStd140 for mint::ColumnMatrix2<f32> {
    type Std140Type = std140::Mat2;

    fn as_std140(&self) -> Self::Std140Type {
        std140::Mat2 {
            x: self.x.as_std140(),
            y: self.y.as_std140(),
            ..Zeroable::zeroed()
        }
    }
}

impl AsStd140 for mint::ColumnMatrix3<f32> {
    type Std140Type = std140::Mat3;

    fn as_std140(&self) -> Self::Std140Type {
        std140::Mat3 {
            x: self.x.as_std140(),
            y: self.y.as_std140(),
            z: self.z.as_std140(),
            ..Zeroable::zeroed()
        }
    }
}

impl AsStd140 for mint::ColumnMatrix4<f32> {
    type Std140Type = std140::Mat4;

    fn as_std140(&self) -> Self::Std140Type {
        std140::Mat4 {
            x: self.x.as_std140(),
            y: self.y.as_std140(),
            z: self.z.as_std140(),
            w: self.w.as_std140(),
        }
    }
}
