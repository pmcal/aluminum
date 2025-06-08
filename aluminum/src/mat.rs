use cblas::{dgemm, Layout, Transpose};
use std::ops::{Add, Mul};

#[derive(Debug, Clone)]
pub struct Mat {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
}

impl Mat {
    pub fn new(rows: usize, cols: usize, data: Vec<f64>) -> Self {
        assert_eq!(
            data.len(),
            rows * cols,
            "Data does not match matrix dimensions"
        );
        Mat { rows, cols, data }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<f64> {
        if row < self.rows && col < self.cols {
            Some(self.data[row * self.cols + col])
        } else {
            None
        }
    }

    pub fn dot(&self, other: &Mat) -> Option<f64> {
        // Row vector (1xN) dot column vector (Nx1)
        if self.rows == 1 && other.cols == 1 && self.cols == other.rows {
            Some(self.data.iter().zip(&other.data).map(|(a, b)| a * b).sum())
        } else {
            None
        }
    }

    pub fn multiply_blas(&self, rhs: &Mat) -> Mat {
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

        let data = self
            .data
            .iter()
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

        let mut data = vec![0.0; self.rows * rhs.cols];

        for i in 0..self.rows {
            for j in 0..rhs.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.data[i * self.cols + k] * rhs.data[k * rhs.cols + j];
                }
                data[i * rhs.cols + j] = sum;
            }
        }

        Mat::new(self.rows, rhs.cols, data)
    }
}
