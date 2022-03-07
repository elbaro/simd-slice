#![feature(portable_simd)]

use std::simd::{Simd, SimdElement};

pub trait AsSimdSlice<T: SimdElement> {
    fn as_simd_slice<'a>(&'a self) -> SimdSlice<'a, T>;
}

impl<T: SimdElement> AsSimdSlice<T> for [T] {
    fn as_simd_slice<'a>(&'a self) -> SimdSlice<'a, T> {
        SimdSlice(self)
    }
}

pub struct SimdSlice<'a, T: SimdElement>(&'a [T]);

macro_rules! impl_op {
    ($t: ty) => {
        impl<'a> SimdSlice<'a, $t> {
            pub fn sum(&self) -> $t {
                let n = self.0.len() & (!3); // round down to a multiple of 4
                let slice = &self.0[..n];

                let mut i = 0;
                let mut agg = Simd::<$t, 4>::splat(num::zero());

                // this is much slower:
                // slice
                //     .chunks(4)
                //     .for_each(|s| agg += Simd::<$t, 4>::from_slice(s));

                while i < n {
                    agg += Simd::<$t, 4>::from_slice(unsafe { &slice.get_unchecked(i..i + 4) });
                    i += 4;
                }
                let mut agg = agg.horizontal_sum();
                self.0[n..].iter().for_each(|x| agg += x);
                agg
            }

            pub fn min(&self) -> Option<$t> {
                let n = self.0.len() & (!3); // round down to a multiple of 4

                if n > 0 {
                    let slice = &self.0[..n];
                    let mut i = 4;
                    let mut agg = Simd::<$t, 4>::from_slice(&slice[..4]);
                    while i < n {
                        agg += Simd::<$t, 4>::from_slice(unsafe { &slice.get_unchecked(i..i + 4) });
                        i += 4;
                    }
                    let mut agg = agg.horizontal_min();
                    self.0[n..].iter().for_each(|&x| {
                        if x < agg {
                            agg = x;
                        }
                    });
                    Some(agg)
                } else if self.0.len() > 0 {
                    let mut agg = self.0[0];
                    self.0[1..].iter().for_each(|&x| {
                        if x < agg {
                            agg = x;
                        }
                    });
                    Some(agg)
                } else {
                    None
                }
            }

            pub fn max(&self) -> Option<$t> {
                let n = self.0.len() & (!3); // round down to a multiple of 4

                if n > 0 {
                    let slice = &self.0[..n];
                    let mut i = 4;
                    let mut agg = Simd::<$t, 4>::from_slice(&slice[..4]);
                    while i < n {
                        agg += Simd::<$t, 4>::from_slice(unsafe { &slice.get_unchecked(i..i + 4) });
                        i += 4;
                    }
                    let mut agg = agg.horizontal_max();
                    self.0[n..].iter().for_each(|&x| {
                        if x < agg {
                            agg = x;
                        }
                    });
                    Some(agg)
                } else if self.0.len() > 0 {
                    let mut agg = self.0[0];
                    self.0[1..].iter().for_each(|&x| {
                        if x < agg {
                            agg = x;
                        }
                    });
                    Some(agg)
                } else {
                    None
                }
            }
        }
    };
}
impl_op!(u8);
impl_op!(u16);
impl_op!(u32);
impl_op!(u64);
impl_op!(usize);
impl_op!(i8);
impl_op!(i16);
impl_op!(i32);
impl_op!(i64);
impl_op!(isize);
impl_op!(f32);
impl_op!(f64);

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        use super::AsSimdSlice;

        let a = &[10, 20, 3, 4, 5, 6, 7_i32];
        assert_eq!(a[1..5].iter().sum::<i32>(), a[1..5].as_simd_slice().sum());
        assert_eq!(a[1..5].iter().max().cloned(), a[1..5].as_simd_slice().max());
        assert_eq!(a[1..5].iter().min().cloned(), a[1..5].as_simd_slice().min());
    }
}
