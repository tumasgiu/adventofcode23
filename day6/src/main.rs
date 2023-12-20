
fn main() {
    let races = vec![
        Race { duration: 40, record: 219 },
        Race { duration: 81, record: 1012 },
        Race { duration: 77, record: 1365 },
        Race { duration: 72, record: 1089 },
    ];

    println!("Part 1: {}", part1(races));

    println!("Part 2: {}", part1(vec![
        Race{
            duration: 40817772,
            record: 219101213651089,
        }
    ]));
}

struct Race {
    duration: usize,
    record: usize,
}

fn winning_speeds_count(race: &Race) -> usize {
    let mut winning_speeds = 0;
    let mut speed: usize = 0;
    loop {
        let distance_traveled = speed * (race.duration - speed);
        if distance_traveled > race.record {
            winning_speeds += 1;
        }
        speed += 1;
        if speed == race.duration {
            break
        }
    }
    winning_speeds
}

fn part1(races: Vec<Race>) -> usize {
    races.iter().map(|r| winning_speeds_count(r)).fold(1, |acc, e| acc * e)
}

#[test]
fn test_winning_speeds() {
    let races = vec![
        Race{ duration: 7, record: 9 },
        Race{ duration: 15, record: 40 },
        Race{ duration: 30, record: 200 },
    ];

    assert_eq!(288, part1(races));

    let races = vec![
        Race{ duration: 71530, record: 940200 },
    ];

    assert_eq!(71503, part1(races));
}