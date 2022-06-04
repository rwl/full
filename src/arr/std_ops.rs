use crate::arr::Arr;
use std::ops::*;

impl<T> Neg for Arr<T>
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

impl<T> Add<T> for Arr<T>
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

impl<T> Sub<T> for Arr<T>
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

impl<T> Mul<T> for Arr<T>
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

impl<T> Div<T> for Arr<T>
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

impl<T> AddAssign<T> for Arr<T>
where
    T: AddAssign<T> + Copy,
{
    fn add_assign(&mut self, rhs: T) {
        self.data.iter_mut().for_each(|a| *a += rhs);
    }
}

impl<T> SubAssign<T> for Arr<T>
where
    T: SubAssign<T> + Copy,
{
    fn sub_assign(&mut self, rhs: T) {
        self.data.iter_mut().for_each(|a| *a -= rhs);
    }
}

impl<T> MulAssign<T> for Arr<T>
where
    T: MulAssign<T> + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        self.data.iter_mut().for_each(|a| *a *= rhs);
    }
}

impl<T> DivAssign<T> for Arr<T>
where
    T: DivAssign<T> + Copy,
{
    fn div_assign(&mut self, rhs: T) {
        self.data.iter_mut().for_each(|a| *a /= rhs);
    }
}

// Add/Sub/Mul/Div Array<T> //

impl<T> Add<Arr<T>> for Arr<T>
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

impl<T> Sub<Arr<T>> for Arr<T>
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

impl<T> Mul<Arr<T>> for Arr<T>
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

impl<T> Div<Arr<T>> for Arr<T>
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

impl<T> AddAssign<Arr<T>> for Arr<T>
where
    T: AddAssign<T> + Copy,
{
    fn add_assign(&mut self, rhs: Arr<T>) {
        assert_eq!(self.data.len(), rhs.data.len());
        self.data
            .iter_mut()
            .zip(rhs.data.into_iter())
            .for_each(|(a, b)| *a += b);
    }
}

impl<T> SubAssign<Arr<T>> for Arr<T>
where
    T: SubAssign<T> + Copy,
{
    fn sub_assign(&mut self, rhs: Arr<T>) {
        assert_eq!(self.data.len(), rhs.data.len());
        self.data
            .iter_mut()
            .zip(rhs.data.into_iter())
            .for_each(|(a, b)| *a -= b);
    }
}

impl<T> MulAssign<Arr<T>> for Arr<T>
where
    T: MulAssign<T> + Copy,
{
    fn mul_assign(&mut self, rhs: Arr<T>) {
        assert_eq!(self.data.len(), rhs.data.len());
        self.data
            .iter_mut()
            .zip(rhs.data.into_iter())
            .for_each(|(a, b)| *a *= b);
    }
}

impl<T> DivAssign<Arr<T>> for Arr<T>
where
    T: DivAssign<T> + Copy,
{
    fn div_assign(&mut self, rhs: Arr<T>) {
        assert_eq!(self.data.len(), rhs.data.len());
        self.data
            .iter_mut()
            .zip(rhs.data.into_iter())
            .for_each(|(a, b)| *a /= b);
    }
}

// Add/Sub/Mul/Div Assign &Array //

impl<T> AddAssign<&Arr<T>> for Arr<T>
where
    T: AddAssign<T> + Copy,
{
    fn add_assign(&mut self, rhs: &Arr<T>) {
        assert_eq!(self.data.len(), rhs.data.len());
        self.data
            .iter_mut()
            .zip(rhs.data.iter())
            .for_each(|(a, b)| *a += *b);
    }
}

impl<T> SubAssign<&Arr<T>> for Arr<T>
where
    T: SubAssign<T> + Copy,
{
    fn sub_assign(&mut self, rhs: &Arr<T>) {
        assert_eq!(self.data.len(), rhs.data.len());
        self.data
            .iter_mut()
            .zip(rhs.data.iter())
            .for_each(|(a, b)| *a -= *b);
    }
}

impl<T> MulAssign<&Arr<T>> for Arr<T>
where
    T: MulAssign<T> + Copy,
{
    fn mul_assign(&mut self, rhs: &Arr<T>) {
        assert_eq!(self.data.len(), rhs.data.len());
        self.data
            .iter_mut()
            .zip(rhs.data.iter())
            .for_each(|(a, b)| *a *= *b);
    }
}

impl<T> DivAssign<&Arr<T>> for Arr<T>
where
    T: DivAssign<T> + Copy,
{
    fn div_assign(&mut self, rhs: &Arr<T>) {
        assert_eq!(self.data.len(), rhs.data.len());
        self.data
            .iter_mut()
            .zip(rhs.data.iter())
            .for_each(|(a, b)| *a /= *b);
    }
}
