use serde::{Deserialize, Serialize};
use serde_json::{Result};
use core::panic;
use std::{fs::{self, Permissions}, vec};
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
    Multiply
}

fn check_if_equation_can_be_true(equation: &Equation) -> bool {
    let permutations: Vec<Vec<Symbol>> = generate_permutations(equation.values.len() as u32);
    let mut result: i64;

    for permutation in permutations {
        result = equation.values[0];
        for (index, symbol) in permutation.iter().enumerate() {
            match symbol {
                Symbol::Add => {
                    result += equation.values[index + 1];
                },
                Symbol::Multiply => {
                    result *= equation.values[index + 1];
                }
            }
            if result > equation.sum {
                break;
            }
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
    let upper_limit = 2_i32.pow(length - 1);

    let mut symbols: Vec<Symbol>;
    let mut copied_number;
    let mut ending_digit: i32;
    let mut new_symbol: Symbol;

    while number < upper_limit {
        symbols = vec![];
        copied_number = number;

        for _ in 1..length {
            ending_digit = copied_number % 2;
            new_symbol = match ending_digit {
                0 => Symbol::Add,
                _ => Symbol::Multiply,
            };

            symbols.push(new_symbol);
            copied_number = copied_number >> 1;
        }
        number += 1;
        permutations.push(symbols);
    }
    permutations
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
    // dbg!(&lines);

    // proper equations
    let equations: Vec<Equation> = lines.lines.iter().map(|improper_equation| Equation {
            sum: improper_equation.sum,
            values: improper_equation.values
                .split(" ")
                .flat_map(|number| number.parse())
                .collect(),
    }).collect();

    // dbg!(equations.len());

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

    Ok(())
}
