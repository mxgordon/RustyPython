from datetime import datetime
import subprocess

iters = 1

def benchmark_rs():
    start = datetime.now()
    for i in range(iters):
        subprocess.call(["./target/release/RustyPython.exe", "test_addition.py"])
    end = datetime.now()

    return (end - start).total_seconds()

def benchmark_py():
    start = datetime.now()
    for i in range(iters):
        subprocess.call(["python", "tests/test_addition.py"])
    end = datetime.now()

    return (end - start).total_seconds()

def benchmark_rspy():
    start = datetime.now()
    for i in range(iters):
        subprocess.call(["./tests/rustpython.exe", "tests/test_addition.py"])
    end = datetime.now()

    return (end - start).total_seconds()


"""
RustPython: 2.809106s
RustyPython: 1.787605s
CPython: 0.382912s
Speedup: 0.21x

flamegraph samples: 46,512
flamegraph samples: 38,965
flamegraph samples: 33,060
flamegraph samples: 26,436
flamegraph samples: 24,913
flamegraph samples: 16,219
"""

if __name__ == '__main__':
    rs_time = benchmark_rs()
    py_time = benchmark_py()
    rsp_time = benchmark_rspy()

    print(f"RustPython: {rsp_time}s")
    print(f"RustyPython: {rs_time}s")
    print(f"CPython: {py_time}s")
    print(f"Speedup: {py_time / rs_time:.2f}x")