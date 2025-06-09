mod mat;
use mat::Mat;

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
}
