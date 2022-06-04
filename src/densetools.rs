use crate::slice::dot_product;
use num_traits::{One, Zero};
use std::fmt::Display;
use std::ops::{Add, AddAssign, Mul};

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

/// Performs matrix-vector multiplication.
pub fn mat_vec<T>(n_row: usize, n_col: usize, a_x: &[T], b: &[T], row_major: bool) -> Vec<T>
where
    T: Mul<Output = T> + Add<Output = T> + Zero + Copy,
{
    assert_eq!(b.len(), n_col);

    let mut y = Vec::with_capacity(b.len());
    for i in 0..n_row {
        y.push(dot_product(&a_x[ix(n_row, n_col, i, 0, row_major)..], b))
    }
    y
}

/// Performs matrix-matrix multiplication.
/// `b_row` must equal `a_col`.
/// Returns an `a_row*b_col` vector.
pub fn mat_mat<T>(
    a_row: usize,
    a_col: usize,
    a_x: &[T],
    b_row: usize,
    b_col: usize,
    b_x: &[T],
    row_major: bool,
) -> Vec<T>
where
    T: Mul<Output = T> + AddAssign + Zero + Copy,
{
    assert_eq!(
        a_col, b_row,
        "rows of b {} must equal columns of a {}",
        b_row, a_col
    );

    let mut c = zeros(a_row, b_col);

    for i in 0..a_row {
        for j in 0..b_col {
            let c_ij = ix(a_row, b_col, i, j, row_major);
            for k in 0..a_col {
                let ax_ik = get(a_row, a_col, a_x, i, k, row_major);
                let bx_kj = get(b_row, b_col, b_x, k, j, row_major);
                c[c_ij] += ax_ik * bx_kj;
            }
        }
    }
    c
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
