use core::fmt::Debug;
use core::mem::{transmute_copy, MaybeUninit};

use bytemuck::{Pod, Zeroable};

use super::{AsStd430, Std430};

pub unsafe trait Std430ArrayItem: Std430 {
    type Padding: Zeroable + Copy + Debug;
}

type Padded<T> = (T, <T as Std430ArrayItem>::Padding);

fn wrap<T: AsStd430>(x: &T) -> Padded<T::Output>
where
    T::Output: Std430ArrayItem,
{
    (x.as_std430(), Zeroable::zeroed())
}

fn unwrap<T: AsStd430>(x: Padded<T::Output>) -> T
where
    T::Output: Std430ArrayItem,
{
    T::from_std430(x.0)
}

#[doc(hidden)]
#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Std430Array<T: Std430ArrayItem, const N: usize>([Padded<T>; N]);

unsafe impl<T: Std430ArrayItem, const N: usize> Zeroable for Std430Array<T, N> {}
unsafe impl<T: Std430ArrayItem, const N: usize> Pod for Std430Array<T, N> {}
unsafe impl<T: Std430ArrayItem, const N: usize> Std430 for Std430Array<T, N> {
    const ALIGNMENT: usize = T::ALIGNMENT;
}

impl<T: AsStd430, const N: usize> AsStd430 for [T; N]
where
    T::Output: Std430ArrayItem,
{
    type Output = Std430Array<T::Output, N>;
    fn as_std430(&self) -> Self::Output {
        let mut res: [MaybeUninit<_>; N] = unsafe { MaybeUninit::uninit().assume_init() };

        for i in 0..N {
            res[i] = MaybeUninit::new(wrap(&self[i]));
        }

        unsafe { transmute_copy(&res) }
    }

    fn from_std430(val: Self::Output) -> Self {
        val.0.map(|x| unwrap(x))
    }
}

unsafe impl Std430ArrayItem for f32 {
    type Padding = [u8; 0];
}

unsafe impl Std430ArrayItem for f64 {
    type Padding = [u8; 0];
}

unsafe impl Std430ArrayItem for i32 {
    type Padding = [u8; 0];
}

unsafe impl Std430ArrayItem for u32 {
    type Padding = [u8; 0];
}

unsafe impl Std430ArrayItem for crate::bool::Bool {
    type Padding = [u8; 0];
}
