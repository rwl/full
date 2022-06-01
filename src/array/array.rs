use crate::densetools::{ones, to_string, zeros};
use num_traits::{One, Zero};
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct Array<T> {
    pub(crate) data: Vec<T>,
}

impl<T> Array<T> {
    pub fn zeros(n: usize) -> Self
    where
        T: Clone + Zero,
    {
        Self { data: zeros(1, n) }
    }

    pub fn ones(n: usize) -> Self
    where
        T: Clone + One,
    {
        Self { data: ones(1, n) }
    }

    pub fn data(&self) -> &[T]
    where
        T: Clone,
    {
        &self.data
    }
}

impl<T> Deref for Array<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for Array<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T> Display for Array<T>
where
    T: Display + Copy,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", to_string(1, self.len(), &self.data, true))
    }
}
