use crate::array::Array;
use crate::traits::{Complex, Float};

pub trait CArray<F, C> {
    fn from_parts(re: &[F], im: &[F]) -> Array<C>;
    fn from_real(re: &[F]) -> Array<C>;
    fn from_imag(im: &[F]) -> Array<C>;

    /// Returns a complex array of size `a.len()/2`, where `a`
    /// has interleaved complex parts e.g. `[re0, im0, re1, im1...]`.
    fn from_interleaved(interleaved: &[F]) -> Array<C>;

    fn from_polar(norm: &[F], arg: &[F]) -> Array<C>;

    fn real(&self) -> Array<F>;
    fn imag(&self) -> Array<F>;

    fn conj(&self) -> Self;

    fn norm(&self) -> Array<F>;
    fn arg(&self) -> Array<F>;
    fn to_polar(&self) -> (Array<F>, Array<F>);

    /// Returns a float array of size `2*self.len()` with complex
    /// parts interleaved e.g. `[re0, im0, re1, im1...]`.
    fn interleave(&self) -> Array<F>;
}

impl<F, C> CArray<F, C> for Array<C>
where
    F: Float,
    C: Complex<F>,
{
    fn from_parts(real: &[F], imag: &[F]) -> Array<C> {
        assert_eq!(real.len(), imag.len());

        Array {
            data: real
                .iter()
                .zip(imag)
                .map(|(&re, &im)| C::new(re, im))
                .collect(),
        }
    }

    fn from_real(real: &[F]) -> Array<C> {
        Array {
            data: real.iter().map(|&re| C::new(re, F::zero())).collect(),
        }
    }
    fn from_imag(imag: &[F]) -> Array<C> {
        Array {
            data: imag.iter().map(|&im| C::new(F::zero(), im)).collect(),
        }
    }

    fn from_interleaved(interleaved: &[F]) -> Array<C> {
        assert_eq!(interleaved.len() % 2, 0);

        let n = interleaved.len() / 2;
        let mut data = Vec::with_capacity(n);
        for i in 0..n {
            data.push(C::new(interleaved[2 * i], interleaved[2 * i + 1]));
        }

        Array { data }
    }

    fn from_polar(r: &[F], theta: &[F]) -> Array<C> {
        assert_eq!(r.len(), theta.len());

        Array {
            data: r
                .iter()
                .zip(theta)
                .map(|p| C::from_polar(*p.0, *p.1))
                .collect(),
        }
    }

    fn real(&self) -> Array<F> {
        Array {
            data: self.data.iter().map(|c| c.real()).collect(),
        }
    }
    fn imag(&self) -> Array<F> {
        Array {
            data: self.data.iter().map(|c| c.imag()).collect(),
        }
    }

    fn conj(&self) -> Self {
        Array {
            data: self.data.iter().map(|c| c.conj()).collect(),
        }
    }

    fn norm(&self) -> Array<F> {
        Array {
            data: self.data.iter().map(|c| c.norm()).collect(),
        }
    }
    fn arg(&self) -> Array<F> {
        Array {
            data: self.data.iter().map(|c| c.arg()).collect(),
        }
    }
    fn to_polar(&self) -> (Array<F>, Array<F>) {
        (self.norm(), self.arg())
    }

    fn interleave(&self) -> Array<F> {
        let mut data = Vec::with_capacity(2 * self.len());
        for c in &self.data {
            data.push(c.real());
            data.push(c.imag());
        }
        Array { data }
    }
}
