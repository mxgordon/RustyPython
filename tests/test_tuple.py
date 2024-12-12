tup = (1, "a", 3.3, None)

a, b, c, d = tup

print(a, b, c, d)
print(tup[0], tup[1], tup[2], tup[3])

for i, ele in enumerate(tup):
    print(i, c)
    
g, *h, j = tup

print(g, h, j)
