/*!
[![GitHub CI Status](https://github.com/LPGhatguy/crevice/workflows/CI/badge.svg)](https://github.com/LPGhatguy/crevice/actions)
[![crevice on crates.io](https://img.shields.io/crates/v/crevice.svg)](https://crates.io/crates/crevice)
[![crevice docs](https://img.shields.io/badge/docs-docs.rs-orange.svg)](https://docs.rs/crevice)

Crevice creates GLSL-compatible versions of types through the power of derive
macros. Generated structs implement [`bytemuck::Zeroable`][Zeroable] and
[`bytemuck::Pod`][Pod] to ease packing data into buffers for uploading.

Crevice is similar to [`glsl-layout`][glsl-layout], but supports `mint` types
and explicitly initializes padding to remove one source of undefined behavior.

Examples in this crate use cgmath, but any math crate that works with the mint
crate will also work. Some other crates include nalgebra, ultraviolet, glam, and
vek.

## Examples

Uploading many types can be done by deriving `AsStd140` and using the bytemuck
crate to turn the result into bytes.

```rust
use crevice::std140::AsStd140;
use cgmath::prelude::*;
use cgmath::{Matrix3, Vector3};

#[derive(AsStd140)]
struct MainUniform {
    orientation: mint::ColumnMatrix3<f32>,
    position: mint::Vector3<f32>,
    scale: f32,
}

let value = MainUniform {
    orientation: Matrix3::identity().into(),
    position: Vector3::new(1.0, 2.0, 3.0).into(),
    scale: 4.0,
};

let value_std140 = value.as_std140();

# fn upload_data_to_gpu(_value: &[u8]) {}
upload_data_to_gpu(bytemuck::bytes_of(&value_std140));
```

More complicated data can be uploaded using the std140 `Writer` type:

```rust
use crevice::std140::{self, AsStd140};

#[derive(AsStd140)]
struct PointLight {
    position: mint::Vector3<f32>,
    color: mint::Vector3<f32>,
    brightness: f32,
}

let lights = vec![
    PointLight {
        position: [0.0, 1.0, 0.0].into(),
        color: [1.0, 0.0, 0.0].into(),
        brightness: 0.6,
    },
    PointLight {
        position: [0.0, 4.0, 3.0].into(),
        color: [1.0, 1.0, 1.0].into(),
        brightness: 1.0,
    },
];

# fn map_gpu_buffer_for_write() -> &'static mut [u8] {
#     Box::leak(vec![0; 1024].into_boxed_slice())
# }
let target_buffer = map_gpu_buffer_for_write();
let mut writer = std140::Writer::new(target_buffer);

let light_count = lights.len() as u32;
writer.write(&light_count)?;

// Crevice will automatically insert the required padding to align the
// PointLight structure correctly. In this case, there will be 12 bytes of
// padding between the length field and the light list.

for light in &lights {
    writer.write(light)?;

    // Crevice will also pad between each array element.
}

# fn unmap_gpu_buffer() {}
unmap_gpu_buffer();

# Ok::<(), std::io::Error>(())
```

## Minimum Supported Rust Version (MSRV)

Crevice supports Rust 1.46.0 and newer due to use of new `const fn` features.

[glsl-layout]: https://github.com/rustgd/glsl-layout
[Zeroable]: https://docs.rs/bytemuck/latest/bytemuck/trait.Zeroable.html
[Pod]: https://docs.rs/bytemuck/latest/bytemuck/trait.Pod.html
[TypeLayout]: https://docs.rs/type-layout/latest/type_layout/trait.TypeLayout.html
*/

pub use bytemuck;

pub mod std140;

#[doc(hidden)]
pub mod internal;

mod mint;
