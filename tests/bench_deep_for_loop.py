ITERATIONS = 50
start = 257

total = 0
for i in range(start, start + ITERATIONS):
    for j in range(start, i):
        for k in range(start, j):
            for l in range(start, k):
                for n in range(start, l):
                    total += n

print(total)