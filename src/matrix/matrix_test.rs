use crate::matrix::Matrix;

#[test]
fn test_matrix() {
    let mut a = Matrix::<f64>::identity(6);
    let mut b = Matrix::<f64>::identity(6);

    a /= 2.0;
    b *= 3.0;
    // println!("{}", a * b);
    a *= b;
    println!("{}", -a - 3.0);
}
