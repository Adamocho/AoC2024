use serde::{Deserialize, Serialize};
use serde_json::{Result};
use std::fs;
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

#[derive(Serialize, Deserialize, Debug)]
struct Equation {
    sum: i32,
    values: Vec<i32>
}


fn main() -> Result<()> {
    let lines: String = match fs::read_to_string("input.json") {
        Ok(x) => x,
        Err(_) => { println!("Given file doesn't exist!"); return Ok(()); },
    };
    let lines: Lines = match serde_json::from_str(&lines) {
        Ok(x) => x,
        Err(e) => { println!("Couln't parse object: {}", e); return Ok(()); },
    };
    dbg!(&lines);

    let equations: Vec<Equation> = vec![];
    
    // now change them into proper equations
    //
    // then do a magic with changing + and * symbols and counting out the result (product)
    // compare to the normal one
    // done

    Ok(())
}
