use rayon::prelude::*;
use std::fmt;
use std::ops::{Add, Mul};

use crate::mul_blas::BlasGemm;

#[derive(Clone)]
pub struct Mat<T> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<T>,
}

// === Mat constructors ===
impl<T> Mat<T> {
    pub fn new(rows: usize, cols: usize, data: Vec<T>) -> Self {
        Self::from_vec(rows, cols, data)
    }

    pub fn from_vec(rows: usize, cols: usize, data: Vec<T>) -> Self {
        assert_eq!(
            data.len(),
            rows * cols,
            "Data does not match matrix dimensions"
        );
        Mat { rows, cols, data }
    }

    pub fn from_fn<F>(rows: usize, cols: usize, mut f: F) -> Self
    where
        F: FnMut(usize, usize) -> T,
    {
        let mut data = Vec::with_capacity(rows * cols);
        for i in 0..rows {
            for j in 0..cols {
                data.push(f(i, j));
            }
        }
        Self::from_vec(rows, cols, data)
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.rows && col < self.cols {
            Some(&self.data[row * self.cols + col])
        } else {
            None
        }
    }
}

impl<T> Add for &Mat<T>
where
    T: Add<Output = T> + Copy + Send + Sync,
{
    type Output = Mat<T>;
    fn add(self, rhs: &Mat<T>) -> Mat<T> {
        assert_eq!(
            self.rows, rhs.rows,
            "Matrix row dimensions must match for addition"
        );
        assert_eq!(
            self.cols, rhs.cols,
            "Matrix col dimensions must match for addition"
        );
        let data: Vec<T> = self
            .data
            .par_iter()
            .zip(&rhs.data)
            .map(|(&a, &b)| a + b)
            .collect();
        Mat::new(self.rows, self.cols, data)
    }
}

impl Mul for &Mat<f64> {
    type Output = Mat<f64>;
    fn mul(self, rhs: &Mat<f64>) -> Mat<f64> {
        assert_eq!(
            self.cols, rhs.rows,
            "Matrix dimensions do not align for multiplication"
        );
        let (m, n, k) = (self.rows as i32, rhs.cols as i32, self.cols as i32);
        let mut result = vec![0.0; self.rows * rhs.cols];
        <f64 as BlasGemm>::gemm(
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
        Mat::new(self.rows, rhs.cols, result)
    }
}

impl Mul<f64> for &Mat<f64> {
    type Output = Mat<f64>;
    fn mul(self, rhs: f64) -> Mat<f64> {
        let data: Vec<f64> = self.data.par_iter().map(|a| a * rhs).collect();
        Mat::new(self.rows, self.cols, data)
    }
}

impl Mul<&Mat<f64>> for f64 {
    type Output = Mat<f64>;
    fn mul(self, rhs: &Mat<f64>) -> Mat<f64> {
        let data: Vec<f64> = rhs.data.par_iter().map(|a| self * a).collect();
        Mat::new(rhs.rows, rhs.cols, data)
    }
}

impl fmt::Display for Mat<f64> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

impl std::fmt::Debug for Mat<f64> {
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
