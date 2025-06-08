use aluminum::mat::Mat;
use rand::prelude::*;
use std::time::Instant;

pub fn run_aluminum_benchmarks(size: usize, runs: usize) {
    let mut rng = rand::thread_rng();
    let a = Mat::new(size, size, (0..size * size).map(|_| rng.r#gen()).collect());
    let b = Mat::new(size, size, (0..size * size).map(|_| rng.r#gen()).collect());
    println!("\n[aluminum] Benchmarking for {}x{} matrices:", size, size);
    benchmark_addition(&a, &b, runs);
    benchmark_multiplication(&a, &b, runs);
}

fn benchmark_addition(a: &Mat, b: &Mat, runs: usize) {
    let mut times = Vec::with_capacity(runs);
    let mut result = Mat::new(a.rows, a.cols, vec![0.0; a.rows * a.cols]);
    for _ in 0..runs {
        let start = Instant::now();
        for i in 0..a.data.len() {
            result.data[i] = a.data[i] + b.data[i];
        }
        let duration = start.elapsed().as_secs_f64();
        times.push(duration);
    }
    let mean = times.iter().sum::<f64>() / times.len() as f64;
    let std = (times.iter().map(|t| (t - mean).powi(2)).sum::<f64>() / times.len() as f64).sqrt();
    println!("{:<28} {:.2e} s ± {:.2e} s", "Addition:", mean, std);
}

fn benchmark_multiplication(a: &Mat, b: &Mat, runs: usize) {
    let mut times = Vec::with_capacity(runs);
    let mut result = Mat::new(a.rows, b.cols, vec![0.0; a.rows * b.cols]);
    for _ in 0..runs {
        let start = Instant::now();
        for i in 0..a.rows {
            for j in 0..b.cols {
                let mut sum = 0.0;
                for k in 0..a.cols {
                    sum += a.data[i * a.cols + k] * b.data[k * b.cols + j];
                }
                result.data[i * b.cols + j] = sum;
            }
        }
        let duration = start.elapsed().as_secs_f64();
        times.push(duration);
    }
    let mean = times.iter().sum::<f64>() / times.len() as f64;
    let std = (times.iter().map(|t| (t - mean).powi(2)).sum::<f64>() / times.len() as f64).sqrt();
    println!("{:<28} {:.2e} s ± {:.2e} s", "Multiplication:", mean, std);
}
