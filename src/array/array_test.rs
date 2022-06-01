use crate::array::Array;

#[test]
fn test_array() {
    let mut a = Array::<f64>::ones(6);
    let mut b = Array::<f64>::ones(6);

    a += 1.0;
    b *= 2.0;
    a += &b;
    println!("{}", a);
    println!("{}", b);
}
