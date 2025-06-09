use aluminum::mat::Mat;

fn main() {
    // Initialize using new
    let a = Mat::new(2, 2, vec![1.0, 3.0, 2.0, 4.0]);
    println!("a (new), default precision:\n{:.0?}", a);

    // Initialize using from_vec
    let b = Mat::from_vec(2, 2, vec![5.0, 7.0, 6.0, 8.0]);
    println!("b (from_vec):\n{}", b);

    // Initialize using from_fn
    let c = Mat::from_fn(2, 2, |i, j| (i + j) as f64);
    println!("c (from_fn):\n{}", c);

    // Addition
    let ab_sum = &a + &b;
    println!("a + b =\n{}", ab_sum);

    // Multiplication
    let ab_prod = &a * &b;
    println!("a * b =\n{}", ab_prod);

    // Multiplication with from_fn
    let ac_prod = &a * &c;
    println!("a * c =\n{}", ac_prod);

    // Scalar multiplication
    let scalar_prod = 2.5 * &a;
    println!("a * 2.5 =\n{}", scalar_prod);

    // f32 test
    let a32 = Mat::new(2, 2, vec![1.0f32, 3.0, 2.0, 4.0]);
    let b32 = Mat::from_vec(2, 2, vec![5.0f32, 7.0, 6.0, 8.0]);
    let ab32_sum = &a32 + &b32;
    println!("a32 + b32 =\n{:?}", ab32_sum);
    let ab32_prod = &a32 * &b32;
    println!("a32 * b32 =\n{:?}", ab32_prod);

    // f64 test
    let a64 = Mat::new(2, 2, vec![1.0f64, 3.0, 2.0, 4.0]);
    let b64 = Mat::from_vec(2, 2, vec![5.0f64, 7.0, 6.0, 8.0]);
    let ab64_sum = &a64 + &b64;
    println!("a64 + b64 =\n{:?}", ab64_sum);
    let ab64_prod = &a64 * &b64;
    println!("a64 * b64 =\n{:?}", ab64_prod);
}
