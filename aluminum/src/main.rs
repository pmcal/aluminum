mod mat;
use mat::Mat;

fn main() {
    let a = Mat::new(2, 2, vec![1.0, 3.0, 2.0, 4.0]);
    let b = Mat::new(2, 2, vec![5.0, 7.0, 6.0, 8.0]);
    let c = a.multiply_blas(&b);
    println!("C = {:?}", c.data);
}
