use std::fmt;
use std::ops::{Add, Mul};

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
