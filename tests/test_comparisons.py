a = 12
b = 10001

c = True

d = -.1

e = 0

print(a or e)
print((c * a - b) and c * d)

assert c and e or a and d
assert (c and e) or (a and d)
assert (c and (e or a) and d)
assert not (c and not (e or a) and d)
assert not ((c and not (e or a)) and d)
