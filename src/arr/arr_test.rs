use crate::arr::random::Rand;
use crate::arr::Arr;
use crate::slice::dot;
use rand::Rng;

const TEST_N: usize = 10_000;

// #[test]
// fn test_array() {
//     let mut a = Arr::<f64>::ones(6);
//     let mut b = Arr::<f64>::ones(6);
//
//     a += 1.0;
//     b *= 2.0;
//     a += &b;
//     println!("{}", a);
//     println!("{}", b);
//
//     let r = Arr::<Complex64>::rand(6);
//     println!("{}", r);
// }

#[test]
fn test_new() {
    let mut rng = rand::thread_rng();

    let n = rng.gen_range(0..TEST_N) + 1;
    let v0: f64 = rng.gen();
    let a = Arr::with_value(n, v0);
    assert_eq!(a.len(), n, "new len, expected {} actual {}", n, a.len());
    // if len(a) != n {
    // 	t.Errorf("new, expected %v actual %v", n, a)
    // }
    for i in 0..n {
        assert_eq!(a[i], v0, "new[{}], expected {} actual {}", i, v0, a[i])
        // if a[i] != v0 {
        // 	t.Errorf("new, expected %v actual %v", v0, a[i])
        // }
    }
}

#[test]
fn test_zeros() {
    let mut rng = rand::thread_rng();

    let n = rng.gen_range(0..TEST_N) + 1;
    let a = Arr::<f64>::zeros(n);
    assert_eq!(a.len(), n, "zeros, expected {} actual {}", n, a.len());
    for i in 0..n {
        assert_eq!(a[i], 0.0, "zeros[{}], expected {} actual {}", i, 0.0, a[i]);
    }
}

#[test]
fn test_ones() {
    let mut rng = rand::thread_rng();

    let n = rng.gen_range(0..TEST_N) + 1;
    let a = Arr::<f64>::ones(n);
    assert_eq!(a.len(), n, "ones, expected {} actual {}", n, a.len());
    for i in 0..n {
        assert_eq!(a[i], 1.0, "ones[{}], expected {} actual {}", i, 1.0, a[i]);
    }
}

#[test]
fn test_rand() {
    let mut rng = rand::thread_rng();

    let n = rng.gen_range(0..TEST_N) + 1;
    let a = Arr::<f64>::rand(n);
    assert_eq!(a.len(), n, "rand, expected {} actual {}", n, a.len());
    for i in 0..n {
        assert!(
            a[i] >= 0.0,
            "rand[{}], expected >= {} actual {}",
            i,
            0.0,
            a[i]
        );
        assert!(
            a[i] <= 1.0,
            "rand[{}], expected <= {} actual {}",
            i,
            1.0,
            a[i]
        );
    }
}

#[test]
fn test_fill() {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..TEST_N) + 1;
    let v = rng.gen();
    let mut a = Arr::<f64>::zeros(n);
    a.fill(v);
    for i in 0..n {
        assert_eq!(a[i], v, "fill[{}], expected {} actual {}", i, v, a[i])
    }
}

#[test]
fn test_add() {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..TEST_N) + 1;
    let a = Arr::<f64>::rand(n);
    let b = Arr::<f64>::rand(n);
    let c = &a + &b;
    assert_eq!(c.len(), n, "add, expected {} actual {}", n, c.len());
    for i in 0..n {
        assert_eq!(
            c[i],
            a[i] + b[i],
            "add[{}], expected {} actual {}",
            i,
            a[i] + b[i],
            c[i]
        )
    }
}

#[test]
fn test_add_assign() {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..TEST_N) + 1;
    let mut a = Arr::<f64>::rand(n);
    let a0 = a.clone();
    let b = Arr::<f64>::rand(n);
    a += &b;
    assert_eq!(a.len(), n, "add, expected {} actual {}", n, a.len());
    for i in 0..n {
        assert_eq!(
            a[i],
            a0[i] + b[i],
            "add[{}], expected {} actual {}",
            i,
            a0[i] + b[i],
            a[i]
        )
    }
}

#[test]
fn test_sub() {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..TEST_N) + 1;
    let a = Arr::<f64>::rand(n);
    let b = Arr::<f64>::rand(n);
    let c = &a - &b;
    assert_eq!(c.len(), n, "sub, expected {} actual {}", n, c.len());
    for i in 0..n {
        assert_eq!(
            c[i],
            a[i] - b[i],
            "sub[{}], expected {} actual {}",
            i,
            a[i] - b[i],
            c[i]
        )
    }
}

#[test]
fn test_sub_assign() {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..TEST_N) + 1;
    let mut a = Arr::<f64>::rand(n);
    let a0 = a.clone();
    let b = Arr::<f64>::rand(n);
    a -= &b;
    assert_eq!(a.len(), n, "sub, expected {} actual {}", n, a.len());
    for i in 0..n {
        assert_eq!(
            a[i],
            a0[i] - b[i],
            "sub[{}], expected {} actual {}",
            i,
            a0[i] - b[i],
            a[i]
        )
    }
}

#[test]
fn test_mul() {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..TEST_N) + 1;
    let a = Arr::<f64>::rand(n);
    let b = Arr::<f64>::rand(n);
    let c = &a * &b;
    assert_eq!(c.len(), n, "mul, expected {} actual {}", n, c.len());
    for i in 0..n {
        assert_eq!(
            c[i],
            a[i] * b[i],
            "mul[{}], expected {} actual {}",
            i,
            a[i] * b[i],
            c[i]
        )
    }
}

#[test]
fn test_mul_assign() {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..TEST_N) + 1;
    let mut a = Arr::<f64>::rand(n);
    let a0 = a.clone();
    let b = Arr::<f64>::rand(n);
    a *= &b;
    assert_eq!(a.len(), n, "mul, expected {} actual {}", n, a.len());
    for i in 0..n {
        assert_eq!(
            a[i],
            a0[i] * b[i],
            "mul[{}], expected {} actual {}",
            i,
            a0[i] * b[i],
            a[i]
        )
    }
}

#[test]
fn test_div() {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..TEST_N) + 1;
    let a = Arr::<f64>::rand(n);
    let b = Arr::<f64>::rand(n);
    let c = &a / &b;
    assert_eq!(c.len(), n, "div, expected {} actual {}", n, c.len());
    for i in 0..n {
        assert_eq!(
            c[i],
            a[i] / b[i],
            "div[{}], expected {} actual {}",
            i,
            a[i] / b[i],
            c[i]
        )
    }
}

#[test]
fn test_div_assign() {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..TEST_N) + 1;
    let mut a = Arr::<f64>::rand(n);
    let a0 = a.clone();
    let b = Arr::<f64>::rand(n);
    a /= &b;
    assert_eq!(a.len(), n, "div, expected {} actual {}", n, a.len());
    for i in 0..n {
        assert_eq!(
            a[i],
            a0[i] / b[i],
            "div[{}], expected {} actual {}",
            i,
            a0[i] / b[i],
            a[i]
        )
    }
}

#[test]
fn test_dot() {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..TEST_N) + 1;
    let a = Arr::<f64>::rand(n);
    let b = Arr::<f64>::rand(n);
    let prod = dot(&a, &b);
    assert_ne!(prod, 0.0, "dot, actual {}", prod);
}

#[test]
fn test_scale() {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..TEST_N) + 1;
    let a: Arr<f64> = Arr::ones(n);
    let r = rng.gen::<f64>();
    let b = &a * r;
    assert_eq!(
        b.len(),
        a.len(),
        "scale, expected {} actual {}",
        a.len(),
        b.len()
    );
    assert_eq!(a[0], 1.0, "scale, expected {} actual {}", 1.0, a[0]);
    assert_eq!(b[0], r, "scale, expected {} actual {}", r, b[0]);
}

#[test]
fn test_shift() {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..TEST_N) + 1;
    let a = Arr::ones(n);
    let r = rng.gen::<f64>();
    let b = &a + r;
    assert_eq!(
        b.len(),
        a.len(),
        "shift, expected {} actual {}",
        a.len(),
        b.len()
    );

    assert_eq!(a[0], 1.0, "shift, expected {} actual {}", 1.0, a[0]);
    assert_eq!(b[0], 1.0 + r, "shift, expected {} actual {}", 1.0 + r, b[0]);
}

#[test]
fn test_reciprocal() {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..TEST_N) + 1;
    let a: Arr<f64> = Arr::rand(n);
    let b = 1.0 / &a;
    assert_eq!(
        b.len(),
        a.len(),
        "reciprocal, expected {} actual {}",
        a.len(),
        b.len()
    );

    assert_ne!(a[0], b[0], "reciprocal, expected not {}", b[0]);
    assert_eq!(
        b[0],
        1.0 / a[0],
        "reciprocal, expected {} actual {}",
        1.0 / a[0],
        b[0]
    )
}

#[test]
fn test_sum() {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..TEST_N) + 1;
    let a = Arr::<f64>::ones(n);
    let sum = a.sum();
    assert_eq!(sum, n as f64, "sum, expected {} actual {}", n, sum);
}

#[test]
fn test_select() {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..TEST_N) + 1;
    let a = Arr::<f64>::rand(n);
    let ix = vec![1];
    let b = a.select(&ix);
    assert_eq!(
        b.len(),
        ix.len(),
        "select, expected {} actual {}",
        ix.len(),
        b.len()
    );
    assert_eq!(b[0], a[1], "select, expected {} actual {}", a[1], b[0])
}

#[test]
fn test_set() {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..TEST_N) + 1;
    let mut a = Arr::<f64>::zeros(n);
    let v = vec![rng.gen()];
    a.set(&vec![1], &v);
    assert_eq!(a[0], 0.0, "set, expected {} actual {}", 0.0, a[0]);
    assert_eq!(a[1], v[0], "set, expected {} actual {}", v[0], a[1]);
}

#[test]
fn test_set_all() {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..TEST_N) + 1;
    let mut a = Arr::<f64>::zeros(n);
    let v = rng.gen();
    a.set_all(&[1], v);
    assert_eq!(a[0], 0.0, "set all, expected {} actual {}", 0.0, a[0]);
    assert_eq!(a[1], v, "set all, expected {} actual {}", v, a[1]);
}

#[test]
fn test_concatenate() {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..TEST_N) + 1;
    let a1 = Arr::<f64>::rand(n);
    let a2 = Arr::<f64>::rand(n);
    let a3 = Arr::<f64>::rand(n);
    let b = Arr::concat(&[&a1, &a2, &a3]);
    assert_eq!(
        b.len(),
        a1.len() + a2.len() + a3.len(),
        "concatenate, expected {} actual {}",
        a1.len() + a2.len() + a3.len(),
        b.len()
    );

    assert_eq!(
        b[0], a1[0],
        "concatenate, expected {} actual {}",
        a1[0], b[0]
    );

    assert_eq!(
        b[a1.len()],
        a2[0],
        "concatenate, expected {} actual {}",
        a2[0],
        b[a1.len()]
    );

    assert_eq!(
        b[a1.len() + a2.len()],
        a3[0],
        "concatenate, expected {} actual {}",
        a3[0],
        b[a1.len() + a2.len()],
    )
}

#[test]
fn test_nonzero() {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..TEST_N) + 1;
    let mut a = Arr::<f64>::zeros(n);
    a[1] = rng.gen();
    let ix = a.nonzero();
    assert_eq!(ix.len(), n, "nonzero, expected {} actual {}", n, ix.len());
    assert_eq!(ix[0], 0, "nonzero, expected {} actual {}", 0, ix[0]);
    assert_eq!(ix[1], 1, "nonzero, expected {} actual {}", 1, ix[1]);
}

#[test]
fn test_negate() {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..TEST_N) + 1;
    let a = Arr::<f64>::rand(n);
    let b = -&a;
    assert_eq!(
        b.len(),
        a.len(),
        "negate, expected {} actual {}",
        a.len(),
        b.len()
    );
    for i in 0..n {
        assert_eq!(
            b[i], -a[i],
            "negate {}, expected {} actual {}",
            i, -a[i], b[i]
        );
    }
}

#[test]
fn test_prod() {
    let mut rng = rand::thread_rng();
    let a = Arr::<f64>::with_vec(vec![rng.gen(), rng.gen()]);
    let p = a.prod();
    assert_eq!(
        p,
        a[0] * a[1],
        "prod, expected {} actual {}",
        a[0] * a[1],
        p
    );
}

#[test]
fn test_cum_sum() {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..TEST_N) + 1;
    let a = Arr::<f64>::rand(n);
    let b = a.cumsum();
    assert_eq!(
        b.len(),
        a.len(),
        "cumsum, expected {} actual {}",
        a.len(),
        b.len()
    );
    assert_eq!(b[0], a[0], "cumsum, expected {} actual {}", a[0], b[0]);
    assert_eq!(
        b[1],
        a[0] + a[1],
        "cumsum, expected {} actual {}",
        a[0] + a[1],
        b[1]
    );
}

#[test]
fn test_sort() {
    let mut rng = rand::thread_rng();
    // let n = rng.gen_range(0..TEST_N) + 1;
    let n = rng.gen_range(0..6) + 1;
    let a0 = Arr::<f64>::rand(n);
    let mut a = a0.clone();
    let ix = a.sort();
    assert_eq!(
        ix.len(),
        a.len(),
        "sort, expected {} actual {}",
        a.len(),
        ix.len()
    );
    for i in 1..n {
        assert!(
            a[i] >= a[i - 1],
            "sort, a[{}]={} < a[{}]={}",
            i,
            a[i],
            i - 1,
            a[i - 1]
        );
    }
    let mut b = a.clone();
    b.set(&ix, &a);
    for i in 0..n {
        assert_eq!(b[i], a0[i], "sort, expected {} actual {}", a0[i], b[i]);
    }
}

#[test]
fn test_sort_order() {
    let mut rng = rand::thread_rng();
    // let n = rng.gen_range(0..TEST_N) + 1;
    let n = rng.gen_range(0..6) + 1;
    let a0 = Arr::<f64>::rand(n);
    let mut a = a0.clone();
    let ix = a.sort_order(true);
    assert_eq!(
        ix.len(),
        a.len(),
        "sort, expected {} actual {}",
        a.len(),
        ix.len()
    );
    for i in 1..n {
        assert!(
            a[i] <= a[i - 1],
            "sort, a[{}]={} > a[{}]={}",
            i,
            a[i],
            i - 1,
            a[i - 1]
        );
    }
    let mut b = a.clone();
    b.set(&ix, &a);
    for i in 0..n {
        assert_eq!(b[i], a0[i], "sort, expected {} actual {}", a0[i], b[i]);
    }
}
