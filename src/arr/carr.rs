use crate::arr::Arr;
use crate::traits::{Complex, Float};

pub trait CArr<F, C> {
    fn from_parts(re: &[F], im: &[F]) -> Arr<C>;
    fn from_real(re: &[F]) -> Arr<C>;
    fn from_imag(im: &[F]) -> Arr<C>;

    /// Returns a complex array of size `a.len()/2`, where `a`
    /// has interleaved complex parts e.g. `[re0, im0, re1, im1...]`.
    fn from_interleaved(interleaved: &[F]) -> Arr<C>;

    fn from_polar(norm: &[F], arg: &[F]) -> Arr<C>;

    fn real(&self) -> Arr<F>;
    fn imag(&self) -> Arr<F>;

    fn conj(&self) -> Self;

    fn norm(&self) -> Arr<F>;
    fn arg(&self) -> Arr<F>;
    fn to_polar(&self) -> (Arr<F>, Arr<F>);

    /// Returns a float array of size `2*self.len()` with complex
    /// parts interleaved e.g. `[re0, im0, re1, im1...]`.
    fn interleave(&self) -> Arr<F>;
}

impl<F, C> CArr<F, C> for Arr<C>
where
    F: Float,
    C: Complex<F>,
{
    fn from_parts(real: &[F], imag: &[F]) -> Arr<C> {
        assert_eq!(real.len(), imag.len());

        Arr {
            data: real
                .iter()
                .zip(imag)
                .map(|(&re, &im)| C::new(re, im))
                .collect(),
        }
    }

    fn from_real(real: &[F]) -> Arr<C> {
        Arr {
            data: real.iter().map(|&re| C::new(re, F::zero())).collect(),
        }
    }
    fn from_imag(imag: &[F]) -> Arr<C> {
        Arr {
            data: imag.iter().map(|&im| C::new(F::zero(), im)).collect(),
        }
    }

    fn from_interleaved(interleaved: &[F]) -> Arr<C> {
        assert_eq!(interleaved.len() % 2, 0);

        let n = interleaved.len() / 2;
        let mut data = Vec::with_capacity(n);
        for i in 0..n {
            data.push(C::new(interleaved[2 * i], interleaved[2 * i + 1]));
        }

        Arr { data }
    }

    fn from_polar(r: &[F], theta: &[F]) -> Arr<C> {
        assert_eq!(r.len(), theta.len());

        Arr {
            data: r
                .iter()
                .zip(theta)
                .map(|p| C::from_polar(*p.0, *p.1))
                .collect(),
        }
    }

    fn real(&self) -> Arr<F> {
        Arr {
            data: self.data.iter().map(|c| c.real()).collect(),
        }
    }
    fn imag(&self) -> Arr<F> {
        Arr {
            data: self.data.iter().map(|c| c.imag()).collect(),
        }
    }

    fn conj(&self) -> Self {
        Arr {
            data: self.data.iter().map(|c| c.conj()).collect(),
        }
    }

    fn norm(&self) -> Arr<F> {
        Arr {
            data: self.data.iter().map(|c| c.norm()).collect(),
        }
    }
    fn arg(&self) -> Arr<F> {
        Arr {
            data: self.data.iter().map(|c| c.arg()).collect(),
        }
    }
    fn to_polar(&self) -> (Arr<F>, Arr<F>) {
        (self.norm(), self.arg())
    }

    fn interleave(&self) -> Arr<F> {
        let mut data = Vec::with_capacity(2 * self.len());
        for c in &self.data {
            data.push(c.real());
            data.push(c.imag());
        }
        Arr { data }
    }
}

pub fn conj<F, C>(a: &[C]) -> Arr<C>
where
    F: Float,
    C: Complex<F>,
{
    Arr {
        data: a.iter().map(|c| c.conj()).collect(),
    }
}

pub fn pow<F, C>(a: &[C], r: i32) -> Arr<C>
where
    F: Float,
    C: Complex<F>,
{
    Arr {
        data: a.iter().map(|c| c.pow(r)).collect(),
    }
}
