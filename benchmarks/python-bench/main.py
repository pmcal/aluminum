import numpy as np
import time
from scipy.linalg import lu, cholesky, qr, schur


def benchmark_addition(a, b, runs=10):
    # Warm up
    _ = a + b
    add_times = []
    for _ in range(runs):
        start = time.perf_counter()
        _ = a + b
        end = time.perf_counter()
        add_times.append(end - start)
    add_avg = np.mean(add_times)
    add_std = np.std(add_times)
    print(f"Addition: {add_avg:.2e} s ± {add_std:.2e} s")


def benchmark_multiplication(a, b, runs=10):
    # Warm up
    _ = a @ b
    mul_times = []
    for _ in range(runs):
        start = time.perf_counter()
        _ = a @ b
        end = time.perf_counter()
        mul_times.append(end - start)
    mul_avg = np.mean(mul_times)
    mul_std = np.std(mul_times)
    print(f"Multiplication: {mul_avg:.2e} s ± {mul_std:.2e} s")


def benchmark_inversion(a, runs=10):
    # Warm up
    _ = np.linalg.inv(a)
    inv_times = []
    for _ in range(runs):
        start = time.perf_counter()
        _ = np.linalg.inv(a)
        end = time.perf_counter()
        inv_times.append(end - start)
    inv_avg = np.mean(inv_times)
    inv_std = np.std(inv_times)
    print(f"Inversion: {inv_avg:.2e} s ± {inv_std:.2e} s")


def benchmark_determinant(a, runs=10):
    # Warm up
    _ = np.linalg.det(a)
    det_times = []
    for _ in range(runs):
        start = time.perf_counter()
        _ = np.linalg.det(a)
        end = time.perf_counter()
        det_times.append(end - start)
    det_avg = np.mean(det_times)
    det_std = np.std(det_times)
    print(f"Determinant: {det_avg:.2e} s ± {det_std:.2e} s")


def benchmark_lu_decomposition(a, runs=10):
    # Warm up
    _ = lu(a)
    lu_times = []
    for _ in range(runs):
        start = time.perf_counter()
        _ = lu(a)
        end = time.perf_counter()
        lu_times.append(end - start)
    lu_avg = np.mean(lu_times)
    lu_std = np.std(lu_times)
    print(f"LU Decomposition: {lu_avg:.2e} s ± {lu_std:.2e} s")


def benchmark_cholesky(a, runs=10):
    # Warm up
    _ = cholesky(a @ a.T + np.eye(a.shape[0]))  # Ensure positive-definite
    chol_times = []
    for _ in range(runs):
        start = time.perf_counter()
        _ = cholesky(a @ a.T + np.eye(a.shape[0]))
        end = time.perf_counter()
        chol_times.append(end - start)
    chol_avg = np.mean(chol_times)
    chol_std = np.std(chol_times)
    print(f"Cholesky Decomposition: {chol_avg:.2e} s ± {chol_std:.2e} s")


def benchmark_qr(a, runs=10):
    # Warm up
    _ = qr(a)
    qr_times = []
    for _ in range(runs):
        start = time.perf_counter()
        _ = qr(a)
        end = time.perf_counter()
        qr_times.append(end - start)
    qr_avg = np.mean(qr_times)
    qr_std = np.std(qr_times)
    print(f"QR Decomposition: {qr_avg:.2e} s ± {qr_std:.2e} s")


def benchmark_schur(a, runs=10):
    # Warm up
    _ = schur(a)
    schur_times = []
    for _ in range(runs):
        start = time.perf_counter()
        _ = schur(a)
        end = time.perf_counter()
        schur_times.append(end - start)
    schur_avg = np.mean(schur_times)
    schur_std = np.std(schur_times)
    print(f"Schur Decomposition: {schur_avg:.2e} s ± {schur_std:.2e} s")


def benchmark_svd(a, runs=10):
    # Warm up
    _ = np.linalg.svd(a, full_matrices=False)
    svd_times = []
    for _ in range(runs):
        start = time.perf_counter()
        _ = np.linalg.svd(a, full_matrices=False)
        end = time.perf_counter()
        svd_times.append(end - start)
    svd_avg = np.mean(svd_times)
    svd_std = np.std(svd_times)
    print(f"SVD: {svd_avg:.2e} s ± {svd_std:.2e} s")


def benchmark(size: int, dtype=np.float64, runs: int = 10):
    print("\n\n")
    print(f"Benchmarking for {size}x{size} matrices ({dtype})")
    print(f"Runs: {runs}")
    a = np.random.rand(size, size).astype(dtype)
    b = np.random.rand(size, size).astype(dtype)
    benchmark_addition(a, b, runs)
    benchmark_multiplication(a, b, runs)
    benchmark_inversion(a, runs)
    benchmark_lu_decomposition(a, runs)
    benchmark_cholesky(a, runs)
    benchmark_qr(a, runs)
    benchmark_schur(a, runs)
    benchmark_svd(a, runs)


def main():
    sizes = [10, 50, 100, 500, 1000, 5000, 10000]
    for size in sizes:
        benchmark(size)


if __name__ == "__main__":
    main()
