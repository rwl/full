use rand::distributions::{Distribution, Standard};
use rand::Rng;

pub fn random<T>(n_row: usize, n_col: usize) -> Vec<T>
where
    Standard: Distribution<T>,
{
    let mut rng = rand::thread_rng();
    (0..n_row * n_col).map(|_| rng.gen::<T>()).collect()
}
