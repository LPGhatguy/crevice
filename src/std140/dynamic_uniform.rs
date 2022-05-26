use bytemuck::{Pod, Zeroable};

use crate::internal::max;
use crate::std140::{AsStd140, Std140};

/// Wrapper type that aligns the inner type to at least 256 bytes.
///
/// This type is useful for ensuring correct alignment when creating dynamic
/// uniform buffers in APIs like WebGPU.
pub struct DynamicUniform<T>(pub T);

impl<T: AsStd140> AsStd140 for DynamicUniform<T> {
    type Output = DynamicUniformStd140<<T as AsStd140>::Output>;

    fn as_std140(&self) -> Self::Output {
        DynamicUniformStd140(self.0.as_std140())
    }

    fn from_std140(value: Self::Output) -> Self {
        DynamicUniform(<T as AsStd140>::from_std140(value.0))
    }
}

/// std140 variant of [`DynamicUniform`].
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct DynamicUniformStd140<T>(T);

unsafe impl<T: Std140> Std140 for DynamicUniformStd140<T> {
    const ALIGNMENT: usize = max(256, T::ALIGNMENT);
}

unsafe impl<T: Zeroable> Zeroable for DynamicUniformStd140<T> {}
unsafe impl<T: Pod> Pod for DynamicUniformStd140<T> {}

#[cfg(test)]
mod test {
    use super::*;

    use crate::std140::AsStd140;

    #[test]
    fn size_is_unchanged() {
        assert_eq!(
            DynamicUniform::<f32>::std140_size_static(),
            f32::std140_size_static()
        );
    }

    #[test]
    #[cfg(feature = "std")]
    fn alignment_applies() {
        use crate::std140;

        let mut output = Vec::new();
        let mut writer = std140::Writer::new(&mut output);

        writer.write(&DynamicUniform(0.0f32)).unwrap();
        assert_eq!(writer.len(), 4);

        writer.write(&DynamicUniform(1.0f32)).unwrap();
        assert_eq!(writer.len(), 260);
    }
}
