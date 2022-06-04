use num_traits::{FromPrimitive, One, ToPrimitive, Zero};
use std::ops::{Add, AddAssign, Div, Mul, Sub};

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
        .map(|v| if v.is_zero() { U::one() } else { U::zero() })
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
pub fn dot_product<T>(a: &[T], b: &[T]) -> T
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
pub fn argsort<T>(a: &mut [T], reverse: bool) -> Vec<usize>
where
    T: Ord + Copy,
{
    let mut ix: Vec<usize> = (0..a.len()).collect();
    ix.sort_by_key(|&i| a[i]);
    if reverse {
        ix.reverse()
    }
    (0..).zip(&ix).for_each(|(i, &j)| a.swap(i, j));
    ix
}
