use std::{collections::{HashMap, HashSet}, fs, vec};

fn main() {
    let lines = match fs::read_to_string("example") {
        Ok(value) => value,
        Err(e) => panic!("something went wrong: {}", e)
    };

    let fields: Vec<Vec<u32>> = lines.split("\n")
        .map(|line| line.chars()
            .map(|character| u32::from(character))
            .collect())
        .collect();

    // dbg!(field);

    let mut fence_plans: HashMap<u32, (u32, u32)> = HashMap::new();

    let mut field_types: HashMap<usize, (usize, usize)> = HashMap::new();
    for row in fields.iter().enumerate() {
        for field in row.1.iter().enumerate() {
            field_types.entry(field.0).or_insert((row.0, field.0));
        }
    }

    // the field_types need to acknowledge multiple coordinates

    // let mut coords;
    // for key in field_types.keys() {
    //    coords = field_types[key];
    // }


}
