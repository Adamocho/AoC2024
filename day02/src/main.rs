use std::io::prelude::*;
use std::fs::File;
use std::error::Error;


fn is_raport_increasing(raport: &Vec<i32>, is_deleted: bool) -> bool {
    let mut result;
    for i in 0..raport.len()-1 {
        result = raport[i + 1] - raport[i];
        if result < 1 || result > 3 {
            if is_deleted {
                return false;
            }
            let mut another = raport.clone();
            let mut another_two = raport.clone();
            another.remove(i + 1);
            another_two.remove(i);

            let shared = is_raport_increasing(&another, true) || is_raport_increasing(&another_two, true);

            return shared
        }
    }
    true
}

fn is_raport_decreasing(raport: &Vec<i32>, is_deleted: bool) -> bool {
    let mut result;
    for i in 0..raport.len()-1 {
        result = raport[i] - raport[i + 1];
        if result < 1 || result > 3 {
            if is_deleted {
                return false;
            }
            let mut another = raport.clone();
            let mut another_two = raport.clone();
            another.remove(i + 1);
            another_two.remove(i);

            let shared = is_raport_decreasing(&another, true) || is_raport_decreasing(&another_two, true);

            return shared
        }
    }
    true
}

fn evaluate_raport(raport: &Vec<i32>) -> bool {
    let one:bool = is_raport_decreasing(raport, false);
    let two:bool = is_raport_increasing(raport, false);

    return one || two
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input_file = File::open("./input/raports.txt")?;
    let mut lines: String = String::new();

    input_file.read_to_string(&mut lines)?;

    let mut good_raports = 0;

    for line in lines.split('\n') {
        let raport = line.split_whitespace().map(|value| value.parse::<i32>().unwrap_or(0)).collect::<Vec<i32>>();

        if raport.len() < 3 { continue; }
        
        good_raports += evaluate_raport(&raport) as u32;
    }

    println!("{}", good_raports);

    Ok(())
}
