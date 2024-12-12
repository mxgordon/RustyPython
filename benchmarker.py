from datetime import datetime
import subprocess

iters = 1

def benchmark_rs():
    start = datetime.now()
    for i in range(iters):
        subprocess.call(["./target/release/RustyPython.exe", "addition.py"])
    end = datetime.now()

    return (end - start).total_seconds()

def benchmark_py():
    start = datetime.now()
    for i in range(iters):
        subprocess.call(["python", "tests/addition.py"])
    end = datetime.now()

    return (end - start).total_seconds()

def benchmark_rspy():
    start = datetime.now()
    for i in range(iters):
        subprocess.call(["./tests/rustpython.exe", "tests/addition.py"])
    end = datetime.now()

    return (end - start).total_seconds()


"""
RustPython: 3.308255s
RustyPython: 5.261442s
Python: 0.425162s
Speedup: 0.08x
"""

if __name__ == '__main__':
    rs_time = benchmark_rs()
    py_time = benchmark_py()
    rsp_time = benchmark_rspy()

    print(f"RustPython: {rsp_time}s")
    print(f"RustyPython: {rs_time}s")
    print(f"Python: {py_time}s")
    print(f"Speedup: {py_time / rs_time:.2f}x")