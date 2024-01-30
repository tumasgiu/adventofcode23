def first_digit(line):
    for char in line:
        try:
            int(char)
            return char
        except:
            pass


def last_digit(line):
    return first_digit(reversed(line))


res = 0
with open('input.txt') as f:
    line = f.readline()
    while line:
        fd = first_digit(line)
        if fd:
            val = fd + last_digit(line)
            res += int(val)
            line = f.readline()

print(res)
