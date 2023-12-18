use num_traits::float::FloatCore;

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
    fn pow(&self, r: i32) -> Self;
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
    fn pow(&self, r: i32) -> Self {
        num_complex::Complex::powi(self, r)
    }
}

// Norm //

pub trait Norm<F> {
    fn norm(&self) -> F;
}

impl Norm<f64> for f64 {
    fn norm(&self) -> f64 {
        f64::abs(*self)
    }
}

impl Norm<f32> for f32 {
    fn norm(&self) -> f32 {
        f32::abs(*self)
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

// Ln //

pub trait Ln {
    fn ln(&self) -> Self;
}

impl Ln for f64 {
    fn ln(&self) -> Self {
        f64::ln(*self)
    }
}

impl Ln for f32 {
    fn ln(&self) -> Self {
        f32::ln(*self)
    }
}

impl<F: Float> Ln for num_complex::Complex<F> {
    fn ln(&self) -> Self {
        num_complex::Complex::ln(*self)
    }
}

// Exp //

pub trait Exp {
    fn exp(&self) -> Self;
}

impl Exp for f64 {
    fn exp(&self) -> Self {
        f64::exp(*self)
    }
}

impl Exp for f32 {
    fn exp(&self) -> Self {
        f32::exp(*self)
    }
}

impl<F: Float> Exp for num_complex::Complex<F> {
    fn exp(&self) -> Self {
        num_complex::Complex::exp(*self)
    }
}

// Abs //

pub trait Abs {
    fn abs(&self) -> Self;
}

impl Abs for f64 {
    fn abs(&self) -> Self {
        f64::abs(*self)
    }
}

impl Abs for f32 {
    fn abs(&self) -> Self {
        f32::abs(*self)
    }
}

// Round //

pub trait Round {
    fn round(&self) -> Self;
}

impl Round for f64 {
    fn round(&self) -> Self {
        f64::round(*self)
    }
}

impl Round for f32 {
    fn round(&self) -> Self {
        f32::round(*self)
    }
}

// Sqrt //

pub trait Sqrt {
    fn sqrt(&self) -> Self;
}

impl Sqrt for f64 {
    fn sqrt(&self) -> Self {
        f64::sqrt(*self)
    }
}

impl Sqrt for f32 {
    fn sqrt(&self) -> Self {
        f32::sqrt(*self)
    }
}

impl<F: Float> Sqrt for num_complex::Complex<F> {
    fn sqrt(&self) -> Self {
        num_complex::Complex::sqrt(*self)
    }
}

// IsNaN //

pub trait IsNaN {
    fn is_nan(&self) -> bool;
}

impl IsNaN for f64 {
    fn is_nan(&self) -> bool {
        f64::is_nan(*self)
    }
}

impl IsNaN for f32 {
    fn is_nan(&self) -> bool {
        f32::is_nan(*self)
    }
}

impl<F: Float + FloatCore> IsNaN for num_complex::Complex<F> {
    fn is_nan(&self) -> bool {
        num_complex::Complex::is_nan(*self)
    }
}

// Sin //

pub trait Sin {
    fn sin(&self) -> Self;
}

impl Sin for f64 {
    fn sin(&self) -> Self {
        f64::sin(*self)
    }
}

impl Sin for f32 {
    fn sin(&self) -> Self {
        f32::sin(*self)
    }
}

impl<F: Float> Sin for num_complex::Complex<F> {
    fn sin(&self) -> Self {
        num_complex::Complex::sin(*self)
    }
}

// Cos //

pub trait Cos {
    fn cos(&self) -> Self;
}

impl Cos for f64 {
    fn cos(&self) -> Self {
        f64::cos(*self)
    }
}

impl Cos for f32 {
    fn cos(&self) -> Self {
        f32::cos(*self)
    }
}

impl<F: Float> Cos for num_complex::Complex<F> {
    fn cos(&self) -> Self {
        num_complex::Complex::cos(*self)
    }
}

// ArcSin //

pub trait ArcSin {
    fn asin(&self) -> Self;
}

impl ArcSin for f64 {
    fn asin(&self) -> Self {
        f64::asin(*self)
    }
}

impl ArcSin for f32 {
    fn asin(&self) -> Self {
        f32::asin(*self)
    }
}

impl<F: Float> ArcSin for num_complex::Complex<F> {
    fn asin(&self) -> Self {
        num_complex::Complex::asin(*self)
    }
}

// ArcCos //

pub trait ArcCos {
    fn acos(&self) -> Self;
}

impl ArcCos for f64 {
    fn acos(&self) -> Self {
        f64::acos(*self)
    }
}

impl ArcCos for f32 {
    fn acos(&self) -> Self {
        f32::acos(*self)
    }
}

impl<F: Float> ArcCos for num_complex::Complex<F> {
    fn acos(&self) -> Self {
        num_complex::Complex::acos(*self)
    }
}
