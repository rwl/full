use crate::densetools::{ones, to_string, zeros};
use crate::slice::{all, any, arange, find, range, select};
use num_traits::{FromPrimitive, One, ToPrimitive, Zero};
use std::fmt::{Display, Formatter};
use std::ops::{AddAssign, Deref, DerefMut, Div, Sub};

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

    pub fn concat(a: &[T], b: &[T]) -> Self {
        Self {
            data: [a, b].concat(),
        }
    }

    pub fn data(&self) -> &[T] {
        &self.data
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
