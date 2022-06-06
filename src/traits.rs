pub trait Float: num_traits::Float + num_traits::Zero {}

impl Float for f64 {}
impl Float for f32 {}

pub trait Complex<T> {
    fn new(re: T, im: T) -> Self;
    fn from_polar(norm: T, arg: T) -> Self;

    fn real(&self) -> T;
    fn imag(&self) -> T;

    fn conj(&self) -> Self;

    fn norm(&self) -> T;
    fn arg(&self) -> T;
}

impl<F> Complex<F> for num_complex::Complex<F>
where
    F: Float,
{
    fn new(re: F, im: F) -> Self {
        num_complex::Complex::new(re, im)
    }
    fn from_polar(r: F, theta: F) -> Self {
        num_complex::Complex::from_polar(r, theta)
    }

    fn real(&self) -> F {
        self.re
    }
    fn imag(&self) -> F {
        self.im
    }

    fn conj(&self) -> Self {
        num_complex::Complex::conj(self)
    }

    fn norm(&self) -> F {
        num_complex::Complex::norm(*self)
    }
    fn arg(&self) -> F {
        num_complex::Complex::arg(*self)
    }
}

pub trait Norm<F> {
    fn norm(&self) -> F;
}

impl Norm<f64> for f64 {
    fn norm(&self) -> f64 {
        f64::abs(*self)
    }
}

impl<F> Norm<F> for num_complex::Complex<F>
where
    F: Float,
{
    fn norm(&self) -> F {
        num_complex::Complex::norm(*self)
    }
}
