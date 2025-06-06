use faer::{FaerMat, Mat};
use rand::prelude::*;
use std::time::Instant;

pub fn run_faer_benchmarks(size: usize, runs: usize) {
    let mut rng = rand::thread_rng();
    let a = Mat::<f64>::from_fn(size, size, |_, _| rng.r#gen());
    let b = Mat::<f64>::from_fn(size, size, |_, _| rng.r#gen());
    println!("\n[faer] Benchmarking for {}x{} matrices:", size, size);
    benchmark_addition(&a, &b, runs);
    benchmark_multiplication(&a, &b, runs);
    benchmark_lu(&a, runs);
    benchmark_qr(&a, runs);

    if size < 500 {
        benchmark_svd(&a, runs);
    }
}

fn benchmark_addition(a: &Mat<f64>, b: &Mat<f64>, runs: usize) {
    let _addition = a + b;
    let mut times = Vec::with_capacity(runs);
    for _ in 0..runs {
        let start = Instant::now();
        let _addition = a + b;
        let duration = start.elapsed().as_secs_f64();
        times.push(duration);
    }
    let mean = times.iter().sum::<f64>() / times.len() as f64;
    let std = (times.iter().map(|t| (t - mean).powi(2)).sum::<f64>() / times.len() as f64).sqrt();
    println!("{:<28} {:.2e} s ± {:.2e} s", "Addition:", mean, std);
}

fn benchmark_multiplication(a: &Mat<f64>, b: &Mat<f64>, runs: usize) {
    let _multiplication = a * b;
    let mut times = Vec::with_capacity(runs);
    for _ in 0..runs {
        let start = Instant::now();
        let _multiplication = a * b;
        let duration = start.elapsed().as_secs_f64();
        times.push(duration);
    }
    let mean = times.iter().sum::<f64>() / times.len() as f64;
    let std = (times.iter().map(|t| (t - mean).powi(2)).sum::<f64>() / times.len() as f64).sqrt();
    println!("{:<28} {:.2e} s ± {:.2e} s", "Multiplication:", mean, std);
}

fn benchmark_lu(a: &Mat<f64>, runs: usize) {
    {
        let _lu = a.partial_piv_lu();
    }
    let mut times = Vec::with_capacity(runs);
    for _ in 0..runs {
        let start = Instant::now();
        {
            let _lu = a.partial_piv_lu();
        }
        let duration = start.elapsed().as_secs_f64();
        times.push(duration);
    }
    let mean = times.iter().sum::<f64>() / times.len() as f64;
    let std = (times.iter().map(|t| (t - mean).powi(2)).sum::<f64>() / times.len() as f64).sqrt();
    println!("{:<28} {:.2e} s ± {:.2e} s", "LU Decomposition:", mean, std);
}

fn benchmark_qr(a: &Mat<f64>, runs: usize) {
    {
        let _qr = a.qr();
    }
    let mut times = Vec::with_capacity(runs);
    for _ in 0..runs {
        let start = Instant::now();
        {
            let _qr = a.qr();
        }
        let duration = start.elapsed().as_secs_f64();
        times.push(duration);
    }
    let mean = times.iter().sum::<f64>() / times.len() as f64;
    let std = (times.iter().map(|t| (t - mean).powi(2)).sum::<f64>() / times.len() as f64).sqrt();
    println!("{:<28} {:.2e} s ± {:.2e} s", "QR Decomposition:", mean, std);
}

fn benchmark_svd(a: &Mat<f64>, runs: usize) {
    {
        let _svd = a.svd();
    }
    let mut times = Vec::with_capacity(runs);
    for _ in 0..runs {
        let start = Instant::now();
        {
            let _svd = a.svd();
        }
        let duration = start.elapsed().as_secs_f64();
        times.push(duration);
    }
    let mean = times.iter().sum::<f64>() / times.len() as f64;
    let std = (times.iter().map(|t| (t - mean).powi(2)).sum::<f64>() / times.len() as f64).sqrt();
    println!("{:<28} {:.2e} s ± {:.2e} s", "SVD:", mean, std);
}
