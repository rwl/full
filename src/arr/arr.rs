use crate::densetools::{ones, to_string, zeros};
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
    pub(crate) data: Vec<T>,
}

impl<T> Arr<T>
where
    T: Clone + Copy + Zero + One,
{
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn with_vec(data: Vec<T>) -> Self {
        Self { data }
    }

    pub fn with_value(n: usize, v: T) -> Self {
        Self { data: vec![v; n] }
    }

    pub fn with_capacity(n: usize) -> Self {
        Self {
            data: Vec::with_capacity(n),
        }
    }

    pub fn zeros(n: usize) -> Self {
        Self { data: zeros(1, n) }
    }

    pub fn ones(n: usize) -> Self {
        Self { data: ones(1, n) }
    }

    pub fn range(stop: usize) -> Self
    where
        T: FromPrimitive,
    {
        Self { data: range(stop) }
    }
    pub fn arange(start: T, stop: T, step: T) -> Self
    where
        T: Sub<Output = T> + Div<Output = T> + ToPrimitive + PartialOrd + AddAssign,
    {
        Self {
            data: arange(start, stop, step),
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
            data: linspace(start, stop, num, inclusive),
        }
    }

    pub fn concat(a: &[&[T]]) -> Self {
        Self { data: a.concat() }
    }

    pub fn data(&self) -> &[T] {
        &self.data
    }

    pub fn vec(self) -> Vec<T> {
        self.data
    }

    pub fn find<U>(&self) -> Vec<U>
    where
        U: FromPrimitive,
    {
        find(&self.data)
    }
    pub fn any(&self) -> bool {
        any(&self.data)
    }
    pub fn all(&self) -> bool {
        all(&self.data)
    }

    pub fn select(&self, ix: &[usize]) -> Arr<T> {
        Arr {
            data: select(&self.data, ix),
        }
    }

    pub fn set(&mut self, ix: &[usize], v: &[T]) {
        assert_eq!(ix.len(), v.len());
        set_slice(&mut self.data, ix, v);
    }

    pub fn set_all(&mut self, ix: &[usize], v: T) {
        set_all(&mut self.data, ix, v);
    }

    pub fn sum(&self) -> T {
        self.data
            .iter()
            .map(|&x| x)
            .reduce(|x, y| x + y)
            .unwrap_or(T::zero())
    }

    pub fn nonzero(&self) -> Vec<usize> {
        nonzero(&self.data)
    }

    pub fn prod(&self) -> T {
        prod(&self.data)
    }
    pub fn cumsum(&self) -> Vec<T>
    where
        T: AddAssign,
    {
        cum_sum(&self.data)
    }

    pub fn sort(&mut self) -> Vec<usize>
    where
        T: PartialOrd,
    {
        argsort(&mut self.data, false)
    }

    pub fn sort_order(&mut self, reverse: bool) -> Vec<usize>
    where
        T: PartialOrd,
    {
        argsort(&mut self.data, reverse)
    }

    /// Returns an array where the values are `T::ln(a[i])`.
    pub fn ln(&self) -> Arr<T>
    where
        T: Ln,
    {
        Arr {
            data: self.data.iter().map(|x| x.ln()).collect(),
        }
    }

    /// Returns an array where the values are `T::exp(a[i])`.
    pub fn exp(&self) -> Arr<T>
    where
        T: Exp,
    {
        Arr {
            data: self.data.iter().map(|x| x.exp()).collect(),
        }
    }

    /// Returns an array where the values are `T::abs(a[i])`.
    pub fn abs(&self) -> Arr<T>
    where
        T: Abs,
    {
        Arr {
            data: self.data.iter().map(|x| x.abs()).collect(),
        }
    }

    /// Returns an array where the values are `T::pow(a[i], e)`.
    pub fn pow<R>(&self, e: R) -> Arr<T>
    where
        T: Pow<R, Output = T>,
        R: Copy,
    {
        Arr {
            data: self.data.iter().map(|x| x.pow(e)).collect(),
        }
    }

    /// Returns an array where the values are rounded.
    pub fn round(&self) -> Arr<T>
    where
        T: Round,
    {
        Arr {
            data: self.data.iter().map(|x| x.round()).collect(),
        }
    }

    pub fn round_mut(&mut self)
    where
        T: Round,
    {
        for v in self.data.iter_mut() {
            *v = v.round();
        }
    }

    /// Returns the maximum value of `a`.
    pub fn max(&self) -> T
    where
        T: Bounded + PartialOrd + Copy,
    {
        max(&self.data)
    }

    /// Returns the minimum value of `a`.
    pub fn min(&self) -> T
    where
        T: Bounded + PartialOrd + Copy,
    {
        min(&self.data)
    }

    /// Returns the index of the maximum value of `a`.
    pub fn argmax(&self) -> usize
    where
        T: Bounded + PartialOrd,
    {
        argmax(&self.data)
    }

    /// Returns the mean of all element values.
    pub fn mean(&self) -> T
    where
        T: Zero + Copy + AddAssign + Div<Output = T> + FromPrimitive,
    {
        mean(&self.data)
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
        std(&self.data)
    }

    /// Returns an array with the discrete difference of `a`.
    /// Length of result is 1 less than length of `a`.
    pub fn diff(&self) -> Arr<T>
    where
        T: Zero + Copy + Sub<Output = T>,
    {
        Arr {
            data: diff(&self.data),
        }
    }

    /// Returns the 2-norm (Euclidean).
    pub fn norm2(&self) -> T
    where
        T: Zero + Copy + Sqrt + AddAssign,
    {
        norm(&self.data)
    }

    /// Returns an integer array with 1s where `T::is_nan(self[i])`.
    pub fn is_nan(&self) -> Vec<usize>
    where
        T: IsNaN,
    {
        is_nan(&self.data)
    }

    pub fn sqrt(&self) -> Arr<T>
    where
        T: Sqrt,
    {
        Arr {
            data: self.data.iter().map(|v| T::sqrt(v)).collect(),
        }
    }

    pub fn sin(&self) -> Arr<T>
    where
        T: Sin,
    {
        Arr {
            data: self.data.iter().map(|v| T::sin(v)).collect(),
        }
    }

    pub fn cos(&self) -> Arr<T>
    where
        T: Cos,
    {
        Arr {
            data: self.data.iter().map(|v| T::cos(v)).collect(),
        }
    }

    pub fn asin(&self) -> Arr<T>
    where
        T: ArcSin,
    {
        Arr {
            data: self.data.iter().map(|v| T::asin(v)).collect(),
        }
    }

    pub fn acos(&self) -> Arr<T>
    where
        T: ArcCos,
    {
        Arr {
            data: self.data.iter().map(|v| T::acos(v)).collect(),
        }
    }

    pub fn to_string_col(&self) -> String
    where
        T: Display,
    {
        self.data
            .iter()
            .map(|v| format!("{}", v).to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl<T> Deref for Arr<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for Arr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T> Display for Arr<T>
where
    T: Display + Copy,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", to_string(1, self.len(), &self.data, true))
    }
}
