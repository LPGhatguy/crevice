use std::mem::size_of;

use bytemuck::{bytes_of, Pod, Zeroable};

/// Trait implemented for all `std140` primitives. Generally should not be
/// implemented outside this crate.
pub unsafe trait Std140: Copy + Zeroable + Pod {
    /// The required alignment of the type. Must be a power of two.
    ///
    /// This is distinct from the value returned by `std::mem::align_of` because
    /// `AsStd140` structs do not use Rust's alignment. This enables them to
    /// control and zero their padding bytes, making converting them to and from
    /// slices safe.
    const ALIGNMENT: usize;

    /// Casts the type to a byte array. Implementors should not override this
    /// method.
    ///
    /// # Safety
    /// This is always safe due to the requirements of [`bytemuck::Pod`] being a
    /// prerequisite for this trait.
    fn as_bytes(&self) -> &[u8] {
        bytes_of(self)
    }
}

/**
Trait implemented for all types that can be turned into `std140` values.

This trait can often be `#[derive]`'d instead of manually implementing it. Any
struct which contains only fields that also implement `AsStd140` can derive
`AsStd140`.

Types from the mint crate implement `AsStd140`, making them convenient for use
in uniform types. Most Rust geometry crates, like cgmath, nalgebra, and
ultraviolet support mint.

## Example

```glsl
uniform CAMERA {
    mat4 view;
    mat4 projection;
} camera;
```

```
use cgmath::prelude::*;
use cgmath::{Matrix4, Deg, perspective};
use crevice::std140::{AsStd140, Std140};

#[derive(AsStd140)]
struct CameraUniform {
    view: mint::ColumnMatrix4<f32>,
    projection: mint::ColumnMatrix4<f32>,
}

let camera = CameraUniform {
    view: Matrix4::identity().into(),
    projection: perspective(Deg(60.0), 16.0/9.0, 0.01, 100.0).into(),
};

# fn write_to_gpu_buffer(bytes: &[u8]) {}
let camera_std140 = camera.as_std140();
write_to_gpu_buffer(camera_std140.as_bytes());
```
*/
pub trait AsStd140 {
    /// The `std140` version of this value.
    type Std140Type: Std140;

    /// Convert this value into the `std140` version of itself.
    fn as_std140(&self) -> Self::Std140Type;

    /// Returns the size of the `std140` version of this type. Useful for
    /// pre-sizing buffers.
    fn std140_size(&self) -> usize {
        size_of::<Self::Std140Type>()
    }
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
