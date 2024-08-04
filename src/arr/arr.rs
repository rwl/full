use crate::full::{ones, to_string, zeros};
use crate::slice::{
    all, any, arange, argmax, argsort, cum_sum, diff, find, is_nan, linspace, max, mean, min,
    nonzero, norm, prod, range, select, set_all, set_slice, std,
};
use crate::traits::{Abs, ArcCos, ArcSin, Cos, Exp, IsNaN, Ln, Round, Sin, Sqrt};

use num_traits::{Bounded, FromPrimitive, One, Pow, ToPrimitive, Zero};
use std::fmt::{Display, Formatter};
use std::ops::{AddAssign, Deref, DerefMut, Div, DivAssign, MulAssign, Sub};

// pub type Array = Arr<f64>;

#[derive(Debug, Clone)]
pub struct Arr<T> {
    pub(crate) values: Vec<T>,
    // pub(crate) column: bool,
}

impl<T> Arr<T>
where
    T: Clone + Copy + Zero + One,
{
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn with_vec(values: Vec<T>) -> Self {
        Self { values }
    }

    pub fn with_value(n: usize, v: T) -> Self {
        Self { values: vec![v; n] }
    }

    pub fn with_capacity(n: usize) -> Self {
        Self {
            values: Vec::with_capacity(n),
        }
    }

    pub fn zeros(n: usize) -> Self {
        Self {
            values: zeros(1, n),
        }
    }

    pub fn ones(n: usize) -> Self {
        Self { values: ones(1, n) }
    }

    pub fn range(stop: usize) -> Self
    where
        T: FromPrimitive,
    {
        Self {
            values: range(stop),
        }
    }
    pub fn arange(start: T, stop: T, step: T) -> Self
    where
        T: Sub<Output = T> + Div<Output = T> + ToPrimitive + PartialOrd + AddAssign,
    {
        Self {
            values: arange(start, stop, step),
        }
    }

    /// Returns an array with values linearly spaced
    /// between start and stop (optionally inclusively).
    pub fn linspace(start: T, stop: T, num: usize, inclusive: bool) -> Self
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
        Self {
            values: linspace(start, stop, num, inclusive),
        }
    }

    pub fn concat(a: &[&[T]]) -> Self {
        Self { values: a.concat() }
    }

    pub fn values(&self) -> &[T] {
        &self.values
    }

    pub fn vec(self) -> Vec<T> {
        self.values
    }

    pub fn find<U>(&self) -> Vec<U>
    where
        U: FromPrimitive,
    {
        find(&self.values)
    }
    pub fn any(&self) -> bool {
        any(&self.values)
    }
    pub fn all(&self) -> bool {
        all(&self.values)
    }

    pub fn select(&self, ix: &[usize]) -> Arr<T> {
        Arr {
            values: select(&self.values, ix),
        }
    }

    pub fn set(&mut self, ix: &[usize], v: &[T]) {
        assert_eq!(ix.len(), v.len());
        set_slice(&mut self.values, ix, v);
    }

    pub fn set_all(&mut self, ix: &[usize], v: T) {
        set_all(&mut self.values, ix, v);
    }

    pub fn sum(&self) -> T {
        self.values
            .iter()
            .map(|&x| x)
            .reduce(|x, y| x + y)
            .unwrap_or(T::zero())
    }

    pub fn nonzero(&self) -> Vec<usize> {
        nonzero(&self.values)
    }

    pub fn prod(&self) -> T {
        prod(&self.values)
    }
    pub fn cumsum(&self) -> Vec<T>
    where
        T: AddAssign,
    {
        cum_sum(&self.values)
    }

    pub fn sort(&mut self) -> Vec<usize>
    where
        T: PartialOrd,
    {
        self.sort_order(false)
    }

    pub fn sort_order(&mut self, reverse: bool) -> Vec<usize>
    where
        T: PartialOrd,
    {
        let ix = argsort(&self.values, reverse);
        let values0 = self.values.clone();
        for (i, &j) in ix.iter().enumerate() {
            self.values[i] = values0[j];
        }
        ix
    }

    /// Returns an array where the values are `T::ln(a[i])`.
    pub fn ln(&self) -> Arr<T>
    where
        T: Ln,
    {
        Arr {
            values: self.values.iter().map(|x| x.ln()).collect(),
        }
    }

    /// Returns an array where the values are `T::exp(a[i])`.
    pub fn exp(&self) -> Arr<T>
    where
        T: Exp,
    {
        Arr {
            values: self.values.iter().map(|x| x.exp()).collect(),
        }
    }

    /// Returns an array where the values are `T::abs(a[i])`.
    pub fn abs(&self) -> Arr<T>
    where
        T: Abs,
    {
        Arr {
            values: self.values.iter().map(|x| x.abs()).collect(),
        }
    }

    /// Returns an array where the values are `T::pow(a[i], e)`.
    pub fn pow<R>(&self, e: R) -> Arr<T>
    where
        T: Pow<R, Output = T>,
        R: Copy,
    {
        Arr {
            values: self.values.iter().map(|x| x.pow(e)).collect(),
        }
    }

    /// Returns an array where the values are rounded.
    pub fn round(&self) -> Arr<T>
    where
        T: Round,
    {
        Arr {
            values: self.values.iter().map(|x| x.round()).collect(),
        }
    }

    pub fn round_mut(&mut self)
    where
        T: Round,
    {
        for v in self.values.iter_mut() {
            *v = v.round();
        }
    }

    /// Returns the maximum value of `a`.
    pub fn max(&self) -> T
    where
        T: Bounded + PartialOrd + Copy,
    {
        max(&self.values)
    }

    /// Returns the minimum value of `a`.
    pub fn min(&self) -> T
    where
        T: Bounded + PartialOrd + Copy,
    {
        min(&self.values)
    }

    /// Returns the index of the maximum value of `a`.
    pub fn argmax(&self) -> usize
    where
        T: Bounded + PartialOrd,
    {
        argmax(&self.values)
    }

    /// Returns the mean of all element values.
    pub fn mean(&self) -> T
    where
        T: Zero + Copy + AddAssign + Div<Output = T> + FromPrimitive,
    {
        mean(&self.values)
    }

    /// Returns the standard deviation.
    pub fn std(&self) -> T
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
        std(&self.values)
    }

    /// Returns an array with the discrete difference of `a`.
    /// Length of result is 1 less than length of `a`.
    pub fn diff(&self) -> Arr<T>
    where
        T: Zero + Copy + Sub<Output = T>,
    {
        Arr {
            values: diff(&self.values),
        }
    }

    /// Returns the 2-norm (Euclidean).
    pub fn norm2(&self) -> T
    where
        T: Zero + Copy + Sqrt + AddAssign,
    {
        norm(&self.values)
    }

    /// Returns an integer array with 1s where `T::is_nan(self[i])`.
    pub fn is_nan(&self) -> Vec<usize>
    where
        T: IsNaN,
    {
        is_nan(&self.values)
    }

    pub fn sqrt(&self) -> Arr<T>
    where
        T: Sqrt,
    {
        Arr {
            values: self.values.iter().map(|v| T::sqrt(v)).collect(),
        }
    }

    pub fn sin(&self) -> Arr<T>
    where
        T: Sin,
    {
        Arr {
            values: self.values.iter().map(|v| T::sin(v)).collect(),
        }
    }

    pub fn cos(&self) -> Arr<T>
    where
        T: Cos,
    {
        Arr {
            values: self.values.iter().map(|v| T::cos(v)).collect(),
        }
    }

    pub fn asin(&self) -> Arr<T>
    where
        T: ArcSin,
    {
        Arr {
            values: self.values.iter().map(|v| T::asin(v)).collect(),
        }
    }

    pub fn acos(&self) -> Arr<T>
    where
        T: ArcCos,
    {
        Arr {
            values: self.values.iter().map(|v| T::acos(v)).collect(),
        }
    }

    pub fn to_string_col(&self) -> String
    where
        T: Display,
    {
        self.values
            .iter()
            .map(|v| format!("{}", v).to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl<T> Deref for Arr<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl<T> DerefMut for Arr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values
    }
}

impl<T> Display for Arr<T>
where
    T: Display + Copy,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", to_string(1, self.len(), &self.values, false))
    }
}

#[macro_export]
macro_rules! arr {
    () => (
        $crate::Arr::new()
    );
    ($elem:expr; $n:expr) => (
        $crate::Arr::with_value( $n, $elem)
    );
    ($($x:expr),+ $(,)?) => (
        $crate::Arr::with_vec(vec![$($x),+])
    );
}
