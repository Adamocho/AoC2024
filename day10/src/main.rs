use std::fs;

fn main() {
    let lines = match fs::read_to_string("input") {
    // let lines = match fs::read_to_string("example") {
        Ok(x) => x,
        Err(_) => panic!("Couldn't read from file"),
    };

    let mut trail_map: Vec<Vec<u32>> = vec![];

    lines
        .split("\n")
        .for_each(
            |row|  {
                trail_map.push(
                    row
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
                )
            }
        );

    dbg!(trail_map);
}
