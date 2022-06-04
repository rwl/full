use crate::arr::{Arr, CArr};
use num_complex::Complex64;

#[test]
fn test_complex() {
    let c = Arr::<Complex64>::from_parts(&vec![1.0, 2.0], &vec![2.0, 3.0]);

    println!("{}", c.to_polar().0)
}
