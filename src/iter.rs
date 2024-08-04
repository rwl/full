use crate::IsNaN;
use num_traits::{Bounded, Zero};
use std::iter::zip;
use std::ops::{Add, Mul};

/// Computes the dot-product of `a` and `b`.
pub fn dot<T>(a: impl Iterator<Item = T>, b: impl Iterator<Item = T>) -> T
where
    T: Mul<Output = T> + Add<Output = T> + Zero + Copy,
{
    return zip(a, b)
        .map(|(ai, bi)| ai * bi)
        .reduce(|x, y| x + y)
        .unwrap_or(T::zero());
}

/// Returns the maximum value of `a`.
pub fn max<T>(a: impl Iterator<Item = T>) -> T
where
    T: Bounded + PartialOrd + Copy,
{
    // assert_ne!(self.len(), 0);
    //
    // let mut v = T::min_value();
    // for &x in &self.data {
    //     if x > v {
    //         v = x;
    //     }
    // }
    // v

    a.max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
}

/// Returns the minimum value of `a`.
pub fn min<T>(a: impl Iterator<Item = T>) -> T
where
    T: Bounded + PartialOrd + Copy,
{
    // assert_ne!(self.len(), 0);
    //
    // let mut v = T::max_value();
    // for &x in &self.data {
    //     if x < v {
    //         v = x;
    //     }
    // }
    // v

    a.min_by(|&a, b| a.partial_cmp(b).unwrap()).unwrap()
}

/// Returns an integer array with 1s where `T::is_nan(self[i])`.
pub fn is_nan<T>(a: impl Iterator<Item = T>) -> Vec<usize>
where
    T: IsNaN,
{
    // let mut b = vec![0; a.len()];
    // for i in 0..a.len() {
    //     if T::is_nan(&a[i]) {
    //         b[i] = 1;
    //     }
    // }
    // b
    a.map(|v| if T::is_nan(&v) { 1 } else { 0 }).collect()
}
