import re
from collections import namedtuple

# RGB
BAG_CONFIG = (12, 13, 14)

id_regex = re.compile(r'Game (?P<id>\d+):')
pattern = re.compile(r'(\d+)\s+(blue|green|red)')

Turn = namedtuple('Turn', ['r', 'g', 'b'])
Game = namedtuple('Game', ['id', 'turns'])


def is_possible(game):
    for turn in game.turns:
        if turn.r > BAG_CONFIG[0] or turn.g > BAG_CONFIG[1] or turn.b > BAG_CONFIG[2]:
            return False
    return True


def parse_game(line):
    id = id_regex.search(line).group('id')
    groups = line.split(';')
    turns = list()
    for turn in groups:
        matches = pattern.findall(turn)
        red = sum(int(m[0]) for m in matches if m[1] == 'red')
        green = sum(int(m[0]) for m in matches if m[1] == 'green')
        blue = sum(int(m[0]) for m in matches if m[1] == 'blue')
        turns.append(Turn(red, green, blue))
    return Game(int(id), turns)


def parse_games(filepath):
    with open(filepath, 'r') as file:
        return [parse_game(line) for line in file]


def game_power(game):
    rmax, gmax, bmax = 0, 0, 0
    for turn in game.turns:
        rmax, gmax, bmax = max(rmax, turn.r), max(gmax, turn.g), max(bmax, turn.b)
    return rmax * gmax * bmax


if __name__ == "__main__":
    expected_possible_ids = [1, 2, 5]
    games = parse_games('test_input.txt')
    possible_games = filter(is_possible, games)
    possible_ids = [game.id for game in possible_games]
    assert possible_ids == expected_possible_ids, f'Should be {expected_possible_ids}'

    expected_powers = [48, 12, 1560, 630, 36]
    powers = [game_power(game) for game in games]
    assert powers == expected_powers, f'Should be {expected_powers}'

    games = parse_games('input.txt')
    possible_games = filter(is_possible, games)
    possible_ids = [game.id for game in possible_games]
    print(f'possible_games: {possible_ids}')
    print(f'answer: {sum(possible_ids)}')

    powers = [game_power(game) for game in games]
    print(powers)
    print(f'answer: {sum(powers)}')
