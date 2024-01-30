import re

tokens = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine"
]


def reverse(token):
    return token[::-1]


reversed_tokens = list(map(reverse, tokens))

token_regex = re.compile(f"(\d|{"|".join(tokens)})")
reversed_token_regex = re.compile(f"(\d|{"|".join(reversed_tokens)})")


def find(line, regex, tokens):
    if regex.search(line):
        val = regex.search(line).group()
        try:
            int(val)
            return val
        except ValueError:
            return str(tokens.index(val) + 1)


def find_first(line):
    return find(line, token_regex, tokens)


def find_last(line):
    line = line[::-1]
    return find(line, reversed_token_regex, reversed_tokens)


def compute(filepath):
    res = 0
    with open(filepath, 'r') as f:
        line = f.readline()
        while line:
            fd = find_first(line)
            if fd:
                val = fd + find_last(line)
                res += int(val)
                line = f.readline()
    return res


if __name__ == "__main__":
    assert compute('./test_input.txt') == 281, 'Should be 281'
    print(compute('input.txt'))
