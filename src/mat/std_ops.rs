use crate::mat::Mat;
use std::ops::*;

impl<T> Neg for Mat<T>
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
            col_major: self.col_major,
        }
    }
}

// Add/Sub/Mul/Div <T> //

impl<T> Add<T> for Mat<T>
where
    T: Add<T, Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Self::Output {
            rows: self.rows,
            cols: self.cols,
            data: self.data.into_iter().map(|a| a + rhs).collect(),
            col_major: self.col_major,
        }
    }
}

impl<T> Sub<T> for Mat<T>
where
    T: Sub<T, Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Self::Output {
            rows: self.rows,
            cols: self.cols,
            data: self.data.into_iter().map(|a| a - rhs).collect(),
            col_major: self.col_major,
        }
    }
}

impl<T> Mul<T> for Mat<T>
where
    T: Mul<T, Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::Output {
            rows: self.rows,
            cols: self.cols,
            data: self.data.into_iter().map(|a| a * rhs).collect(),
            col_major: self.col_major,
        }
    }
}

impl<T> Div<T> for Mat<T>
where
    T: Div<T, Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self::Output {
            rows: self.rows,
            cols: self.cols,
            data: self.data.into_iter().map(|a| a / rhs).collect(),
            col_major: self.col_major,
        }
    }
}

// Add/Sub/Mul/Div Assign<T> //

impl<T> AddAssign<T> for Mat<T>
where
    T: AddAssign<T> + Copy,
{
    fn add_assign(&mut self, rhs: T) {
        self.data.iter_mut().for_each(|a| *a += rhs);
    }
}

impl<T> SubAssign<T> for Mat<T>
where
    T: SubAssign<T> + Copy,
{
    fn sub_assign(&mut self, rhs: T) {
        self.data.iter_mut().for_each(|a| *a -= rhs);
    }
}

impl<T> MulAssign<T> for Mat<T>
where
    T: MulAssign<T> + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        self.data.iter_mut().for_each(|a| *a *= rhs);
    }
}

impl<T> DivAssign<T> for Mat<T>
where
    T: DivAssign<T> + Copy,
{
    fn div_assign(&mut self, rhs: T) {
        self.data.iter_mut().for_each(|a| *a /= rhs);
    }
}

// Add/Sub/Mul/Div Mat<T> //

impl<T> Add<Mat<T>> for Mat<T>
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
            col_major: self.col_major,
        }
    }
}

impl<T> Sub<Mat<T>> for Mat<T>
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
            col_major: self.col_major,
        }
    }
}

impl<T> Mul<Mat<T>> for Mat<T>
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
            col_major: self.col_major,
        }
    }
}

impl<T> Div<Mat<T>> for Mat<T>
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
            col_major: self.col_major,
        }
    }
}

// Add/Sub/Mul/Div Assign Mat<T> //

impl<T> AddAssign<Mat<T>> for Mat<T>
where
    T: AddAssign<T> + Copy,
{
    fn add_assign(&mut self, rhs: Mat<T>) {
        assert_eq!(self.rows, rhs.rows);
        assert_eq!(self.cols, rhs.cols);

        self.data
            .iter_mut()
            .zip(rhs.data.into_iter())
            .for_each(|(a, b)| *a += b);
    }
}

impl<T> SubAssign<Mat<T>> for Mat<T>
where
    T: SubAssign<T> + Copy,
{
    fn sub_assign(&mut self, rhs: Mat<T>) {
        assert_eq!(self.rows, rhs.rows);
        assert_eq!(self.cols, rhs.cols);

        self.data
            .iter_mut()
            .zip(rhs.data.into_iter())
            .for_each(|(a, b)| *a -= b);
    }
}

impl<T> MulAssign<Mat<T>> for Mat<T>
where
    T: MulAssign<T> + Copy,
{
    fn mul_assign(&mut self, rhs: Mat<T>) {
        assert_eq!(self.rows, rhs.rows);
        assert_eq!(self.cols, rhs.cols);

        self.data
            .iter_mut()
            .zip(rhs.data.into_iter())
            .for_each(|(a, b)| *a *= b);
    }
}

impl<T> DivAssign<Mat<T>> for Mat<T>
where
    T: DivAssign<T> + Copy,
{
    fn div_assign(&mut self, rhs: Mat<T>) {
        assert_eq!(self.rows, rhs.rows);
        assert_eq!(self.cols, rhs.cols);

        self.data
            .iter_mut()
            .zip(rhs.data.into_iter())
            .for_each(|(a, b)| *a /= b);
    }
}
