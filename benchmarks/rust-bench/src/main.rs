mod aluminum_bench;
mod faer_bench;
mod nalgebra_bench;

// Correct FFI bindings for your OpenBLAS
unsafe extern "C" {
    fn openblas_set_num_threads(n: i32);
    fn openblas_get_num_threads() -> i32;
}

fn main() {
    // Set number of threads *before* any computation
    unsafe {
        openblas_set_num_threads(1);
        let threads = openblas_get_num_threads();
        println!("✅ OpenBLAS num threads at runtime: {}", threads);
    }

    let sizes = [10, 50, 100, 500, 1000];
    let runs = 10;

    for &size in &sizes {
        aluminum_bench::run_aluminum_benchmarks(size, runs);
        // faer_bench::run_faer_benchmarks(size, runs);
        // nalgebra_bench::run_nalgebra_benchmarks(size, runs);
    }
}
