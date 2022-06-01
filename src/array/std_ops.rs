use crate::array::Array;
use std::ops::*;

impl<T> Neg for Array<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            data: self.data.into_iter().map(|a| -a).collect(),
        }
    }
}

// Add/Sub/Mul/Div <T> //

impl<T> Add<T> for Array<T>
where
    T: Add<T, Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Self::Output {
            data: self.data.into_iter().map(|a| a + rhs).collect(),
        }
    }
}

impl<T> Sub<T> for Array<T>
where
    T: Sub<T, Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Self::Output {
            data: self.data.into_iter().map(|a| a - rhs).collect(),
        }
    }
}

impl<T> Mul<T> for Array<T>
where
    T: Mul<T, Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::Output {
            data: self.data.into_iter().map(|a| a * rhs).collect(),
        }
    }
}

impl<T> Div<T> for Array<T>
where
    T: Div<T, Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self::Output {
            data: self.data.into_iter().map(|a| a / rhs).collect(),
        }
    }
}

// Add/Sub/Mul/Div Assign<T> //

impl<T> AddAssign<T> for Array<T>
where
    T: AddAssign<T> + Copy,
{
    fn add_assign(&mut self, rhs: T) {
        self.data.iter_mut().for_each(|a| *a += rhs);
    }
}

impl<T> SubAssign<T> for Array<T>
where
    T: SubAssign<T> + Copy,
{
    fn sub_assign(&mut self, rhs: T) {
        self.data.iter_mut().for_each(|a| *a -= rhs);
    }
}

impl<T> MulAssign<T> for Array<T>
where
    T: MulAssign<T> + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        self.data.iter_mut().for_each(|a| *a *= rhs);
    }
}

impl<T> DivAssign<T> for Array<T>
where
    T: DivAssign<T> + Copy,
{
    fn div_assign(&mut self, rhs: T) {
        self.data.iter_mut().for_each(|a| *a /= rhs);
    }
}

// Add/Sub/Mul/Div Array<T> //

impl<T> Add<Array<T>> for Array<T>
where
    T: Add<T, Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.data.len(), rhs.data.len());

        Self::Output {
            data: self
                .data
                .into_iter()
                .zip(rhs.data.into_iter())
                .map(|(a, b)| a + b)
                .collect(),
        }
    }
}

impl<T> Sub<Array<T>> for Array<T>
where
    T: Sub<T, Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.data.len(), rhs.data.len());

        Self::Output {
            data: self
                .data
                .into_iter()
                .zip(rhs.data.into_iter())
                .map(|(a, b)| a - b)
                .collect(),
        }
    }
}

impl<T> Mul<Array<T>> for Array<T>
where
    T: Mul<T, Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.data.len(), rhs.data.len());

        Self::Output {
            data: self
                .data
                .into_iter()
                .zip(rhs.data.into_iter())
                .map(|(a, b)| a * b)
                .collect(),
        }
    }
}

impl<T> Div<Array<T>> for Array<T>
where
    T: Div<T, Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        assert_eq!(self.data.len(), rhs.data.len());

        Self::Output {
            data: self
                .data
                .into_iter()
                .zip(rhs.data.into_iter())
                .map(|(a, b)| a / b)
                .collect(),
        }
    }
}

// Add/Sub/Mul/Div Assign Array<T> //

impl<T> AddAssign<Array<T>> for Array<T>
where
    T: AddAssign<T> + Copy,
{
    fn add_assign(&mut self, rhs: Array<T>) {
        assert_eq!(self.data.len(), rhs.data.len());
        self.data
            .iter_mut()
            .zip(rhs.data.into_iter())
            .for_each(|(a, b)| *a += b);
    }
}

impl<T> SubAssign<Array<T>> for Array<T>
where
    T: SubAssign<T> + Copy,
{
    fn sub_assign(&mut self, rhs: Array<T>) {
        assert_eq!(self.data.len(), rhs.data.len());
        self.data
            .iter_mut()
            .zip(rhs.data.into_iter())
            .for_each(|(a, b)| *a -= b);
    }
}

impl<T> MulAssign<Array<T>> for Array<T>
where
    T: MulAssign<T> + Copy,
{
    fn mul_assign(&mut self, rhs: Array<T>) {
        assert_eq!(self.data.len(), rhs.data.len());
        self.data
            .iter_mut()
            .zip(rhs.data.into_iter())
            .for_each(|(a, b)| *a *= b);
    }
}

impl<T> DivAssign<Array<T>> for Array<T>
where
    T: DivAssign<T> + Copy,
{
    fn div_assign(&mut self, rhs: Array<T>) {
        assert_eq!(self.data.len(), rhs.data.len());
        self.data
            .iter_mut()
            .zip(rhs.data.into_iter())
            .for_each(|(a, b)| *a /= b);
    }
}

// Add/Sub/Mul/Div Assign &Array //

impl<T> AddAssign<&Array<T>> for Array<T>
where
    T: AddAssign<T> + Copy,
{
    fn add_assign(&mut self, rhs: &Array<T>) {
        assert_eq!(self.data.len(), rhs.data.len());
        self.data
            .iter_mut()
            .zip(rhs.data.iter())
            .for_each(|(a, b)| *a += *b);
    }
}

impl<T> SubAssign<&Array<T>> for Array<T>
where
    T: SubAssign<T> + Copy,
{
    fn sub_assign(&mut self, rhs: &Array<T>) {
        assert_eq!(self.data.len(), rhs.data.len());
        self.data
            .iter_mut()
            .zip(rhs.data.iter())
            .for_each(|(a, b)| *a -= *b);
    }
}

impl<T> MulAssign<&Array<T>> for Array<T>
where
    T: MulAssign<T> + Copy,
{
    fn mul_assign(&mut self, rhs: &Array<T>) {
        assert_eq!(self.data.len(), rhs.data.len());
        self.data
            .iter_mut()
            .zip(rhs.data.iter())
            .for_each(|(a, b)| *a *= *b);
    }
}

impl<T> DivAssign<&Array<T>> for Array<T>
where
    T: DivAssign<T> + Copy,
{
    fn div_assign(&mut self, rhs: &Array<T>) {
        assert_eq!(self.data.len(), rhs.data.len());
        self.data
            .iter_mut()
            .zip(rhs.data.iter())
            .for_each(|(a, b)| *a /= *b);
    }
}
