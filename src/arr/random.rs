use crate::arr::Arr;
use num_complex::Complex64;
use rand::Rng;

pub trait Rand {
    /// Creates an array with pseudo-random values in `[0.0,1.0)`.
    fn rand(n: usize) -> Self;

    /// Creates a normally distributed array with
    /// values in the range `[-T::MAX, +T::MAX]`
    /// from a standard normal distribution (mean=0, stddev=1).
    fn randn(n: usize) -> Self;
}

impl Rand for Arr<f64> {
    fn rand(n: usize) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            values: (0..n).map(|_| rng.gen()).collect(),
        }
    }

    fn randn(n: usize) -> Self {
        let mut rng = rand::thread_rng();
        // let normal = rand::distributions::StandardNormal::new();
        Self {
            values: (0..n)
                .map(|_| rng.sample(rand_distr::StandardNormal))
                .collect(),
        }
    }
}

impl Rand for Arr<Complex64> {
    fn rand(n: usize) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            values: (0..n)
                .map(|_| Complex64::new(rng.gen(), rng.gen()))
                .collect(),
        }
    }

    fn randn(n: usize) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            values: (0..n)
                .map(|_| {
                    Complex64::new(
                        rng.sample(rand_distr::StandardNormal),
                        rng.sample(rand_distr::StandardNormal),
                    )
                })
                .collect(),
        }
    }
}
