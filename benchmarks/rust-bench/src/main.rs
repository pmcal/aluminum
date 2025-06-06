mod faer_bench;
mod nalgebra_bench;

fn main() {
    let sizes = [10, 50, 100, 500, 1000];
    let runs = 10;

    for &size in &sizes {
        faer_bench::run_faer_benchmarks(size, runs);
        nalgebra_bench::run_nalgebra_benchmarks(size, runs);
    }
}
