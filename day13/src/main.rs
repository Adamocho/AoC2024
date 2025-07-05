use std::fs;

use regex::Regex;

fn rref(matrix: &mut [[f64; 3]; 2]) {
    todo!("Implement Reduced Row Echelon Form");
}

fn is_rref_int_solvable(matrix: &[[f64; 3]; 2]) -> bool {
    matrix[0][0] == 1.0
    && matrix[0][1] == 0.0
    && matrix[1][0] == 0.0
    && matrix[1][1] == 1.0
    && matrix[0][2] == matrix[0][2].floor()
    && matrix[1][2] == matrix[1][2].floor()
}

fn main() {
    let lines = match fs::read_to_string("example") {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };
    let machines = lines.split("\n\n").collect::<Vec<&str>>();

    let query = r"Button A: X\+(\d{1,10}), Y\+(\d{1,10})\nButton B: X\+(\d{1,10}), Y\+(\d{1,10})\nPrize: X\=(\d{1,10}), Y\=(\d{1,10})$";
    let machine_regex = Regex::new(query).unwrap();

    let mut matrix: [[f64; 3]; 2] = [[0.0; 3]; 2];
    let mut has_solution;

    let mut result = 0;

    for machine in machines {
        machine_regex
            .captures_iter(machine)
            .for_each(|captures| { 
            let (_, [x1, y1, x2, y2, z1, z2]) = captures.extract();
                matrix = [
                    [x1.parse().unwrap(), x2.parse().unwrap(), z1.parse().unwrap()],
                    [y1.parse().unwrap(), y2.parse().unwrap(), z2.parse().unwrap()]
                    ];
            });

        rref(&mut matrix);
        has_solution = is_rref_int_solvable(&matrix);

        if has_solution {
            result += 3 * matrix[0][2] as i32 + 1 * matrix[1][2] as i32;
        }
    } 
    dbg!(result);
}
