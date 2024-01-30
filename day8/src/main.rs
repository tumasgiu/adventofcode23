use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use regex::Regex;

fn main() {
    let data = parse("./test-input.txt").unwrap();
    assert_eq!(part1(&data), 6);

    // let data = parse("./test-input2.txt").unwrap();
    // assert_eq!(part2(&data), 6);

    let data = parse("./input.txt").unwrap();
    println!("Part 1 Answer: {}", part1(&data));
    // println!("Part 2 Answer: {}", part2(&data));
}

fn parse_line<'a>(map: &mut HashMap<String, (String, String)>, line: &str) -> Option<String> {
    let re = Regex::new(r"(\w+)\s*=\s*\(([^)]+)\)").ok()?;

    let caps = re.captures(line)?;

    let node = caps.get(1)?.as_str().to_string();

    let lr_unformatted = caps.get(2).unwrap().as_str();
    let lr_vec : Vec<&str> = lr_unformatted.split(", ").collect();
    let lr : (String, String) = (String::from(lr_vec[0]), String::from(lr_vec[1]));

    map.insert(node.clone(), lr);
    return Some(node)
}

type Data = (String, HashMap<String, (String, String)>);

fn parse(filename: &str) -> Result<Data, Box<dyn Error>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut instructions: String = "".to_string();
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if i == 0 {
            instructions = line.clone();
            continue
        }
        if line.is_empty() {
            continue
        }
        parse_line(&mut map, line.as_str());
    }

    return Ok((instructions, map))
}

fn part1(data: &Data)-> usize {
    let instructions = &data.0;
    let map = &data.1;

    let mut steps = 0;
    let start = "AAA";
    let end = "ZZZ";

    let mut current_node: &str = start;
    for char in instructions.chars().cycle() {
        steps += 1;
        let lr = map.get(current_node).unwrap();
        current_node = match char {
            'L' => lr.0.as_str(),
            'R' => lr.1.as_str(),
            _ => panic!("bad instructions!"),
        };
        if current_node == end {
            break
        }
    }

    steps
}