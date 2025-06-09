use num_traits::Zero;
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

// === Addition ===
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

impl<T> Mul for &Mat<T>
where
    T: BlasGemm + Copy + Zero,
{
    type Output = Mat<T>;
    fn mul(self, rhs: &Mat<T>) -> Mat<T> {
        assert_eq!(
            self.cols, rhs.rows,
            "Matrix dimensions do not align for multiplication"
        );
        let (m, n, k) = (self.rows as i32, rhs.cols as i32, self.cols as i32);
        let mut result = vec![T::zero(); self.rows * rhs.cols];
        <T as BlasGemm>::gemm(m, n, k, &self.data, m, &rhs.data, k, &mut result, m);
        Mat::new(self.rows, rhs.cols, result)
    }
}

// === Scalar multiplication (scalar before and after matrix) ===
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

// === Display trait ===
impl<T> fmt::Display for Mat<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let precision = f.precision();
        for i in 0..self.rows {
            for j in 0..self.cols {
                let value = &self.data[i * self.cols + j];
                if let Some(p) = precision {
                    write!(f, "{:.*} ", p, value)?;
                } else {
                    write!(f, "{} ", value)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// === Debug trait ===
impl<T> fmt::Debug for Mat<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let precision = f.precision();
        writeln!(f, "[")?;
        for i in 0..self.rows {
            let start = i * self.cols;
            let end = start + self.cols;
            write!(f, "  [")?;
            for (j, value) in self.data[start..end].iter().enumerate() {
                if j > 0 {
                    write!(f, ", ")?;
                }
                if let Some(p) = precision {
                    write!(f, "{:.*?}", p, value)?;
                } else {
                    write!(f, "{:?}", value)?;
                }
            }
            writeln!(f, "],")?;
        }
        write!(f, "]")
    }
}
