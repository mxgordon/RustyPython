ITERATIONS = 50

total = 0
for i in range(ITERATIONS):
    for j in range(i):
        for k in range(j):
            for l in range(k):
                for n in range(l):
                    total += n

print(total)