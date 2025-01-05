a = 1
b = -3

print(a * b, b * a)
print(a + b, b + a)

c = 1.1

print(a / b)

print(b / c)

a = "hello"
b = "world"

print(a + " " + b)

for letter in a:
    print(letter + " - ")

e = a == b
f = b != b
g = a > b
h = a <= b

print(e, g)
print(f, h)

j = None

print(j)
print(j is None)
