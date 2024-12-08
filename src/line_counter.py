import os
from pathlib import Path

def get_rs_files(path: Path):
    files = []
    
    for name in os.listdir(path):
        if os.path.isdir(name):
            files += get_rs_files(path / name)
        elif name.endswith('.rs'):
            files.append(path / name)
            
    return files

def count_lines(file):
    with open(file, 'r') as f:
        return len(f.read().replace("  ", " ").replace("\n\n", "\n").split("\n"))

if __name__ == '__main__':
    files = get_rs_files(Path('.'))
    total_lines = sum(count_lines(file) for file in files)
    print(f'Total lines of code: {total_lines}')