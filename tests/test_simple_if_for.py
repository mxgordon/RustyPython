total = 0

for i in range(1000):
    for j in range(1000):
        if i == j:
            total += j * i

print(total)