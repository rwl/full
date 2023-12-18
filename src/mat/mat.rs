use crate::full::{get_ref, identity, ix, mat_mat, mat_vec, ones, to_string, zeros};

use num_traits::{One, Zero};
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Mul};

pub struct Mat<T> {
    pub(crate) rows: usize,
    pub(crate) cols: usize,
    // Matrix element values stored in row-major order (C-style)
    // or column-major (Fortran style) order.
    pub(crate) values: Vec<T>,
    pub(crate) col_major: bool,
}

impl<T> Mat<T> {
    pub fn new(rows: usize, cols: usize, values: Vec<T>) -> Self {
        Self {
            rows,
            cols,
            values,
            col_major: false,
        }
    }

    pub fn zeros(rows: usize, cols: usize) -> Self
    where
        T: Clone + Zero,
    {
        Self {
            rows,
            cols,
            values: zeros(rows, cols),
            col_major: false,
        }
    }

    pub fn ones(rows: usize, cols: usize) -> Self
    where
        T: Clone + One,
    {
        Self {
            rows,
            cols,
            values: ones(rows, cols),
            col_major: false,
        }
    }

    pub fn identity(n: usize) -> Self
    where
        T: Clone + Zero + One,
    {
        Self {
            rows: n,
            cols: n,
            values: identity(n),
            col_major: false,
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }
    pub fn cols(&self) -> usize {
        self.cols
    }
    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn values(&self) -> &Vec<T> {
        &self.values
    }

    pub fn values_mut(&mut self) -> &mut Vec<T> {
        &mut self.values
    }

    pub fn get_ref(&self, row: usize, col: usize) -> &T {
        assert!(row < self.rows);
        assert!(col < self.cols);
        get_ref(
            self.rows,
            self.cols,
            self.values.as_slice(),
            row,
            col,
            !self.col_major,
        )
    }

    pub fn row(&self, row: usize) -> impl Iterator<Item = &T> {
        assert!(row < self.rows);
        (0..self.cols).map(move |col| self.get_ref(row, col))
    }

    pub fn col(&self, col: usize) -> impl Iterator<Item = &T> {
        assert!(col < self.cols);
        (0..self.rows).map(move |row| self.get_ref(row, col))
    }

    pub fn select_rows(&self, rows: &[usize]) -> Self
    where
        T: Clone,
    {
        let mut data = vec![];
        for &r in rows {
            let i = ix(self.rows, self.cols, r, 0, !self.col_major);
            let j = ix(self.rows, self.cols, r, self.cols - 1, !self.col_major);
            data.extend(self.values[i..=j].to_vec())
        }
        Self {
            rows: rows.len(),
            cols: self.cols,
            values: data,
            col_major: false,
        }
    }

    // pub fn select(&self, rows: Option<&[usize]>, cols: Option<&[usize]>) -> Self {
    //     if self.col_major {
    //     } else {
    //     }
    //     Self {
    //         rows: 0,
    //         cols: 0,
    //         data: vec![],
    //         col_major: false,
    //     }
    // }

    pub fn mat_vec(&self, b: &[T]) -> Vec<T>
    where
        T: Mul<Output = T> + Add<Output = T> + Zero + Copy,
    {
        mat_vec(self.rows, self.cols, &self.values, b, true)
    }

    pub fn mat_mat(&self, b: &Self) -> Self
    where
        T: Mul<Output = T> + AddAssign + Zero + Copy,
    {
        Self {
            rows: self.rows,
            cols: b.cols,
            values: mat_mat(
                self.rows, self.cols, &self.values, b.rows, b.cols, &b.values, true,
            ),
            col_major: false,
        }
    }
}

impl<T> Display for Mat<T>
where
    T: Copy + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", to_string(self.rows, self.cols, &self.values, true))
    }
}
