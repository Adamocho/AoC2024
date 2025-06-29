use std::{fs, vec};

fn blink<'a>(stones: &Vec<u64>) -> Vec<u64> {
    let mut new_stones: Vec<u64> = vec![];
    let mut value_length: usize;
    let mut stone_string: String;


    for stone in stones {
        stone_string = stone.to_string();
        value_length = stone_string.len();

        if *stone == 0 {
            new_stones.push(1);
        } else if value_length % 2 == 0 {
            let split_stones = stone_string.split_at(value_length/2);
            let left_stone = split_stones.0.parse::<u64>().unwrap();
            let right_stone = split_stones.1.parse::<u64>().unwrap();
            new_stones.push(left_stone);
            new_stones.push(right_stone);
        } else {
            new_stones.push(stone * 2024);
        }
    }
    new_stones
}

fn main() {
    // let lines = match fs::read_to_string("example") {
    let lines = match fs::read_to_string("input") {
        Ok(x) => x,
        Err(e) => panic!("Could not access file: {}", e),
    };

    let stones: Vec<&str> = lines.trim().split(" ").collect();
    let mut stones: Vec<u64> = stones.iter().map(|value| value.parse().unwrap()).collect();

    let mut counter = 0;
    let mut new_stones = vec![];

    while counter < 25 {
        new_stones = blink(&stones);
        // dbg!(&new_stones);

        counter += 1;
        stones = new_stones;
    }

    let sum = stones.len();
    dbg!(sum);
    
}
