use crate::arr::Arr;
use num_complex::Complex64;
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

impl<T> Neg for &Arr<T>
where
    T: Neg<Output = T> + Copy,
{
    type Output = Arr<T>;

    fn neg(self) -> Self::Output {
        Self::Output {
            data: self.data.iter().map(|&a| -a).collect(),
        }
    }
}

// T Add/Sub/Mul/Div //

#[opimps::impl_ops_lprim(Add)]
fn add(self: f64, rhs: Arr<f64>) -> Arr<f64> {
    Arr {
        data: rhs.data.iter().map(|&a| self + a).collect(),
    }
}

#[opimps::impl_ops_lprim(Add)]
fn add(self: Complex64, rhs: Arr<Complex64>) -> Arr<Complex64> {
    Arr {
        data: rhs.data.iter().map(|&a| self + a).collect(),
    }
}

#[opimps::impl_ops_lprim(Sub)]
fn sub(self: f64, rhs: Arr<f64>) -> Arr<f64> {
    Arr {
        data: rhs.data.iter().map(|&a| self - a).collect(),
    }
}

#[opimps::impl_ops_lprim(Sub)]
fn sub(self: Complex64, rhs: Arr<Complex64>) -> Arr<Complex64> {
    Arr {
        data: rhs.data.iter().map(|&a| self - a).collect(),
    }
}

#[opimps::impl_ops_lprim(Mul)]
fn mul(self: f64, rhs: Arr<f64>) -> Arr<f64> {
    Arr {
        data: rhs.data.iter().map(|&a| self * a).collect(),
    }
}

#[opimps::impl_ops_lprim(Mul)]
fn mul(self: Complex64, rhs: Arr<Complex64>) -> Arr<Complex64> {
    Arr {
        data: rhs.data.iter().map(|&a| self * a).collect(),
    }
}

#[opimps::impl_ops_lprim(Div)]
fn div(self: f64, rhs: Arr<f64>) -> Arr<f64> {
    Arr {
        data: rhs.data.iter().map(|&a| self / a).collect(),
    }
}

#[opimps::impl_ops_lprim(Div)]
fn div(self: Complex64, rhs: Arr<Complex64>) -> Arr<Complex64> {
    Arr {
        data: rhs.data.iter().map(|&a| self / a).collect(),
    }
}

// TODO: 32bit

// Add/Sub/Mul/Div T //

#[opimps::impl_ops_rprim(Add)]
fn add<T>(self: Arr<T>, rhs: T) -> Arr<T>
where
    T: Add<T, Output = T> + Copy,
{
    Arr {
        data: self.data.iter().map(|&a| a + rhs).collect(),
    }
}

#[opimps::impl_ops_rprim(Sub)]
fn sub<T>(self: Arr<T>, rhs: T) -> Arr<T>
where
    T: Sub<T, Output = T> + Copy,
{
    Arr {
        data: self.data.iter().map(|&a| a - rhs).collect(),
    }
}

#[opimps::impl_ops_rprim(Mul)]
fn mul<T>(self: Arr<T>, rhs: T) -> Arr<T>
where
    T: Mul<T, Output = T> + Copy,
{
    Arr {
        data: self.data.iter().map(|&a| a * rhs).collect(),
    }
}

#[opimps::impl_ops_rprim(Div)]
fn div<T>(self: Arr<T>, rhs: T) -> Arr<T>
where
    T: Div<T, Output = T> + Copy,
{
    Arr {
        data: self.data.iter().map(|&a| a / rhs).collect(),
    }
}

// Add/Sub/Mul/Div Arr<T> //

#[opimps::impl_ops(Add)]
fn add<T>(self: Arr<T>, rhs: Arr<T>) -> Arr<T>
where
    T: Add<T, Output = T> + Copy,
{
    assert_eq!(self.data.len(), rhs.data.len());

    Self::Output {
        data: self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(&a, &b)| a + b)
            .collect(),
    }
}

#[opimps::impl_ops(Sub)]
fn sub<T>(self: Arr<T>, rhs: Arr<T>) -> Arr<T>
where
    T: Sub<T, Output = T> + Copy,
{
    assert_eq!(self.data.len(), rhs.data.len());

    Self::Output {
        data: self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(&a, &b)| a - b)
            .collect(),
    }
}

#[opimps::impl_ops(Mul)]
fn mul<T>(self: Arr<T>, rhs: Arr<T>) -> Arr<T>
where
    T: Mul<T, Output = T> + Copy,
{
    assert_eq!(self.data.len(), rhs.data.len());

    Self::Output {
        data: self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(&a, &b)| a * b)
            .collect(),
    }
}

#[opimps::impl_ops(Div)]
fn div<T>(self: Arr<T>, rhs: Arr<T>) -> Arr<T>
where
    T: Div<T, Output = T> + Copy,
{
    assert_eq!(self.data.len(), rhs.data.len());

    Self::Output {
        data: self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(&a, &b)| a / b)
            .collect(),
    }
}

// Add/Sub/Mul/Div Assign<Arr<T>> //

#[opimps::impl_ops_assign(std::ops::AddAssign)]
fn add_assign<T>(self: Arr<T>, rhs: Arr<T>)
where
    T: AddAssign<T> + Copy,
{
    assert_eq!(self.data.len(), rhs.data.len());
    self.data
        .iter_mut()
        .zip(rhs.data.iter())
        .for_each(|(a, &b)| *a += b);
}

#[opimps::impl_ops_assign(std::ops::SubAssign)]
fn sub_assign<T>(self: Arr<T>, rhs: Arr<T>)
where
    T: SubAssign<T> + Copy,
{
    assert_eq!(self.data.len(), rhs.data.len());
    self.data
        .iter_mut()
        .zip(rhs.data.iter())
        .for_each(|(a, &b)| *a -= b);
}

#[opimps::impl_ops_assign(std::ops::MulAssign)]
fn mul_assign<T>(self: Arr<T>, rhs: Arr<T>)
where
    T: MulAssign<T> + Copy,
{
    assert_eq!(self.data.len(), rhs.data.len());
    self.data
        .iter_mut()
        .zip(rhs.data.iter())
        .for_each(|(a, &b)| *a *= b);
}

#[opimps::impl_ops_assign(std::ops::DivAssign)]
fn div_assign<T>(self: Arr<T>, rhs: Arr<T>)
where
    T: DivAssign<T> + Copy,
{
    assert_eq!(self.data.len(), rhs.data.len());
    self.data
        .iter_mut()
        .zip(rhs.data.iter())
        .for_each(|(a, &b)| *a /= b);
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
