use crate::matrix::Matrix;
use std::ops::*;

impl<T> Neg for Matrix<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            rows: self.rows,
            cols: self.cols,
            // data: self.into_iter().map(|a| -a).collect(),
            data: self.data.into_iter().map(|a| -a).collect(),
        }
    }
}

// Add/Sub/Mul/Div <T> //

impl<T> Add<T> for Matrix<T>
where
    T: Add<T, Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Self::Output {
            rows: self.rows,
            cols: self.cols,
            data: self.data.into_iter().map(|a| a + rhs).collect(),
        }
    }
}

impl<T> Sub<T> for Matrix<T>
where
    T: Sub<T, Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Self::Output {
            rows: self.rows,
            cols: self.cols,
            data: self.data.into_iter().map(|a| a - rhs).collect(),
        }
    }
}

impl<T> Mul<T> for Matrix<T>
where
    T: Mul<T, Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::Output {
            rows: self.rows,
            cols: self.cols,
            data: self.data.into_iter().map(|a| a * rhs).collect(),
        }
    }
}

impl<T> Div<T> for Matrix<T>
where
    T: Div<T, Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self::Output {
            rows: self.rows,
            cols: self.cols,
            data: self.data.into_iter().map(|a| a / rhs).collect(),
        }
    }
}

// Add/Sub/Mul/Div Assign<T> //

impl<T> AddAssign<T> for Matrix<T>
where
    T: AddAssign<T> + Copy,
{
    fn add_assign(&mut self, rhs: T) {
        self.data.iter_mut().for_each(|a| *a += rhs);
    }
}

impl<T> SubAssign<T> for Matrix<T>
where
    T: SubAssign<T> + Copy,
{
    fn sub_assign(&mut self, rhs: T) {
        self.data.iter_mut().for_each(|a| *a -= rhs);
    }
}

impl<T> MulAssign<T> for Matrix<T>
where
    T: MulAssign<T> + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        self.data.iter_mut().for_each(|a| *a *= rhs);
    }
}

impl<T> DivAssign<T> for Matrix<T>
where
    T: DivAssign<T> + Copy,
{
    fn div_assign(&mut self, rhs: T) {
        self.data.iter_mut().for_each(|a| *a /= rhs);
    }
}

// Add/Sub/Mul/Div Matrix<T> //

impl<T> Add<Matrix<T>> for Matrix<T>
where
    T: Add<T, Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.rows, rhs.rows);
        assert_eq!(self.cols, rhs.cols);

        Self::Output {
            rows: self.rows,
            cols: self.cols,
            data: self
                .data
                .into_iter()
                .zip(rhs.data.into_iter())
                .map(|(a, b)| a + b)
                .collect(),
        }
    }
}

impl<T> Sub<Matrix<T>> for Matrix<T>
where
    T: Sub<T, Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.rows, rhs.rows);
        assert_eq!(self.cols, rhs.cols);

        Self::Output {
            rows: self.rows,
            cols: self.cols,
            data: self
                .data
                .into_iter()
                .zip(rhs.data.into_iter())
                .map(|(a, b)| a - b)
                .collect(),
        }
    }
}

impl<T> Mul<Matrix<T>> for Matrix<T>
where
    T: Mul<T, Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.rows, rhs.rows);
        assert_eq!(self.cols, rhs.cols);

        Self::Output {
            rows: self.rows,
            cols: self.cols,
            data: self
                .data
                .into_iter()
                .zip(rhs.data.into_iter())
                .map(|(a, b)| a * b)
                .collect(),
        }
    }
}

impl<T> Div<Matrix<T>> for Matrix<T>
where
    T: Div<T, Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        assert_eq!(self.rows, rhs.rows);
        assert_eq!(self.cols, rhs.cols);

        Self::Output {
            rows: self.rows,
            cols: self.cols,
            data: self
                .data
                .into_iter()
                .zip(rhs.data.into_iter())
                .map(|(a, b)| a / b)
                .collect(),
        }
    }
}

// Add/Sub/Mul/Div Assign Matrix<T> //

impl<T> AddAssign<Matrix<T>> for Matrix<T>
where
    T: AddAssign<T> + Copy,
{
    fn add_assign(&mut self, rhs: Matrix<T>) {
        assert_eq!(self.rows, rhs.rows);
        assert_eq!(self.cols, rhs.cols);

        self.data
            .iter_mut()
            .zip(rhs.data.into_iter())
            .for_each(|(a, b)| *a += b);
    }
}

impl<T> SubAssign<Matrix<T>> for Matrix<T>
where
    T: SubAssign<T> + Copy,
{
    fn sub_assign(&mut self, rhs: Matrix<T>) {
        assert_eq!(self.rows, rhs.rows);
        assert_eq!(self.cols, rhs.cols);

        self.data
            .iter_mut()
            .zip(rhs.data.into_iter())
            .for_each(|(a, b)| *a -= b);
    }
}

impl<T> MulAssign<Matrix<T>> for Matrix<T>
where
    T: MulAssign<T> + Copy,
{
    fn mul_assign(&mut self, rhs: Matrix<T>) {
        assert_eq!(self.rows, rhs.rows);
        assert_eq!(self.cols, rhs.cols);

        self.data
            .iter_mut()
            .zip(rhs.data.into_iter())
            .for_each(|(a, b)| *a *= b);
    }
}

impl<T> DivAssign<Matrix<T>> for Matrix<T>
where
    T: DivAssign<T> + Copy,
{
    fn div_assign(&mut self, rhs: Matrix<T>) {
        assert_eq!(self.rows, rhs.rows);
        assert_eq!(self.cols, rhs.cols);

        self.data
            .iter_mut()
            .zip(rhs.data.into_iter())
            .for_each(|(a, b)| *a /= b);
    }
}
