test_str = "Hello world!"

new_str = ""

for i in range(len(test_str)):
    char = test_str[i]

    if char.islower():
        new_str += test_str[i]

print(new_str) # elloworld

while len(new_str) > 0:
    print(new_str[-1])
    new_str = new_str[:-1]
