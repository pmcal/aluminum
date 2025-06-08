use cblas::{dgemm, Layout, Transpose};
use std::time::Instant;

unsafe extern "C" {
    fn openblas_set_num_threads(n: i32);
    fn openblas_get_num_threads() -> i32;
}

fn main() {
    let n = 5000;
    let threads = 4;

    println!("Logical CPUs: {}", num_cpus::get());

    unsafe {
        openblas_set_num_threads(threads);
        let active = openblas_get_num_threads();
        println!("✅ OpenBLAS threads set to: {}", active);
    }

    let a = vec![1.0f64; n * n];
    let b = vec![2.0f64; n * n];
    let mut c = vec![0.0f64; n * n];

    println!("Starting matrix multiply: {} x {}", n, n);
    let start = Instant::now();

    unsafe {
        dgemm(
            Layout::ColumnMajor,
            Transpose::None,
            Transpose::None,
            n as i32,
            n as i32,
            n as i32,
            1.0,
            &a,
            n as i32,
            &b,
            n as i32,
            0.0,
            &mut c,
            n as i32,
        );
    }

    let duration = start.elapsed().as_secs_f64();
    println!("Done in {:.3} seconds", duration);
}
