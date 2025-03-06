from datetime import datetime
import subprocess

iters = 1

def benchmark_rusty(test):
    start = datetime.now()
    for i in range(iters):
        subprocess.call(["./target/release/RustyPython.exe", test])
    end = datetime.now()

    return (end - start).total_seconds()

def benchmark_py(test):
    start = datetime.now()
    for i in range(iters):
        subprocess.call(["python", f"tests/{test}"])
    end = datetime.now()

    return (end - start).total_seconds()

def benchmark_rs(test):
    start = datetime.now()
    for i in range(iters):
        subprocess.call(["./tests/rustpython.exe", f"tests/{test}"])
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


1.58 1.85
1.26 1.55
1.09 1.50
1.33 1.59
1.55 1.84

1.76
1.52
1.68
1.47
1.55

1.46
1.44
1.49
1.44
1.72

1.44
1.55
1.57
1.51
1.67
"""

if __name__ == '__main__':
    test_file = "test_while_add.py"

    rusty_time = benchmark_rusty(test_file)
    rs_time = benchmark_rs(test_file)
    py_time = benchmark_py(test_file)

    print(f"RustyPython: {rusty_time}s")
    print(f"RustPython: {rs_time}s" )
    print(f"CPython: {py_time}s")
    print(f"{rusty_time/py_time :.2f}x slower than CPython")
    print(f"{rs_time/rusty_time :.2f}x faster than RustPython")