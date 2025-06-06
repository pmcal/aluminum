## Benchmark Results

Benchmarking was performed on $n \times n$ matrices using 64-bit floating-point precision (numpy.float64 for Python, f64 for Rust). Each operation was executed $N = 10$ times. The table below shows the mean execution time $\mu$ and standard deviation $\sigma$ across those runs, formatted as $\mu \pm \sigma$. All timings are reported in seconds.


### Python (Scipy/Numpy)

| Operation \\ $n$ | 10 | 50 | 100 | 500 | 1000 | 5000 | 10000 |
|------------------|-----|------|-------|--------|---------|---------|----------|
| Addition | $(4.21 \pm 1.47) \times 10^{-7}$ | $(7.25 \pm 0.06) \times 10^{-7}$ | $(2.84 \pm 0.03) \times 10^{-6}$ | $(1.71 \pm 0.20) \times 10^{-4}$ | $(6.90 \pm 0.46) \times 10^{-4}$ | $(2.10 \pm 0.02) \times 10^{-2}$ | $(8.38 \pm 0.12) \times 10^{-2}$ |
| Multiplication | $(1.12 \pm 0.45) \times 10^{-6}$ | $(3.34 \pm 0.15) \times 10^{-6}$ | $(1.07 \pm 0.02) \times 10^{-5}$ | $(5.43 \pm 0.32) \times 10^{-4}$ | $(4.14 \pm 0.16) \times 10^{-3}$ | $(5.22 \pm 0.03) \times 10^{-1}$ | $(3.97 \pm 0.02) \times 10^{0}$ |
| Inversion | $(5.18 \pm 1.17) \times 10^{-6}$ | $(2.97 \pm 0.20) \times 10^{-5}$ | $(9.45 \pm 0.14) \times 10^{-5}$ | $(3.28 \pm 0.13) \times 10^{-3}$ | $(1.48 \pm 0.06) \times 10^{-2}$ | $(2.25 \pm 0.02) \times 10^{0}$ | $(1.41 \pm 0.04) \times 10^{1}$ |
| LU Decomposition | $(1.01 \pm 0.28) \times 10^{-5}$ | $(3.82 \pm 0.84) \times 10^{-5}$ | $(7.81 \pm 0.28) \times 10^{-5}$ | $(2.30 \pm 0.15) \times 10^{-3}$ | $(8.20 \pm 0.22) \times 10^{-3}$ | $(8.13 \pm 0.35) \times 10^{-1}$ | $(3.29 \pm 0.18) \times 10^{0}$ |
| Cholesky Decomposition | $(8.83 \pm 0.32) \times 10^{-6}$ | $(1.93 \pm 0.24) \times 10^{-5}$ | $(4.87 \pm 0.20) \times 10^{-5}$ | $(1.61 \pm 0.13) \times 10^{-3}$ | $(8.39 \pm 0.33) \times 10^{-3}$ | $(6.64 \pm 0.12) \times 10^{-1}$ | $(6.12 \pm 0.12) \times 10^{0}$ |
| QR Decomposition | $(1.64 \pm 0.39) \times 10^{-5}$ | $(6.08 \pm 0.45) \times 10^{-5}$ | $(2.28 \pm 0.30) \times 10^{-4}$ | $(7.00 \pm 0.33) \times 10^{-3}$ | $(3.78 \pm 0.18) \times 10^{-2}$ | $(3.59 \pm 0.04) \times 10^{0}$ | $(2.03 \pm 0.06) \times 10^{1}$ |
| Schur Decomposition | $(2.20 \pm 0.30) \times 10^{-5}$ | $(4.24 \pm 0.22) \times 10^{-4}$ | $(4.49 \pm 0.10) \times 10^{-3}$ | $(8.20 \pm 0.13) \times 10^{-2}$ | $(4.05 \pm 0.04) \times 10^{-1}$ | $(1.81 \pm 0.06) \times 10^{1}$ | $(1.19 \pm 0.59) \times 10^{2}$ |
| SVD | $(2.21 \pm 0.26) \times 10^{-5}$ | $(2.53 \pm 0.05) \times 10^{-4}$ | $(9.10 \pm 0.27) \times 10^{-4}$ | $(3.60 \pm 0.10) \times 10^{-2}$ | $(1.87 \pm 0.02) \times 10^{-1}$ | $(2.76 \pm 0.37) \times 10^{1}$ | $(2.07 \pm 0.25) \times 10^{2}$ |

### Rust

`faer`
| Operation \ $n$ | 10 | 50 | 100 | 500 | 1000 |
| ----------------- | ---------------------------------- | ---------------------------------- | ---------------------------------- | ---------------------------------- | ---------------------------------- |
| Addition          | $(8.84 \pm 0.09) \times 10^{-6}$ | $(2.01 \pm 0.02) \times 10^{-4}$ | $(6.58 \pm 0.09) \times 10^{-4}$ | $(1.62 \pm 0.02) \times 10^{-2}$ | $(6.44 \pm 0.04) \times 10^{-2}$ |
| Multiplication    | $(3.02 \pm 0.16) \times 10^{-5}$ | $(1.06 \pm 0.02) \times 10^{-3}$ | $(1.72 \pm 0.09) \times 10^{-3}$ | $(1.12 \pm 0.03) \times 10^{-1}$ | $(8.30 \pm 0.10) \times 10^{-1}$ |
| LU Decomposition  | $(2.96 \pm 0.29) \times 10^{-4}$ | $(2.47 \pm 0.05) \times 10^{-3}$ | $(9.75 \pm 0.07) \times 10^{-3}$ | $(2.05 \pm 0.01) \times 10^{-1}$ | $(8.46 \pm 0.07) \times 10^{-1}$ |
| QR Decomposition  | $(4.07 \pm 0.21) \times 10^{-4}$ | $(3.71 \pm 0.03) \times 10^{-3}$ | $(1.32 \pm 0.27) \times 10^{-2}$ | $(2.77 \pm 0.04) \times 10^{-1}$ | $(1.36 \pm 0.04) \times 10^{0}$  |
| SVD               | $(3.14 \pm 0.40) \times 10^{-3}$ | $(2.08 \pm 0.10) \times 10^{-2}$ | $(9.40 \pm 0.23) \times 10^{-2}$ | Stack overflow                                | Stack overflow                           |





`nalgebra`:

| Operation \ $n$ | 10 | 50 | 100 | 500 | 1000  |
| ----------------- | ---------------------------------- | ---------------------------------- | ---------------------------------- | ---------------------------------- | ---------------------------------- |
| Addition          | $(3.92 \pm 0.10) \times 10^{-6}$ | $(6.73 \pm 0.15) \times 10^{-5}$ | $(2.69 \pm 0.02) \times 10^{-4}$ | $(7.18 \pm 0.27) \times 10^{-3}$ | $(2.78 \pm 0.08) \times 10^{-2}$ |
| Multiplication    | $(2.86 \pm 0.03) \times 10^{-5}$ | $(8.82 \pm 0.04) \times 10^{-4}$ | $(5.67 \pm 0.04) \times 10^{-3}$ | $(6.57 \pm 0.17) \times 10^{-1}$ | $(5.12 \pm 0.04) \times 10^{0}$  |
| LU Decomposition  | $(2.99 \pm 0.13) \times 10^{-5}$ | $(1.30 \pm 0.03) \times 10^{-3}$ | $(8.62 \pm 0.05) \times 10^{-3}$ | $(9.19 \pm 0.13) \times 10^{-1}$ | $(7.17 \pm 0.04) \times 10^{0}$  |
| QR Decomposition  | $(5.38 \pm 0.02) \times 10^{-5}$ | $(2.59 \pm 0.03) \times 10^{-3}$ | $(1.79 \pm 0.03) \times 10^{-2}$ | $(1.97 \pm 0.02) \times 10^{0}$  | $(1.55 \pm 0.04) \times 10^{1}$  |
| SVD               | $(5.21 \pm 0.01) \times 10^{-4}$ | $(2.64 \pm 0.04) \times 10^{-2}$ | $(1.89 \pm 0.02) \times 10^{-1}$ | $(2.02 \pm 0.04) \times 10^{1}$  | $(1.59 \pm 0.01) \times 10^{2}$ |


### Comparison

#### $n = 100$
| Operation        | NumPy                              | faer                               | nalgebra                           |
| ---------------- | ---------------------------------- | ---------------------------------- | ---------------------------------- |
| Addition         | $(2.84 \pm 0.03) \times 10^{-6}$ | $(6.58 \pm 0.09) \times 10^{-4}$ | $(2.69 \pm 0.02) \times 10^{-4}$ |
| Multiplication   | $(1.07 \pm 0.02) \times 10^{-5}$ | $(1.72 \pm 0.09) \times 10^{-3}$ | $(5.67 \pm 0.04) \times 10^{-3}$ |
| LU Decomposition | $(7.81 \pm 0.28) \times 10^{-5}$ | $(9.75 \pm 0.07) \times 10^{-3}$ | $(8.62 \pm 0.05) \times 10^{-3}$ |
| QR Decomposition | $(2.28 \pm 0.30) \times 10^{-4}$ | $(1.32 \pm 0.27) \times 10^{-2}$ | $(1.79 \pm 0.03) \times 10^{-2}$ |
| SVD              | $(9.10 \pm 0.27) \times 10^{-4}$ | $(9.40 \pm 0.23) \times 10^{-2}$ | $(1.89 \pm 0.02) \times 10^{-1}$ |

---

#### $n = 500$
| Operation        | NumPy                              | faer                               | nalgebra                           |
| ---------------- | ---------------------------------- | ---------------------------------- | ---------------------------------- |
| Addition         | $(1.71 \pm 0.20) \times 10^{-4}$ | $(1.62 \pm 0.02) \times 10^{-2}$ | $(7.18 \pm 0.27) \times 10^{-3}$ |
| Multiplication   | $(5.43 \pm 0.32) \times 10^{-4}$ | $(1.12 \pm 0.03) \times 10^{-1}$ | $(6.57 \pm 0.17) \times 10^{-1}$ |
| LU Decomposition | $(2.30 \pm 0.15) \times 10^{-3}$ | $(2.05 \pm 0.01) \times 10^{-1}$ | $(9.19 \pm 0.13) \times 10^{-1}$ |
| QR Decomposition | $(7.00 \pm 0.33) \times 10^{-3}$ | $(2.77 \pm 0.04) \times 10^{-1}$ | $(1.97 \pm 0.02) \times 10^{0}$  |
| SVD              | $(3.60 \pm 0.10) \times 10^{-2}$ | Stack overflow                     | $(2.02 \pm 0.04) \times 10^{1}$  |
    
---

#### $n = 1000$
| Operation        | NumPy                              | faer                               | nalgebra                           |
| ---------------- | ---------------------------------- | ---------------------------------- | ---------------------------------- |
| Addition         | $(6.90 \pm 0.46) \times 10^{-4}$ | $(6.44 \pm 0.04) \times 10^{-2}$ | $(2.78 \pm 0.08) \times 10^{-2}$ |
| Multiplication   | $(4.14 \pm 0.16) \times 10^{-3}$ | $(8.30 \pm 0.10) \times 10^{-1}$ | $(5.12 \pm 0.04) \times 10^{0}$  |
| LU Decomposition | $(8.20 \pm 0.22) \times 10^{-3}$ | $(8.46 \pm 0.07) \times 10^{-1}$ | $(7.17 \pm 0.04) \times 10^{0}$  |
| QR Decomposition | $(3.78 \pm 0.18) \times 10^{-2}$ | $(1.36 \pm 0.04) \times 10^{0}$  | $(1.55 \pm 0.04) \times 10^{1}$  |
| SVD              | $(1.87 \pm 0.02) \times 10^{-1}$ | Stack overflow                     | $(1.59 \pm 0.01) \times 10^{2}$  |
