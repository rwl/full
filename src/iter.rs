use crate::traits::{Arg, Complex, IsNaN, Norm};

use num_traits::{Bounded, One, Pow, Zero};
use std::cmp::Ordering;
use std::iter::zip;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

pub fn sub<T>(a: impl Iterator<Item = T>, b: impl Iterator<Item = T>) -> impl Iterator<Item = T>
where
    T: Copy + Sub<Output = T>,
{
    zip(a, b).map(|(ai, bi)| ai - bi)
}

pub fn pow<T, P>(a: impl Iterator<Item = T>, x: P) -> impl Iterator<Item = T>
where
    T: Copy + Pow<P, Output = T>,
    P: Copy + 'static,
{
    a.map(move |ai| ai.pow(x))
}

/// Returns an iterator with the values of `a` at indexes `ix`.
pub fn select<'a, T>(
    a: &'a [T],
    ix: impl Iterator<Item = usize> + 'a,
) -> impl Iterator<Item = T> + 'a
where
    T: Copy,
{
    ix.map(|i| a[i])
}

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

/// Returns the maximum elements from `a` or `b`.
pub fn max2<T>(a: impl Iterator<Item = T>, b: impl Iterator<Item = T>) -> impl Iterator<Item = T>
where
    T: Bounded + PartialOrd + Copy,
{
    zip(a, b).map(|(a, b)| match a.partial_cmp(&b).unwrap() {
        Ordering::Less => a,
        Ordering::Equal => a,
        Ordering::Greater => b,
    })
}

/// Returns an integer array with 1s where `T::is_nan(self[i])`.
pub fn is_nan<T>(a: impl Iterator<Item = T>) -> impl Iterator<Item = usize>
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
    a.map(|v| if T::is_nan(&v) { 1 } else { 0 })
}

/// Returns the absolute value of each element in input `a`.
pub fn abs<T, F>(a: impl Iterator<Item = T>) -> impl Iterator<Item = F>
where
    T: Norm<F>,
{
    a.map(|x| x.norm())
}

/// Returns the phase angle for each element of a complex array `a`.
pub fn angle<T, F>(a: impl Iterator<Item = T>) -> impl Iterator<Item = F>
where
    T: Arg<F>,
{
    a.map(|x| x.arg())
}

/// Sums the values of `a`.
pub fn sum<T>(a: impl Iterator<Item = T>) -> T
where
    T: Zero + Copy + AddAssign,
{
    let mut sum = T::zero();
    for v in a {
        sum += v;
    }
    sum
}

pub fn real<C, F>(a: impl Iterator<Item = C>) -> impl Iterator<Item = F>
where
    C: Complex<F>,
{
    a.map(|v| v.real())
}

pub fn imag<C, F>(a: impl Iterator<Item = C>) -> impl Iterator<Item = F>
where
    C: Complex<F>,
{
    a.map(|v| v.imag())
}

pub fn neg<T>(a: impl Iterator<Item = T>) -> impl Iterator<Item = T>
where
    T: Neg<Output = T>,
{
    a.map(|x| -x)
}

pub fn polar<T, C>(
    r: impl Iterator<Item = T>,
    theta: impl Iterator<Item = T>,
) -> impl Iterator<Item = C>
where
    C: Complex<T>,
{
    zip(r, theta).map(|p| C::from_polar(p.0, p.1))
}

pub fn complex<T, C>(
    re: impl Iterator<Item = T>,
    im: impl Iterator<Item = T>,
) -> impl Iterator<Item = C>
where
    C: Complex<T>,
{
    zip(re, im).map(|p| C::new(p.0, p.1))
}

pub fn conj<T, C>(a: impl IntoIterator<Item = C>) -> impl Iterator<Item = C>
where
    C: Complex<T>,
{
    a.into_iter().map(|c| c.conj())
}

pub fn recip<T>(a: impl IntoIterator<Item = T>) -> impl Iterator<Item = T>
where
    T: One + Div<Output = T> + Copy,
{
    a.into_iter().map(|v| T::one() / v)
}
