use std::io::prelude::*;
use std::fs::File;
use std::error::Error;


fn compute_distance_compound(left: &[i32], right: &[i32]) -> u32 {
    if left.len() != right.len() {
        return 0;
    }

    let mut result: u32 = 0;

    for (index, _) in left.iter().enumerate() {
        result += (left[index] - right[index]).unsigned_abs();
   }

    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input_file = File::open("./input/input.txt")?;
    let mut lines: String = String::new();

    input_file.read_to_string(&mut lines)?;

    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for line in lines.split('\n') {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 2 {
            continue;
        }
        
        left.push(parts[0].parse()?);
        right.push(parts[1].parse()?);
    }

    left.sort();
    right.sort();

    println!("The ditance is equal to {}", compute_distance_compound(&left, &right));

    Ok(())
}
