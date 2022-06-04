use crate::arr::Arr;

#[test]
fn test_array() {
    let mut a = Arr::<f64>::ones(6);
    let mut b = Arr::<f64>::ones(6);

    a += 1.0;
    b *= 2.0;
    a += &b;
    println!("{}", a);
    println!("{}", b);
}
