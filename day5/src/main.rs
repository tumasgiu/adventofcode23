use std::collections::HashMap;
use std::fs;
use std::ops::RangeInclusive;
use lyn::{Error, Scanner};

fn main() {
    let al = parse("./input.txt").unwrap();

    let mut ii = al.instructions();
    ii.sort_by(|x, y| x.location.cmp(&y.location));
    let lowest = ii.get(0).unwrap();

    println!("Part 1 Answer: {:?}", lowest);

    println!("Part 2 Answer: {:?}", al.part2());
}

#[derive(PartialEq, Eq, Debug)]
struct Almanac {
    seeds: Vec<usize>,
    seed_soil: VirtualMap,
    soil_fertilizer: VirtualMap,
    fertilizer_water: VirtualMap,
    water_light: VirtualMap,
    light_temp: VirtualMap,
    temp_humidity: VirtualMap,
    humidity_location: VirtualMap,
}

impl Almanac {
    fn instructions(&self) -> Vec<Instruction> {
        let mut v: Vec<Instruction> = Vec::new();
        for seed in &self.seeds {
            let seed = *seed;
            let soil = self.seed_soil.get(&seed);
            let fertilizer = self.soil_fertilizer.get(&soil);
            let water = self.fertilizer_water.get(&fertilizer);
            let light = self.water_light.get(&water);
            let temperature = self.light_temp.get(&light);
            let humidity = self.temp_humidity.get(&temperature);
            let location = self.humidity_location.get(&humidity);

            v.push(Instruction {
                seed,
                soil,
                fertilizer,
                water,
                light,
                temperature,
                humidity,
                location,
            })
        }
        v
    }

    fn seed_ranges(&self) -> Vec<RangeInclusive<usize>> {
        let mut rr = Vec::new();
        for chunk in self.seeds.chunks(2) {
            match chunk {
                &[start, len] => {
                    let r = start..=start+len;
                    rr.push(r)
                }
                _ => {
                    eprintln!("Incomplete pair!");
                }
            }
        }
        rr
    }

    fn get_location(&self, seed: &usize) -> usize {
        let soil = self.seed_soil.get(&seed);
        let fertilizer = self.soil_fertilizer.get(&soil);
        let water = self.fertilizer_water.get(&fertilizer);
        let light = self.water_light.get(&water);
        let temperature = self.light_temp.get(&light);
        let humidity = self.temp_humidity.get(&temperature);
        self.humidity_location.get(&humidity)
    }

    fn seed_for_loc(&self, loc: usize) -> Option<usize> {
        let humidity = self.humidity_location.rget(&loc);
        let temp = self.temp_humidity.rget(&humidity);
        let light = self.light_temp.rget(&temp);
        let water = self.water_light.rget(&light);
        let fertilizer = self.fertilizer_water.rget(&water);
        let soil = self.soil_fertilizer.rget(&fertilizer);
        let seed = self.seed_soil.rget(&soil);
        for r in self.seed_ranges() {
            if r.contains(&seed) {
                return Some(seed)
            }
        }
        None
    }

    fn part2(&self) -> usize {
        let mut i = 0;
        loop {
            let seed = self.seed_for_loc(i);
            if seed.is_some() {
                return i
            }
            i += 1;
        }
    }
}




#[derive(PartialEq, Eq, Debug)]
struct Instruction {
    seed: usize,
    soil: usize,
    fertilizer: usize,
    water: usize,
    light: usize,
    temperature: usize,
    humidity: usize,
    location: usize,
}

fn pop_until_alpha(scanner: &mut Scanner) {
    while scanner.peek().is_some_and(|c| !c.is_alphanumeric()) {
        scanner.pop();
    };
}

// takes the whole line
fn map_identifier(scanner: &mut Scanner) -> Option<String> {
    let mut buf = String::new();
    while scanner.peek().is_some_and(|c| c != &' ') {
        buf.push(*scanner.pop().unwrap())
    }
    consume(scanner, " map:\n");
    if buf.len() > 0 {
        return Some(buf);
    }
    None
}

fn maps(scanner: &mut Scanner) -> Result<HashMap<String, VirtualMap>, Error> {
    let mut r: HashMap<String, VirtualMap> = HashMap::new();

    while !scanner.is_done() {
        pop_until_alpha(scanner);
        let id = map_identifier(scanner);
        let mut m = VirtualMap::new();

        while scanner.peek().is_some_and(|c| c != &'\n') {
            let range_def = number_list(scanner);
            insert_range(&mut m, range_def[0], range_def[1], range_def[2]);
        }

        r.insert(id.unwrap().clone(), m);
    }

    return Ok(r);
}

fn parse(filename: &str) -> Result<Almanac, Error> {
    let contents = match fs::read_to_string(filename) {
        Ok(content) => { content }
        Err(e) => { panic!("Error reading file: {}", e); }
    };

    let mut scanner = Scanner::new(contents.as_str());
    let seeds = seeds(&mut scanner);
    let mut maps = maps(&mut scanner)?;

    return Ok(Almanac {
        seeds,
        seed_soil: maps.remove("seed-to-soil").unwrap(),
        soil_fertilizer: maps.remove("soil-to-fertilizer").unwrap(),
        fertilizer_water: maps.remove("fertilizer-to-water").unwrap(),
        water_light: maps.remove("water-to-light").unwrap(),
        light_temp: maps.remove("light-to-temperature").unwrap(),
        temp_humidity: maps.remove("temperature-to-humidity").unwrap(),
        humidity_location: maps.remove("humidity-to-location").unwrap(),
    });
}

fn number_list(scanner: &mut Scanner) -> Vec<usize> {
    let mut v: Vec<usize> = Vec::new();
    let mut number_buf = String::new();
    loop {
        let c = scanner.pop();
        match c {
            None => {
                if !number_buf.is_empty() {
                    v.push(number_buf.parse::<usize>().unwrap());
                }
                break
            }
            Some(char) => {
                if char.is_numeric() {
                    number_buf.push(*char);
                } else {
                    if !number_buf.is_empty() {
                        v.push(number_buf.parse::<usize>().unwrap());
                        number_buf = String::new();
                    }
                    if char == &'\n' {
                        break;
                    }
                }
            }
        };
    }

    v
}

fn consume(scanner: &mut Scanner, str: &str) -> bool {
    for char in str.chars() {
        if !scanner.take(&char) {
            return false;
        }
    }
    return true;
}

fn seeds(scanner: &mut Scanner) -> Vec<usize> {
    consume(scanner, "seeds:");
    number_list(scanner)
}

fn insert_range(map: &mut VirtualMap, destination_start: usize, source_start: usize, length: usize) {
    map.insert(source_start, length, destination_start);
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Range {
    source_range: RangeInclusive<usize>,
    destination_range: RangeInclusive<usize>,
    // destination_start: usize,
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct VirtualMap {
    ranges: Vec<Range>
}

impl VirtualMap {
    fn new() -> Self {
        Self {
            ranges: vec![],
        }
    }

    fn insert(&mut self, source_start: usize, length: usize, destination_start: usize) {
        let source_range = source_start..=source_start + length;
        let destination_range = destination_start..=destination_start + length;
        self.ranges.push(Range{
            source_range,
            destination_range,
        });
    }

    fn get(&self, query: &usize) -> usize {
        for r in &self.ranges {
            if r.source_range.contains(query) {
                return r.destination_range.start() + (query - r.source_range.start())
            }
        }
        query.clone()
    }

    fn rget(&self, query: &usize) -> usize {
        for r in &self.ranges {
            if r.destination_range.contains(query) {
                return r.source_range.start() + (query - r.destination_range.start())
            }
        }
        query.clone()
    }
}


fn expected_almanac() -> Almanac {
    let mut seed_soil = VirtualMap::new();
    insert_range(&mut seed_soil, 50, 98, 2);
    insert_range(&mut seed_soil, 52, 50, 48);

    let mut soil_fertilizer = VirtualMap::new();
    insert_range(&mut soil_fertilizer, 0, 15, 37);
    insert_range(&mut soil_fertilizer, 37, 52, 2);
    insert_range(&mut soil_fertilizer, 39, 0, 15);

    let mut fertilizer_water = VirtualMap::new();
    insert_range(&mut fertilizer_water, 49, 53, 8);
    insert_range(&mut fertilizer_water, 0, 11, 42);
    insert_range(&mut fertilizer_water, 42, 0, 7);
    insert_range(&mut fertilizer_water, 57, 7, 4);

    let mut water_light = VirtualMap::new();
    insert_range(&mut water_light, 88, 18, 7);
    insert_range(&mut water_light, 18, 25, 70);

    let mut light_temp = VirtualMap::new();
    insert_range(&mut light_temp, 45, 77, 23);
    insert_range(&mut light_temp, 81, 45, 19);
    insert_range(&mut light_temp, 68, 64, 13);

    let mut temp_humidity = VirtualMap::new();
    insert_range(&mut temp_humidity, 0, 69, 1);
    insert_range(&mut temp_humidity, 1, 0, 69);

    let mut humidity_location = VirtualMap::new();
    insert_range(&mut humidity_location, 60, 56, 37);
    insert_range(&mut humidity_location, 56, 93, 4);


    Almanac {
        seeds: vec![79, 14, 55, 13],
        seed_soil,
        soil_fertilizer,
        fertilizer_water,
        water_light,
        light_temp,
        temp_humidity,
        humidity_location,
    }
}


#[test]
fn test_parse() {
    let expected_almanac = expected_almanac();

    let almanac = parse("./test-input.txt").unwrap();

    assert_eq!(almanac, expected_almanac);
}


#[test]
fn test_instructions() {
    let almanac = expected_almanac();

    let expected_instructions = vec![
        Instruction {
            seed: 79,
            soil: 81,
            fertilizer: 81,
            water: 81,
            light: 74,
            temperature: 78,
            humidity: 78,
            location: 82,
        },
        Instruction {
            seed: 14,
            soil: 14,
            fertilizer: 53,
            water: 49,
            light: 42,
            temperature: 42,
            humidity: 43,
            location: 43,
        },
        Instruction {
            seed: 55,
            soil: 57,
            fertilizer: 57,
            water: 53,
            light: 46,
            temperature: 82,
            humidity: 82,
            location: 86,
        },
        Instruction {
            seed: 13,
            soil: 13,
            fertilizer: 52,
            water: 41,
            light: 34,
            temperature: 34,
            humidity: 35,
            location: 35,
        },
    ];

    let instructions = almanac.instructions();

    assert_eq!(instructions, expected_instructions)
}