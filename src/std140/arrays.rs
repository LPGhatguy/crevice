use core::fmt::Debug;
use core::mem::{transmute_copy, MaybeUninit};

use bytemuck::{Pod, Zeroable};

use super::{AsStd140, Std140};

pub unsafe trait Std140ArrayItem: Std140 {
    type Padding: Zeroable + Copy + Debug;
}

type Padded<T> = (T, <T as Std140ArrayItem>::Padding);

fn wrap<T: AsStd140>(x: &T) -> Padded<T::Output>
where
    T::Output: Std140ArrayItem,
{
    (x.as_std140(), Zeroable::zeroed())
}

fn unwrap<T: AsStd140>(x: Padded<T::Output>) -> T
where
    T::Output: Std140ArrayItem,
{
    T::from_std140(x.0)
}

#[doc(hidden)]
#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Std140Array<T: Std140ArrayItem, const N: usize>([Padded<T>; N]);

unsafe impl<T: Std140ArrayItem, const N: usize> Zeroable for Std140Array<T, N> {}
unsafe impl<T: Std140ArrayItem, const N: usize> Pod for Std140Array<T, N> {}
unsafe impl<T: Std140ArrayItem, const N: usize> Std140 for Std140Array<T, N> {
    const ALIGNMENT: usize = crate::internal::max(16, T::ALIGNMENT);
}

impl<T: AsStd140, const N: usize> AsStd140 for [T; N]
where
    T::Output: Std140ArrayItem,
{
    type Output = Std140Array<T::Output, N>;
    fn as_std140(&self) -> Self::Output {
        let mut res: [MaybeUninit<_>; N] = unsafe { MaybeUninit::uninit().assume_init() };

        for i in 0..N {
            res[i] = MaybeUninit::new(wrap(&self[i]));
        }

        unsafe { transmute_copy(&res) }
    }

    fn from_std140(val: Self::Output) -> Self {
        val.0.map(|x| unwrap(x))
    }
}

unsafe impl Std140ArrayItem for f32 {
    type Padding = [u8; 12];
}

unsafe impl Std140ArrayItem for f64 {
    type Padding = [u8; 8];
}

unsafe impl Std140ArrayItem for i32 {
    type Padding = [u8; 12];
}

unsafe impl Std140ArrayItem for u32 {
    type Padding = [u8; 12];
}

unsafe impl Std140ArrayItem for crate::bool::Bool {
    type Padding = [u8; 12];
}
