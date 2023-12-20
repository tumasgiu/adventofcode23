
fn main() {
    let races = vec![
        Race { duration: 40, record: 219 },
        Race { duration: 81, record: 1012 },
        Race { duration: 77, record: 1365 },
        Race { duration: 72, record: 1089 },
    ];

    println!("Part 1: {}", part1(races))
}

struct Race {
    duration: usize,
    record: usize,
}

fn winning_speeds(race: &Race) -> Vec<usize> {
    let mut winning_speeds = Vec::new();
    let mut speed: usize = 0;
    loop {
        let distance_traveled = speed * (race.duration - speed);
        if distance_traveled > race.record {
            winning_speeds.push(distance_traveled);
        }
        speed += 1;
        if speed == race.duration {
            break
        }
    }
    winning_speeds
}

fn part1(races: Vec<Race>) -> usize {
    races.iter().map(|r| winning_speeds(r).len()).fold(1, |acc, e| acc * e)
}

#[test]
fn test_winning_speeds() {
    let races = vec![
        Race{ duration: 7, record: 9 },
        Race{ duration: 15, record: 40 },
        Race{ duration: 30, record: 200 },
    ];

    assert_eq!(288, part1(races));
}