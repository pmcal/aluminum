use std::fmt;
use std::time::Instant;

pub struct Mat {
    pub data: Vec<f32>,
    pub rows: usize,
    pub cols: usize,
}

impl Mat {
    pub fn new(rows: usize, cols: usize, data: Vec<f32>) -> Self {
        assert_eq!(
            data.len(),
            rows * cols,
            "Data does not match matrix dimensions"
        );
        Mat { data, rows, cols }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<f32> {
        if row < self.rows && col < self.cols {
            Some(self.data[row * self.cols + col])
        } else {
            None
        }
    }

    pub fn dot(&self, other: &Mat) -> Option<f32> {
        // Row vector (1xN) dot column vector (Nx1)
        if self.rows == 1 && other.cols == 1 && self.cols == other.rows {
            let mut sum = 0.0;
            for i in 0..self.cols {
                sum += self.data[i] * other.data[i];
            }
            Some(sum)
        } else {
            None // Not a valid dot product
        }
    }

    pub fn bench_addition(a: &Mat, b: &Mat, runs: usize) {
        assert_eq!(a.rows, b.rows);
        assert_eq!(a.cols, b.cols);
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
        let std =
            (times.iter().map(|t| (t - mean).powi(2)).sum::<f64>() / times.len() as f64).sqrt();
        println!(
            "{:<28} {:.2e} s ± {:.2e} s",
            "Aluminum Addition:", mean, std
        );
    }

    pub fn bench_multiplication(a: &Mat, b: &Mat, runs: usize) {
        assert_eq!(a.cols, b.rows);
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
        let std =
            (times.iter().map(|t| (t - mean).powi(2)).sum::<f64>() / times.len() as f64).sqrt();
        println!(
            "{:<28} {:.2e} s ± {:.2e} s",
            "Aluminum Multiplication:", mean, std
        );
    }
}

impl fmt::Display for Mat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for row in 0..self.rows {
            if row > 0 {
                write!(f, "\n ")?;
            }
            write!(f, "[")?;
            for col in 0..self.cols {
                write!(f, "{}", self.data[row * self.cols + col])?;
                if col < self.cols - 1 {
                    write!(f, ", ")?;
                }
            }
            write!(f, "]")?;
            if row < self.rows - 1 {
                write!(f, ",")?;
            }
        }
        write!(f, "]")
    }
}
