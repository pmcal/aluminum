pub mod mat;
#[macro_use]
mod macros;

use crate::mat::Mat;

fn main() {
    let rv = rvec![1, 2, 3, 4];
    println!("Row vector:\n{}", rv);
    let cv = cvec![1, 2, 3, 4];
    println!("Column vector:\n{}", cv);
    if let Some(dot) = rv.dot(&cv) {
        println!("Dot product: {}", dot);
    } else {
        println!("Dot product not defined for these shapes");
    }
}
