use crate::mat::Mat;
use crate::traits::Complex;

pub trait CMat<C, F> {
    fn real(&self) -> Mat<F>;
    fn imag(&self) -> Mat<F>;
    fn conj(&self) -> Mat<C>;
}

impl<C> CMat<C, f64> for Mat<C>
where
    C: Complex<f64>,
{
    fn real(&self) -> Mat<f64> {
        Mat {
            rows: self.rows,
            cols: self.cols,
            data: self.data.iter().map(|c| c.real()).collect(),
            col_major: self.col_major,
        }
    }

    fn imag(&self) -> Mat<f64> {
        Mat {
            rows: self.rows,
            cols: self.cols,
            data: self.data.iter().map(|c| c.imag()).collect(),
            col_major: self.col_major,
        }
    }

    fn conj(&self) -> Mat<C> {
        Mat {
            rows: self.rows,
            cols: self.cols,
            data: self.data.iter().map(|c| c.conj()).collect(),
            col_major: self.col_major,
        }
    }
}
