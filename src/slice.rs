use crate::traits::{IsNaN, Norm, Sqrt};
use num_traits::bounds::Bounded;
use num_traits::{FromPrimitive, One, Pow, ToPrimitive, Zero};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub};

/// Returns a vector with the indexes of the nonzero elements of `a`.
pub fn find<T, U>(a: &[T]) -> Vec<U>
where
    T: Zero,
    U: FromPrimitive,
{
    a.iter()
        .enumerate()
        .filter_map(|(i, v)| {
            if !v.is_zero() {
                Some(U::from_usize(i).unwrap())
            } else {
                None
            }
        })
        .collect()
}

/// Returns true if any elements of `a` are nonzero.
pub fn any<T>(a: &[T]) -> bool
where
    T: Zero,
{
    for v in a {
        if !v.is_zero() {
            return true;
        }
    }
    false
}

/// Returns true if all elements of 'a' are nonzero.
pub fn all<T>(a: &[T]) -> bool
where
    T: Zero,
{
    for v in a {
        if v.is_zero() {
            return false;
        }
    }
    true
}

/// Returns a vector with monotonically increasing values from 0 to stop (exclusive).
pub fn range<T>(stop: usize) -> Vec<T>
where
    T: FromPrimitive,
{
    (0..stop).map(|i| T::from_usize(i).unwrap()).collect()
}

/// Creates a vector of evenly spaced values.
pub fn arange<T>(start: T, stop: T, step: T) -> Vec<T>
where
    T: Copy + Sub<Output = T> + Div<Output = T> + ToPrimitive + PartialOrd + AddAssign,
{
    let mut a = Vec::with_capacity(((stop - start) / step).to_usize().unwrap());
    let mut v = start;
    while v < stop {
        a.push(v);
        v += step;
    }
    a
}

/// Performs a logical (`!= 0`) "and" operation.
pub fn and<T, U>(a: &[T], b: &[T]) -> Vec<U>
where
    T: Zero,
    U: Zero + One,
{
    a.iter()
        .zip(b)
        .map(|(x, y)| {
            if !x.is_zero() && !y.is_zero() {
                U::one()
            } else {
                U::zero()
            }
        })
        .collect()
}

/// Performs a logical (`!= 0`) "or" operation.
pub fn or<T, U>(a: &[T], b: &[T]) -> Vec<U>
where
    T: Zero,
    U: Zero + One,
{
    a.iter()
        .zip(b)
        .map(|(x, y)| {
            if !x.is_zero() || !y.is_zero() {
                U::one()
            } else {
                U::zero()
            }
        })
        .collect()
}

pub fn not<T, U>(a: &[T]) -> Vec<U>
where
    T: Zero,
    U: One + Zero,
{
    a.iter()
        .map(|v| if v.is_zero() { U::one() } else { U::zero() })
        .collect()
}

/// Returns a vector with `1`s where `a[i] == v`.
pub fn eq<T, U>(a: &[T], v: T) -> Vec<U>
where
    T: Copy + PartialEq,
    U: Zero + One,
{
    a.iter()
        .map(|&w| if w == v { U::one() } else { U::zero() })
        .collect()
}

/// Returns a vector with `1`s where `a[i] != v`.
pub fn ne<T, U>(a: &[T], v: T) -> Vec<U>
where
    T: Copy + PartialEq,
    U: Zero + One,
{
    a.iter()
        .map(|&w| if w != v { U::one() } else { U::zero() })
        .collect()
}

/// Returns a vector with `1`s where `a[i] > v`.
pub fn gt<T, U>(a: &[T], v: T) -> Vec<U>
where
    T: Copy + PartialOrd,
    U: Zero + One,
{
    a.iter()
        .map(|&w| if w > v { U::one() } else { U::zero() })
        .collect()
}

/// Returns a vector with `1`s where `a[i] < v`.
pub fn lt<T, U>(a: &[T], v: T) -> Vec<U>
where
    T: Copy + PartialOrd,
    U: Zero + One,
{
    a.iter()
        .map(|&w| if w < v { U::one() } else { U::zero() })
        .collect()
}

/// Returns a vector with `1`s where `a[i] >= v`.
pub fn ge<T, U>(a: &[T], v: T) -> Vec<U>
where
    T: Copy + PartialOrd,
    U: Zero + One,
{
    a.iter()
        .map(|&w| if w >= v { U::one() } else { U::zero() })
        .collect()
}

/// Returns a vector with `1`s where `a[i] <= v`.
pub fn le<T, U>(a: &[T], v: T) -> Vec<U>
where
    T: Copy + PartialOrd,
    U: Zero + One,
{
    a.iter()
        .map(|&w| if w <= v { U::one() } else { U::zero() })
        .collect()
}

/// Returns a vector with 1s where `a[i] < b[i]`.
pub fn less_than<T, U>(a: &[T], b: &[T]) -> Vec<U>
where
    T: Copy + PartialOrd,
    U: One + Zero,
{
    a.iter()
        .zip(b)
        .map(|(&x, &y)| if x < y { U::one() } else { U::zero() })
        .collect()
}

/// Returns a vector with 1s where `a[i] > b[i]`.
pub fn greater_than<T, U>(a: &[T], b: &[T]) -> Vec<U>
where
    T: Copy + PartialOrd,
    U: One + Zero,
{
    a.iter()
        .zip(b)
        .map(|(&x, &y)| if x > y { U::one() } else { U::zero() })
        .collect()
}

/// Returns a vector with 1s where `a[i] == b[i]`.
pub fn equal<T, U>(a: &[T], b: &[T]) -> Vec<U>
where
    T: Copy + PartialEq,
    U: One + Zero,
{
    a.iter()
        .zip(b)
        .map(|(&x, &y)| if x == y { U::one() } else { U::zero() })
        .collect()
}

/// Returns a vector with 1s where `a[i] != b[i]`.
pub fn not_equal<T, U>(a: &[T], b: &[T]) -> Vec<U>
where
    T: Copy + PartialEq,
    U: One + Zero,
{
    a.iter()
        .zip(b)
        .map(|(&x, &y)| if x != y { U::one() } else { U::zero() })
        .collect()
}

/// Returns a vector with the values of `a` at indexes `ix`.
pub fn select<T>(a: &[T], ix: &[usize]) -> Vec<T>
where
    T: Copy,
{
    ix.iter().map(|&i| a[i]).collect()
}

/// Sets `a[ix] = v[ix]` in-place.
pub fn set_slice<T>(a: &mut [T], ix: &[usize], v: &[T])
where
    T: Copy,
{
    ix.iter().zip(v).for_each(|(&i, &x)| a[i] = x);
}

/// Sets `a[ix] = v` in-place.
pub fn set_all<T>(a: &mut [T], ix: &[usize], v: T)
where
    T: Copy,
{
    ix.iter().for_each(|&i| a[i] = v);
}

/// Returns a vector with `1`s where `a[i] != 0`.
pub fn nonzero<T, U>(a: &[T]) -> Vec<U>
where
    T: Zero,
    U: One + Zero,
{
    a.iter()
        .map(|v| if v.is_zero() { U::zero() } else { U::one() })
        .collect()
}

/// Returns the product of slice values.
pub fn prod<T>(a: &[T]) -> T
where
    T: Zero + Mul<T, Output = T> + Copy,
{
    a.into_iter()
        .map(|&v| v)
        .reduce(|x, y| x * y)
        .unwrap_or(T::zero())
}

/// Returns the cumulative sum of slice elements.
pub fn cum_sum<T>(a: &[T]) -> Vec<T>
where
    T: Zero + AddAssign + Copy,
{
    let mut sum = T::zero();
    a.iter()
        .map(|&v| {
            sum += v;
            sum
        })
        .collect()
}

/// Computes the dot-product of `a` and `b`.
pub fn dot<T>(a: &[T], b: &[T]) -> T
where
    T: Mul<Output = T> + Add<Output = T> + Zero + Copy,
{
    return a
        .iter()
        .zip(b)
        .map(|(&ai, &bi)| ai * bi)
        .reduce(|x, y| x + y)
        .unwrap_or(T::zero());
}

/// Sorts the elements of a into ascending order in-place and
/// returns the new indexes of the elements.
pub fn argsort<T>(a: &[T], reverse: bool) -> Vec<usize>
where
    T: PartialOrd + Copy,
{
    let mut ix: Vec<usize> = (0..a.len()).collect();
    // ix.sort_by_key(|&i| a[i]);
    ix.sort_unstable_by(|&i, &j| a[i].partial_cmp(&a[j]).unwrap());
    if reverse {
        ix.reverse()
    }
    ix
}

/// Computes the infinity norm: `max(abs(a))`
pub fn norm_inf<N: Norm<T>, T>(a: &[N]) -> T
where
    T: Bounded + PartialOrd,
{
    let mut max = T::min_value();
    for i in 0..a.len() {
        let absvi = a[i].norm();
        if absvi > max {
            max = absvi
        }
    }
    max
}

// pub fn norm_inf(a: ArrayView1<f64>) -> f64 {
//     a.iter()
//         .max_by(|&a, &b| a.partial_cmp(b).unwrap())
//         .unwrap()
//         .abs()
// }

/// Returns the index of the maximum value of `a`.
pub fn argmax<T>(a: &[T]) -> usize
where
    T: Bounded + PartialOrd + Copy,
{
    assert_ne!(a.len(), 0);

    let mut max = T::min_value();
    let mut ix = 0;
    for (i, &v) in a.iter().enumerate() {
        if v > max {
            max = v;
            ix = i;
        }
    }
    ix
}

/// Sums the values of `a`.
pub fn sum<T>(a: &[T]) -> T
where
    T: Zero + Copy + AddAssign,
{
    assert_ne!(a.len(), 0);
    let mut sum = T::zero();
    for &v in a {
        sum += v;
    }
    sum
}

/// Returns the mean of `a`.
pub fn mean<T>(a: &[T]) -> T
where
    T: Zero + Copy + AddAssign + Div<Output = T> + FromPrimitive,
{
    assert_ne!(a.len(), 0);
    // let n = a.len();
    // if n == 0 {
    // 	return 0
    // }
    sum(a) / T::from_usize(a.len()).unwrap()
}

/// Returns the standard deviation of `a` defined as: `Sqrt(Mean((x - Mean(x))^2))`
pub fn std<T>(a: &[T]) -> T
where
    T: Zero
        + Copy
        + AddAssign
        + Div<Output = T>
        + Sqrt
        + FromPrimitive
        + Pow<usize, Output = T>
        + Sub<Output = T>,
{
    let mean = mean(a);
    let mut sum = T::zero();
    for &v in a {
        sum += T::pow(v - mean, 2);
    }
    let variance = sum / T::from_usize(a.len()).unwrap();
    T::sqrt(&variance)
}

/// Returns an array with values linearly spaced
/// between start and stop (optionally inclusively).
pub fn linspace<T>(start: T, stop: T, num: usize, inclusive: bool) -> Vec<T>
where
    T: Zero
        + FromPrimitive
        + Copy
        + Sub<Output = T>
        + Div<Output = T>
        + PartialEq
        + DivAssign
        + MulAssign
        + AddAssign
        + One,
{
    let div = if inclusive { num - 1 } else { num };
    let div = T::from_usize(div).unwrap();

    // let mut y: Vec<T> = (0..num).map(|i| T::from_usize(i).unwrap()).collect();
    let mut y = vec![T::zero(); num];
    for i in 0..num {
        y[i] = T::from_usize(i).unwrap()
    }
    // let mut y = Arr::range(num);

    let delta = stop - start;
    if num > 1 {
        let step = delta / div;
        if step == T::zero() {
            y.iter_mut().for_each(|v| *v /= div);
            y.iter_mut().for_each(|v| *v *= delta);
        } else {
            y.iter_mut().for_each(|v| *v *= step);
        }
    } else {
        y.iter_mut().for_each(|v| *v *= delta);
    }

    y.iter_mut().for_each(|v| *v += start);

    if inclusive && num > 1 {
        y[num - 1] = stop
    }

    y
}

/// Returns a vector with the discrete difference of `a`.
/// Length of result is 1 less than length of `a`.
pub fn diff<T>(a: &[T]) -> Vec<T>
where
    T: Zero + Copy + Sub<Output = T>,
{
    assert_ne!(a.len(), 0);
    let n = a.len();
    if n == 1 {
        return vec![];
    }

    let mut b = vec![T::zero(); n - 1];
    for i in 1..n {
        b[i - 1] = a[i] - a[i - 1];
    }
    b
}

/// Returns the 2-norm (Euclidean) of `a`.
pub fn norm<T>(a: &[T]) -> T
where
    T: Zero + Copy + Sqrt + Mul<Output = T> + AddAssign,
{
    let mut sqsum = T::zero();
    for i in 0..a.len() {
        sqsum += a[i] * a[i];
    }
    T::sqrt(&sqsum)
}

/// Returns the maximum value of `a`.
pub fn max<T>(a: &[T]) -> T
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

    a.iter()
        .map(|&v| v)
        .max_by(|&a, b| a.partial_cmp(b).unwrap())
        .unwrap()
}

/// Returns the minimum value of `a`.
pub fn min<T>(a: &[T]) -> T
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

    a.iter()
        .map(|&v| v)
        .min_by(|&a, b| a.partial_cmp(b).unwrap())
        .unwrap()
}

/// Returns an integer array with 1s where `T::is_nan(self[i])`.
pub fn is_nan<T>(a: &[T]) -> Vec<usize>
where
    T: IsNaN,
{
    let mut b = vec![0; a.len()];
    for i in 0..a.len() {
        if T::is_nan(&a[i]) {
            b[i] = 1;
        }
    }
    b
}
