use crate::full::{get_ref, get_ref_mut, identity, ix, mat_mat, mat_vec, ones, to_string, zeros};

use num_traits::{One, Zero};
use std::fmt::{Display, Formatter};
use std::iter::zip;
use std::ops::{Add, AddAssign, Index, IndexMut, Mul};

#[derive(Clone)]
pub struct Mat<T> {
    pub(crate) rows: usize,
    pub(crate) cols: usize,
    // Matrix element values stored in row-major order (C-style)
    // or column-major (Fortran style) order.
    pub(crate) values: Vec<T>,
    pub(crate) col_major: bool,
}

impl<T> Mat<T> {
    pub fn new(rows: usize, cols: usize, values: Vec<T>, col_major: bool) -> Self {
        Self {
            rows,
            cols,
            values,
            col_major,
        }
    }

    pub fn from_fn(
        rows: usize,
        cols: usize,
        f: impl Fn(usize, usize) -> T,
        col_major: bool,
    ) -> Self {
        let values = zip(0..rows, 0..cols).map(|(r, c)| f(r, c)).collect();
        Self {
            rows,
            cols,
            values,
            col_major,
        }
    }

    pub fn zeros(rows: usize, cols: usize, col_major: bool) -> Self
    where
        T: Clone + Zero,
    {
        Self {
            rows,
            cols,
            values: zeros(rows, cols),
            col_major,
        }
    }

    pub fn ones(rows: usize, cols: usize, col_major: bool) -> Self
    where
        T: Clone + One,
    {
        Self {
            rows,
            cols,
            values: ones(rows, cols),
            col_major,
        }
    }

    pub fn identity(n: usize, col_major: bool) -> Self
    where
        T: Clone + Zero + One,
    {
        Self {
            rows: n,
            cols: n,
            values: identity(n),
            col_major,
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

    pub fn values(&self) -> &[T] {
        &self.values
    }

    pub fn values_mut(&mut self) -> &mut [T] {
        &mut self.values
    }

    pub fn col_major(&self) -> bool {
        self.col_major
    }

    pub fn get_ref(&self, row: usize, col: usize) -> &T {
        assert!(row < self.rows);
        assert!(col < self.cols);
        get_ref(self.rows, self.cols, &self.values, row, col, self.col_major)
    }

    pub fn get_ref_mut(&mut self, row: usize, col: usize) -> &mut T {
        assert!(row < self.rows);
        assert!(col < self.cols);
        get_ref_mut(
            self.rows,
            self.cols,
            &mut self.values,
            row,
            col,
            self.col_major,
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

    // pub fn row_mut(&mut self, row: usize) -> impl Iterator<Item=&mut T> {
    //     assert!(row < self.rows);
    //     (0..self.cols).map(|col| self.get_ref_mut(row, col))
    // }
    //
    // pub fn col_mut(&mut self, col: usize) -> impl Iterator<Item=&mut T> {
    //     assert!(col < self.cols);
    //     (0..self.rows).map(move |row| self.get_ref_mut(row, col))
    // }

    /// Panics if not in row-major order.
    pub fn row_slice(&self, row: usize) -> &[T] {
        assert!(!self.col_major);
        assert!(row < self.rows);
        let i = ix(self.rows, self.cols, row, 0, self.col_major);
        let j = ix(self.rows, self.cols, row + 1, 0, self.col_major);
        &self.values[i..j]
    }

    /// Panics if not in column-major order.
    pub fn col_slice(&self, col: usize) -> &[T] {
        assert!(self.col_major);
        assert!(col < self.cols);
        let i = ix(self.rows, self.cols, 0, col, self.col_major);
        let j = ix(self.rows, self.cols, 0, col + 1, self.col_major);
        &self.values[i..j]
    }

    /// Panics if not in row-major order.
    pub fn row_slice_mut(&mut self, row: usize) -> &mut [T] {
        assert!(!self.col_major);
        assert!(row < self.rows);
        let i = ix(self.rows, self.cols, row, 0, self.col_major);
        let j = ix(self.rows, self.cols, row + 1, 0, self.col_major);
        &mut self.values[i..j]
    }

    /// Panics if not in column-major order.
    pub fn col_slice_mut(&mut self, col: usize) -> &mut [T] {
        assert!(self.col_major);
        assert!(col < self.cols);
        let i = ix(self.rows, self.cols, 0, col, self.col_major);
        let j = ix(self.rows, self.cols, 0, col + 1, self.col_major);
        &mut self.values[i..j]
    }

    /// Panics if not in row-major order.
    pub fn row_iter(&self) -> impl Iterator<Item = &'_ [T]> {
        assert!(!self.col_major);
        assert_eq!(self.values.len() % self.cols, 0);
        self.values.chunks(self.cols)
    }

    /// Panics if not in column-major order.
    pub fn col_iter(&self) -> impl Iterator<Item = &'_ [T]> {
        assert!(self.col_major);
        assert_eq!(self.values.len() % self.rows, 0);
        self.values.chunks(self.rows)
    }

    pub fn select_rows(&self, rows: &[usize]) -> Self
    where
        T: Clone,
    {
        let mut data = vec![];
        for &r in rows {
            let i = ix(self.rows, self.cols, r, 0, self.col_major);
            let j = ix(self.rows, self.cols, r, self.cols - 1, self.col_major);
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
        mat_vec(self.rows, self.cols, &self.values, b, self.col_major)
    }

    pub fn mat_mat(&self, b: &Self) -> Self
    where
        T: Mul<Output = T> + AddAssign + Zero + Copy,
    {
        Self {
            rows: self.rows,
            cols: b.cols,
            values: mat_mat(
                self.rows,
                self.cols,
                &self.values,
                b.rows,
                b.cols,
                &b.values,
                self.col_major,
            ),
            col_major: self.col_major,
        }
    }
}

impl<T> Index<(usize, usize)> for Mat<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (r, c) = index;
        self.get_ref(r, c)
    }
}

impl<T> IndexMut<(usize, usize)> for Mat<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (r, c) = index;
        self.get_ref_mut(r, c)
    }
}

impl<T> Display for Mat<T>
where
    T: Copy + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            to_string(self.rows, self.cols, &self.values, self.col_major)
        )
    }
}
