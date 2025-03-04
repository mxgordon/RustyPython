import os
from pathlib import Path

def get_rs_files(path: Path):
    files = []
    
    for dirpath, dirname, filenames in os.walk(path):
        for filename in filenames:
            if filename.endswith('.rs'):
                files.append(dirpath + "/" + filename)
            
    return files

def count_lines(file):
    with open(file, 'r') as f:
        return len(f.read().replace("  ", " ").replace("\n\n", "\n").split("\n"))

if __name__ == '__main__':
    files = get_rs_files(Path('.'))
    total_lines = sum(count_lines(file) for file in files)
    print(f'Total lines of code: {total_lines}')