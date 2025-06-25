use serde::{Deserialize, Serialize};
use serde_json::{Result, from_file};

#[derive(Serialize, Deserialize)]
struct Equation {
    sum: i32,
    values: Vec<i32>,
}

fn main() -> Result<()> {
    
    let eqations: Vec<Equation> = serde_json::from_file("input.json")?;
    // eqations: Vec<Equations> = serde::from_file("example.json")?;

    Ok(())
}
