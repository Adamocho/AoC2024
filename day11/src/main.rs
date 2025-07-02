use core::panic;
use std::{collections::{HashMap, HashSet}, fs};

fn main() {
    let lines = match fs::read_to_string("input") {
        Ok(value) => value,
        Err(e) => panic!("Could not read file: {}", e)
    };

    let mut stones: Vec<u64> = lines.split(" ").map(|stone| stone.parse::<u64>().unwrap()).collect();

    let mut map: HashMap<u64, u64> = HashMap::new();
    let mut counter = 0;
    let mut to_add: Vec<(u64, u64)>;
    let mut to_zero: Vec<u64>;

    for stone in &stones {
        map.entry(*stone).and_modify(|value| {*value += 1}).or_insert(1);
    }

    // dbg!(&stones);

    map.entry(0).or_insert(0);

    while  counter < 75 {
        to_add = vec![];
        to_zero = vec![];
        for stone in map.keys() {
            let length: u32= (*stone as f64).log10().floor() as u32 + 1;
            let power: u64 = 10_u64.pow(length / 2);
            
            if *stone == 0 {
                continue;
            }
            else if length % 2 == 0 {
                to_add.push((*stone % power, map[stone]));
                to_add.push((*stone / power, map[stone]));
                to_zero.push(*stone);
            } else {
                to_add.push((*stone * 2024, map[stone]));
                to_zero.push(*stone);
            }
        }

        // reset 0s
        let zeros = map[&0];
        map.entry(0).and_modify(|value| {*value = 0});

        for key in to_zero {
            map.entry(key).and_modify(|value| {*value = 0});
        }

        for (key, quantity) in to_add {
            map.entry(key).and_modify(|value| {*value += quantity}).or_insert(quantity);
        }

        // add 1s
        map.entry(1).and_modify(|value| {*value += zeros}).or_insert(zeros);

        counter += 1;
        // dbg!(counter);
        // dbg!(&map);
    }

    // dbg!(&stones);

    let sum: u64 = map.iter().map(|(_, value)| value).sum();
    dbg!(sum);

}