use crate::matrix::Matrix;
use crate::traits::Complex;

pub trait CMatrix<C, F> {
    fn real(&self) -> Matrix<F>;
    fn imag(&self) -> Matrix<F>;
    fn conj(&self) -> Matrix<C>;
}

impl<C> CMatrix<C, f64> for Matrix<C>
where
    C: Complex<f64>,
{
    fn real(&self) -> Matrix<f64> {
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: self.data.iter().map(|c| c.real()).collect(),
        }
    }

    fn imag(&self) -> Matrix<f64> {
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: self.data.iter().map(|c| c.imag()).collect(),
        }
    }

    fn conj(&self) -> Matrix<C> {
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: self.data.iter().map(|c| c.conj()).collect(),
        }
    }
}
