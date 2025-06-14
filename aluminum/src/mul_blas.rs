use cblas::{dgemm, sgemm, Layout, Transpose};

pub trait BlasGemm {
    fn gemm(
        m: i32,
        n: i32,
        k: i32,
        a: &[Self],
        lda: i32,
        b: &[Self],
        ldb: i32,
        c: &mut [Self],
        ldc: i32,
    ) where
        Self: Sized;
}

impl BlasGemm for f64 {
    fn gemm(
        m: i32,
        n: i32,
        k: i32,
        a: &[f64],
        lda: i32,
        b: &[f64],
        ldb: i32,
        c: &mut [f64],
        ldc: i32,
    ) {
        unsafe {
            dgemm(
                Layout::ColumnMajor,
                Transpose::None,
                Transpose::None,
                m,
                n,
                k,
                1.0, // alpha hardcoded
                a,
                lda,
                b,
                ldb,
                0.0, // beta hardcoded
                c,
                ldc,
            );
        }
    }
}

impl BlasGemm for f32 {
    fn gemm(
        m: i32,
        n: i32,
        k: i32,
        a: &[f32],
        lda: i32,
        b: &[f32],
        ldb: i32,
        c: &mut [f32],
        ldc: i32,
    ) {
        unsafe {
            sgemm(
                Layout::ColumnMajor,
                Transpose::None,
                Transpose::None,
                m,
                n,
                k,
                1.0, // alpha hardcoded
                a,
                lda,
                b,
                ldb,
                0.0, // beta hardcoded
                c,
                ldc,
            );
        }
    }
}

// You can add Complex<f32> and Complex<f64> support here as well, using cblas::cgemm and cblas::zgemm
