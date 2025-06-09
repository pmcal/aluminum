use cblas::{dgemm, Layout, Transpose};
use rayon::prelude::*;
use std::fmt;
use std::ops::{Add, Mul};

#[derive(Clone)]
pub struct Mat {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
}

// === Mat constructors ===
impl Mat {
    pub fn new(rows: usize, cols: usize, data: Vec<f64>) -> Self {
        Self::from_vec(rows, cols, data)
    }

    pub fn from_vec(rows: usize, cols: usize, data: Vec<f64>) -> Self {
        assert_eq!(
            data.len(),
            rows * cols,
            "Data does not match matrix dimensions"
        );
        Mat { rows, cols, data }
    }

    pub fn from_fn<F>(rows: usize, cols: usize, mut f: F) -> Self
    where
        F: FnMut(usize, usize) -> f64,
    {
        let mut data = Vec::with_capacity(rows * cols);
        for i in 0..rows {
            for j in 0..cols {
                data.push(f(i, j));
            }
        }
        Self::from_vec(rows, cols, data)
    }

    pub fn get(&self, row: usize, col: usize) -> Option<f64> {
        if row < self.rows && col < self.cols {
            Some(self.data[row * self.cols + col])
        } else {
            None
        }
    }
}

impl Add for &Mat {
    type Output = Mat;
    fn add(self, rhs: &Mat) -> Mat {
        assert_eq!(
            self.rows, rhs.rows,
            "Matrix row dimensions must match for addition"
        );
        assert_eq!(
            self.cols, rhs.cols,
            "Matrix col dimensions must match for addition"
        );

        let data: Vec<f64> = self
            .data
            .par_iter()
            .zip(&rhs.data)
            .map(|(a, b)| a + b)
            .collect();

        Mat::new(self.rows, self.cols, data)
    }
}

impl Mul for &Mat {
    type Output = Mat;
    fn mul(self, rhs: &Mat) -> Mat {
        assert_eq!(
            self.cols, rhs.rows,
            "Matrix dimensions do not align for multiplication"
        );
        let (m, n, k) = (self.rows as i32, rhs.cols as i32, self.cols as i32);
        let mut result = vec![0.0; self.rows * rhs.cols];
        unsafe {
            dgemm(
                Layout::ColumnMajor,
                Transpose::None,
                Transpose::None,
                m,
                n,
                k,
                1.0,
                &self.data,
                m,
                &rhs.data,
                k,
                0.0,
                &mut result,
                m,
            );
        }
        Mat::new(self.rows, rhs.cols, result)
    }
}

impl fmt::Display for Mat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use the requested precision, or default to 3
        let precision = f.precision().unwrap_or(3);
        for i in 0..self.rows {
            for j in 0..self.cols {
                let value = self.data[i * self.cols + j];
                write!(f, "{:.*} ", precision, value)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl std::fmt::Debug for Mat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let precision = f.precision().unwrap_or(3);
        writeln!(f, "[")?;
        for i in 0..self.rows {
            let start = i * self.cols;
            let end = start + self.cols;
            write!(f, "  [")?;
            for (j, value) in self.data[start..end].iter().enumerate() {
                if j > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:.*}", precision, value)?;
            }
            writeln!(f, "],")?;
        }
        write!(f, "]")
    }
}
