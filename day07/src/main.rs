use serde::{Deserialize, Serialize};
use serde_json::{Result};
// use core::panic;
use std::{fs, vec};
// use std::error::Error;
// use serde::de::Error;
// use serde::ser::Error;

#[derive(Serialize, Deserialize, Debug)]
struct Lines {
    lines: Vec<ImproperEquation>
}

#[derive(Serialize, Deserialize, Debug)]
struct ImproperEquation {
    sum: i64,
    values: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Equation {
    sum: i64,
    values: Vec<i64>
}

#[derive(Debug)]
enum Symbol {
    Add,
    Multiply,
    Concat
}

const MY_RADIX: i32 = 3;

fn check_if_equation_can_be_true(equation: &Equation) -> bool {
    let permutations: Vec<Vec<Symbol>> = generate_permutations(equation.values.len() as u32);
    // dbg!(permutations.len());
    // panic!();

    let mut result: i64;
    let mut left_side: String;
    let mut right_side: String;

    for permutation in permutations {
        result = equation.values[0];
        for (index, symbol) in permutation.iter().enumerate() {
            match symbol {
                Symbol::Add => {
                    result += equation.values[index + 1];
                },
                Symbol::Multiply => {
                    result *= equation.values[index + 1];
                },
                Symbol::Concat => {
                    left_side = result.to_string();
                    right_side = equation.values[index + 1].to_string();
                    // println!("before: {}, {}", left_side, right_side);

                    let joined= (left_side + &right_side).parse();

                    if joined.is_err() {
                        panic!("That's the one");
                    }

                    if let Ok(new_value) = joined {
                        result = new_value;
                        // println!("after: {}", &result);
                    }                }
            }
            // if result > equation.sum {
            //     break;
            // }
        }
        if result == equation.sum {
            return true;
        }
    }
    false
}

fn generate_permutations(length: u32) -> Vec<Vec<Symbol>> {
    let mut number = 0;
    let mut permutations: Vec<Vec<Symbol>> = vec![];
    let upper_limit = 3_i32.pow(length - 1);

    let mut symbols: Vec<Symbol>;
    let mut copied_number;
    let mut ending_digit: i32;
    let mut new_symbol: Symbol;
    let mut formatted_number: String;

    while number < upper_limit {
        symbols = vec![];
        copied_number = number;

        formatted_number = format_radix(copied_number as u32, MY_RADIX as u32);
        ending_digit = formatted_number.parse().unwrap_or(0);
        // dbg!(&formatted_number);
        for _ in 0..(length - 1) {
            new_symbol = match (ending_digit % 10 )% MY_RADIX {
                0 => Symbol::Add,
                1 => Symbol::Multiply,
                _ => Symbol::Concat
            };

            symbols.push(new_symbol);
            ending_digit /= 10;
        }
        number += 1;
        permutations.push(symbols);
    }
    permutations
}

fn format_radix(mut x: u32, radix: u32) -> String {
    let mut result = vec![];

    loop {
        let m = x % radix;
        x = x / radix;

        // will panic if you use a bad radix (< 2 or > 36).
        result.push(std::char::from_digit(m, radix).unwrap());
        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect::<String>()
}

fn main() -> Result<()> {
    let lines: String = match fs::read_to_string("input.json") {
    // let lines: String = match fs::read_to_string("example.json") {
        Ok(x) => x,
        Err(_) => { println!("Given file doesn't exist!"); return Ok(()); },
    };
    let lines: Lines = match serde_json::from_str(&lines) {
        Ok(x) => x,
        Err(e) => { println!("Couln't parse object: {}", e); return Ok(()); },
    };

    dbg!(lines.lines.len());

    // proper equations
    let equations: Vec<Equation> = lines.lines.iter().map(|improper_equation| Equation {
            sum: improper_equation.sum,
            values: improper_equation.values
                .split(" ")
                .flat_map(|number| number.parse())
                .collect(),
    }).collect();

    dbg!(equations.len());

    let mut good_equations: Vec<Equation> = vec![];
    let mut bad_equations: Vec<Equation> = vec![];

    equations.iter().for_each(|equation| {
        let is_true = check_if_equation_can_be_true(equation);
        if !is_true {
            bad_equations.push(equation.clone());
        } else {
            good_equations.push(equation.clone());
        }
    });

    dbg!(good_equations.iter().map(|equation| equation.sum).sum::<i64>());
    dbg!(equations.iter().map(|equation| equation.sum).sum::<i64>());
    dbg!(bad_equations.len());

    // test_check_if_equation_can_be_true();
    Ok(())
}