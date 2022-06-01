use num_traits::{One, Zero};
use std::fmt::Display;

#[inline]
pub fn ix(n_row: usize, n_col: usize, row: usize, col: usize, row_major: bool) -> usize {
    if row_major {
        row * n_col + col
    } else {
        col * n_row + row
    }
}

#[inline]
pub fn get<T>(n_row: usize, n_col: usize, a_x: &[T], row: usize, col: usize, row_major: bool) -> T
where
    T: Copy,
{
    a_x[ix(n_row, n_col, row, col, row_major)]
}

#[inline]
pub fn set<T>(
    n_row: usize,
    n_col: usize,
    a_x: &mut [T],
    row: usize,
    col: usize,
    v: T,
    row_major: bool,
) {
    a_x[ix(n_row, n_col, row, col, row_major)] = v
}

pub fn zeros<T>(n_row: usize, n_col: usize) -> Vec<T>
where
    T: Clone + Zero,
{
    vec![T::zero(); n_row * n_col]
}

pub fn ones<T>(n_row: usize, n_col: usize) -> Vec<T>
where
    T: Clone + One,
{
    vec![T::one(); n_row * n_col]
}

pub fn identity<T>(n: usize) -> Vec<T>
where
    T: Clone + Zero + One,
{
    let mut a_x = vec![T::zero(); n * n];
    for i in 0..n {
        set(n, n, &mut a_x, i, i, T::one(), true);
    }
    a_x
}

pub fn to_string<T>(n_row: usize, n_col: usize, a_x: &[T], row_major: bool) -> String
where
    T: Copy + Display,
{
    use std::io::Write;

    let mut w = Vec::<u8>::new();
    for i in 0..n_row {
        for j in 0..n_col {
            if j != 0 {
                w.write(b" ").unwrap();
            }
            write!(w, "{}", get(n_row, n_col, a_x, i, j, row_major)).unwrap();
        }
        if i != n_row - 1 {
            writeln!(w).unwrap();
        }
    }
    String::from_utf8(w).unwrap()
}
