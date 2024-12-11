use std::{error::Error, fs::File, io::Read};

fn xmas_count(table: &Vec<Vec<char>>) -> u32 {
    let mut possibilities: Vec<Vec<char>> = vec![];
    let mut character;

    let bottom_bound: usize = table[0].len();
    let right_bound: usize = table[0].len();

    for row in 0..bottom_bound {
        for column in 0..right_bound {
            character = table[row][column];

            if character != 'X' {
                continue;
            }

            // configurations
            let is_right = column + 3 < right_bound;
            let is_left = column > 2;
            let is_up = row > 2;
            let is_down = row + 3 < bottom_bound;

            // right
            if is_right {
                possibilities.push(vec![
                    table[row][column + 1],
                    table[row][column + 2],
                    table[row][column + 3]
                ]);
            }
            // up
            if is_up {
                possibilities.push(vec![
                    table[row - 1][column],
                    table[row - 2][column],
                    table[row - 3][column]
                ]);
            }
            // left
            if is_left {
                possibilities.push(vec![
                    table[row][column - 1],
                    table[row][column - 2],
                    table[row][column - 3]
                ]);
            }
            // down
            if is_down {
                possibilities.push(vec![
                    table[row + 1][column],
                    table[row + 2][column],
                    table[row + 3][column]
                ]);
            }
            // right-up
            if is_right && is_up {
                possibilities.push(vec![
                    table[row - 1][column + 1],
                    table[row - 2][column + 2],
                    table[row - 3][column + 3]
                ]);
            }
            // right-down
            if is_right && is_down {
                possibilities.push(vec![
                    table[row + 1][column + 1],
                    table[row + 2][column + 2],
                    table[row + 3][column + 3]
                ]);
            }
            // left-up
            if is_left && is_up {
                possibilities.push(vec![
                    table[row - 1][column - 1],
                    table[row - 2][column - 2],
                    table[row - 3][column - 3]
                ]);
            }

            // left-down
            if is_left && is_down {
                possibilities.push(vec![
                    table[row + 1][column - 1],
                    table[row + 2][column - 2],
                    table[row + 3][column - 3]
                ]);
            }
        }
    }

    let expected = vec!['M', 'A', 'S'];
    let mut counter = 0;
    
    for possibility in possibilities {
        if possibility == expected {
            counter += 1;
        }
    }

    counter
}

fn is_x_mas(line: &[char]) -> bool {
    let expected = ['M', 'A', 'S'];
    let inverse = ['S', 'A', 'M'];
    
    if line == expected || line == inverse {
        return true;
    }
    false
}

fn x_mas_count(table: &Vec<Vec<char>>) -> u32 {
    let mut character;
    let mut counter = 0;

    let bottom_bound: usize = table[0].len();
    let right_bound: usize = table[0].len();

    for row in 0..bottom_bound {
        for column in 0..right_bound {
            character = table[row][column];
            if character != 'A' {
                continue;
            }

            // configurations
            let is_top_left = row > 0 && column > 0;
            let is_bottom_right = row + 1 < bottom_bound && column + 1 < right_bound;
             
            if is_top_left && is_bottom_right {
                // first diagonal
                let first = [
                    table[row + 1][column - 1],
                    table[row][column],
                    table[row - 1][column + 1]
                ];
                // second diagonal
                let second = [
                    table[row - 1][column - 1],
                    table[row][column],
                    table[row + 1][column + 1]
                ];
                if is_x_mas(&first) && is_x_mas(&second) {
                    counter += 1;
                }
            }
        }
    }

    counter
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input_file = File::open("./input.txt")?;
    let mut lines: String = Default::default();

    let _ = input_file.read_to_string(&mut lines); 

    // let input: Vec<&str> = vec![
    //     "MMMSXXMASM",
    //     "MSAMXMSMSA",
    //     "AMXSXMAAMM",
    //     "MSAMASMSMX",
    //     "XMASAMXAMM",
    //     "XXAMMXXAMA",
    //     "SMSMSASXSS",
    //     "SAXAMASAAA",
    //     "MAMMMXMMMM",
    //     "MXMXAXMASX"
    // ];

    // let input: Vec<&str> = vec![    
    //     ".M.S......",
    //     "..A..MSMS.",
    //     ".M.S.MAA..",
    //     "..A.ASMSM.",
    //     ".M.S.M....",
    //     "..........",
    //     "S.S.S.S.S.",
    //     ".A.A.A.A..",
    //     "M.M.M.M.M.",
    //     ".........."
    // ];

    let mut table: Vec<Vec<char>> = vec![];

    for line in lines.lines() {
        table.push(line.chars().collect::<Vec<char>>());
    }
    // for line in input {
    //     table.push(line.chars().collect::<Vec<char>>());
    // }

    dbg!(xmas_count(&table));
    dbg!(x_mas_count(&table));

    Ok(())
}
