mod aluminum_bench;
mod faer_bench;
mod nalgebra_bench;

fn main() {
    let sizes = [10, 50, 100, 500, 1000];
    let runs = 10;

    for &size in &sizes {
        aluminum_bench::run_aluminum_benchmarks(size, runs);
        faer_bench::run_faer_benchmarks(size, runs)
    }
}
