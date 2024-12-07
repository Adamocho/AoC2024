use std::collections::HashMap;
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


fn create_similarity_hashmap(right: &[i32]) -> HashMap<i32, u32> {
    let mut similarity_table: HashMap<i32, u32> = HashMap::new();

    for (_, value) in right.iter().enumerate() {
        if similarity_table.contains_key(value) {
            if let Some(x) = similarity_table.get_mut(&value) {
                *x += 1;
            }
        }
        else {
            similarity_table.insert(*value, 1);
        }
    }

    similarity_table
}

fn compute_similarity_score(left: &[i32], similarity_table: &HashMap<i32, u32>) -> u32 {
    let mut result: u32 = 0;

    for (_, value) in left.iter().enumerate() {
        if let Some(amount) = similarity_table.get(value) {
            result += *value as u32 * amount;
        }
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


    let similarity_table: HashMap<i32, u32> = create_similarity_hashmap(&right);
    
    println!("The similarity score is {}", compute_similarity_score(&left, &similarity_table));



    Ok(())
}
