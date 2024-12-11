use std::{error::Error, fs::File, io::Read};
use regex::Regex;


fn task_one(lines: String) -> i32 {
    let mut result = 0;
    let mut index = 0;
    let mul_regex = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    let number_regex = Regex::new(r"[0-9]{1,3}").unwrap();

    // let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    
    for line in lines.lines() {
        let output: Vec<_> = mul_regex.find_iter(line).map(|f| f.as_str()).collect();

        let binding = output.join("");
        let output: Vec<i32> = number_regex.find_iter(&binding).map(|f| f.as_str().parse().unwrap()).collect();

        while index < output.len() {
            result += output[index] * output[index + 1];

            index += 2;
        }
        index = 0;
    }
    result
}


fn with_dosanddonts(lines: String) -> i32 {
    let mut result = 0;
    let mut index = 0;
    let dos_and_donts_regex = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)|don't\(\)|do\(\)").unwrap();
    let number_regex = Regex::new(r"[0-9]{1,3}").unwrap();
    let mut is_allowed = true;

    // let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    for line in lines.lines() {
        let output: Vec<_> = dos_and_donts_regex.find_iter(line).map(|f| f.as_str()).collect();
        let mut valid: Vec<&str> = vec![];

        for value in &output {
            match *value {
                "do()" => {
                    is_allowed = true;
                    continue;
                }
                "don\'t()" => {
                    is_allowed = false;
                    continue;
                }
                _ => {
                    if is_allowed {
                        valid.push(value);
                    }
                }
            };
        }

        let binding = valid.join("");
        let output: Vec<i32> = number_regex.find_iter(&binding).map(|f| f.as_str().parse().unwrap()).collect();

        while index < output.len() {
            result += output[index] * output[index + 1];

            index += 2;
        }
        index = 0;
    }
    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input_file = File::open("./input.txt")?;
    let mut lines: String = Default::default();

    input_file.read_to_string(&mut lines)?;

    // let task_one = task_one(lines);
    let task_two = with_dosanddonts(lines);

    dbg!(task_two);

    Ok(())
}
