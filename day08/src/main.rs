use serde::{de::value, Deserialize, Serialize};
use serde_json::{Result};
use std::{clone, collections::HashMap, fs, iter::Map, ops::Add, vec};

#[derive(Deserialize, Serialize)]
struct Input {
    values: Vec<String>
}

#[derive(Debug)]
struct Coordinates {
    x: i64,
    y: i64
}

impl Coordinates {
    fn new(x: i64, y: i64) -> Coordinates {
        Coordinates { x: x, y: y }
    }

    fn add(&mut self, other: Coordinates) {
        self.x += other.x;
        self.y += other.y;
    }

    fn distance(&self, other: &Coordinates) -> Coordinates {
        Coordinates {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }

    fn is_contained(&self, width: u64, height: u64) -> bool {
        if self.x < 0 || self.x >= width as i64 {
            return false;
        }
        if self.y < 0 || self.y >= height as i64 {
            return false;
        }
        true
    }

    fn equals(&self, other: &Coordinates) -> bool {
        self.x == other.x && self.y == other.y 
    }

    fn create(one: &Coordinates, two: &Coordinates) -> Coordinates {
        Coordinates { x: one.x + two.x, y: one.y + two.y }
    }

    fn flip(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
    }

}

fn main() -> Result<()> {
    // let lines: String = match fs::read_to_string("input") {
    let lines: String = match fs::read_to_string("example") {
    // let lines: String = match fs::read_to_string("example") {
        Ok(x) => x,
        Err(_) => { println!("Given file doesn't exist!"); return Ok(()); },
    };
    let mut values: Vec<String> = match serde_json::from_str::<Input>(&lines) {
        Ok(x) => x.values,
        Err(e) => { println!("Couln't parse object: {}", e); return Ok(()); },
    };

    // dbg!(&values);

    let mut map: Vec<Vec<char>> = vec![];
    let cloned = values.clone();
    for line in cloned {
        map.push(line.chars().collect());
    }
    // dbg!(map);

    let mut antinodes = map.clone();
    antinodes = antinodes.iter()
        .map(|row| row.iter()
            .map(|_| '.')
            .collect())
        .collect();

    let mut map_of_antennas: HashMap<char, Vec<Coordinates>> = HashMap::new();

    for (y, row) in map.iter().enumerate() {
        for (x, character) in row.iter().enumerate() {
            if *character == '.' {
                continue;
            }
            if map_of_antennas.get(character).is_none() {
                map_of_antennas.insert(*character, vec![]);
            }
            let vector= map_of_antennas.get_mut(character).unwrap();
            vector.push(Coordinates::new(x as i64, y as i64));
        }
    }

    // dbg!(map_of_antennas);
    let mut key_vector: &Vec<Coordinates>;
    let mut distance: Coordinates;
    let mut possible_antinode: Coordinates;

    for key in map_of_antennas.keys() {
        key_vector = map_of_antennas.get(key).unwrap();
        for antenna in key_vector {
            for other_antenna in key_vector {
                if antenna.equals(&other_antenna) {
                    continue;
                }

                distance = antenna.distance(other_antenna);
                possible_antinode = Coordinates::create(antenna, &distance);

                if possible_antinode.is_contained(map.len() as u64, map[0].len() as u64) {
                    antinodes[possible_antinode.x as usize][possible_antinode.y as usize] = '#';
                }

                distance.flip();
                possible_antinode = Coordinates::create(antenna, &distance);

                if possible_antinode.is_contained(map.len() as u64, map[0].len() as u64) {
                    antinodes[possible_antinode.x as usize][possible_antinode.y as usize] = '#';
                }
            }
        }
    }

    for row in antinodes {
        println!("{}", row.concat());
    }

    Ok(())
}
