use crate::densetools::{identity, mat_mat, mat_vec, ones, to_string, zeros};
use num_traits::{One, Zero};
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Mul};

pub struct Matrix<T> {
    pub(crate) rows: usize,
    pub(crate) cols: usize,
    // Matrix element values stored in row-major order (C-style).
    pub(crate) data: Vec<T>,
}

impl<T> Matrix<T> {
    pub fn zeros(rows: usize, cols: usize) -> Self
    where
        T: Clone + Zero,
    {
        Self {
            rows,
            cols,
            data: zeros(rows, cols),
        }
    }

    pub fn ones(rows: usize, cols: usize) -> Self
    where
        T: Clone + One,
    {
        Self {
            rows,
            cols,
            data: ones(rows, cols),
        }
    }

    pub fn identity(n: usize) -> Self
    where
        T: Clone + Zero + One,
    {
        Self {
            rows: n,
            cols: n,
            data: identity(n),
        }
    }

    pub fn mat_vec(&self, b: &[T]) -> Vec<T>
    where
        T: Mul<Output = T> + Add<Output = T> + Zero + Copy,
    {
        mat_vec(self.rows, self.cols, &self.data, b, true)
    }

    pub fn mat_mat(&self, b: &Self) -> Self
    where
        T: Mul<Output = T> + AddAssign + Zero + Copy,
    {
        Self {
            rows: self.rows,
            cols: b.cols,
            data: mat_mat(
                self.rows, self.cols, &self.data, b.rows, b.cols, &b.data, true,
            ),
        }
    }
}

impl<T> Display for Matrix<T>
where
    T: Copy + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", to_string(self.rows, self.cols, &self.data, true))
    }
}
