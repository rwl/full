use crate::mat::Mat;

#[test]
fn test_matrix() {
    let mut a = Mat::<f64>::identity(6);
    let mut b = Mat::<f64>::identity(6);

    a /= 2.0;
    b *= 3.0;
    // println!("{}", a * b);
    a *= b;
    println!("{}", -a - 3.0);
}
