use std::fs;

use regex::Regex;

#[derive(PartialEq, Debug)]
struct Matrix {
    x1: f64,
    x2: f64,
    y1: f64,
    y2: f64,
    z1: f64,
    z2: f64
}

fn rref(matrix: &mut Matrix) {
    let mut is_swapped = false;
    // make setup optimal
    if matrix.x1 > matrix.x2 {
        swap_rows(matrix);
        is_swapped = true;
    }

    // top
    if matrix.x1 != 1.0 {
        matrix.x2 /= matrix.x1;
        matrix.z1 /= matrix.x1;
        matrix.x1 = 0.0;
    }

    if matrix.x2 != 0.0 {
        let scalar = 1.0 / matrix.y1;
        matrix.y1 = 0.0;
        matrix.y2 -= matrix.x2 * scalar;
        matrix.z2 -= matrix.z1 * scalar;
    }

    // bottom
    if matrix.y2 != 1.0 {
        matrix.z2 /= matrix.y2;
        matrix.y2 = 1.0;
    }

    if matrix.x2 != 0.0 {
        let scalar = 1.0 / matrix.x2;
        matrix.x2 = 0.0;
        matrix.z1 -= matrix.z2 * scalar;
    }

    // swap back
    if is_swapped {
        swap_rows(matrix);
    }
}

fn swap_rows(matrix: &mut Matrix) {
    let copy = Matrix { x1: matrix.y1, x2: matrix.y2, y1: matrix.x1, y2: matrix.x2, z1: matrix.z2, z2: matrix.z1 };
    matrix.x1 = copy.x1;
    matrix.x2 = copy.x2;
    matrix.y1 = copy.y1;
    matrix.y2 = copy.y2;
    matrix.z1 = copy.z1;
    matrix.z2 = copy.z2;
}

fn is_rref_int_solvable(matrix: &Matrix) -> bool {
    matrix.z1 == matrix.z1.floor() && matrix.z2 == matrix.z2.floor()
}

fn main() {
    let lines = match fs::read_to_string("example") {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };
    let machines = lines.split("\n\n").collect::<Vec<&str>>();

    let query = r"Button A: X\+(\d{1,10}), Y\+(\d{1,10})\nButton B: X\+(\d{1,10}), Y\+(\d{1,10})\nPrize: X\=(\d{1,10}), Y\=(\d{1,10})$";
    let machine_regex = Regex::new(query).unwrap();

    // let mut matrix: [[f64; 3]; 2] = [[0.0; 3]; 2];
    let mut matrix: Matrix = Matrix { x1: 0.0, x2: 0.0, y1: 0.0, y2: 0.0, z1: 0.0, z2: 0.0 };
    let mut has_solution;

    let mut result = 0;

    for machine in machines {
        machine_regex
            .captures_iter(machine)
            .for_each(|captures| { 
            let (_, [x1, y1, x2, y2, z1, z2]) = captures.extract();
                matrix = Matrix {
                    x1: x1.parse().unwrap(),
                    x2: x2.parse().unwrap(),
                    y1: y1.parse().unwrap(),
                    y2: y2.parse().unwrap(),
                    z1: z1.parse().unwrap(),
                    z2: z2.parse().unwrap()
                };
            });

        rref(&mut matrix);
        has_solution = is_rref_int_solvable(&matrix);

        if has_solution {
            // result += 3 * matrix[0][2] as i32 + 1 * matrix[1][2] as i32;
            result += 3 * matrix.z1 as i32 + 1 * matrix.z2 as i32;
        }
    } 
    dbg!(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rref_works() {
        let mut matrix = Matrix { x1: 2.0, x2: 10.0, z1: 20.0, y1: 5.0, y2: 30.0, z2: 30.0 };
        let result = Matrix { x1: 1.0, x2: 0.0, z1: 30.0, y1: 0.0, y2: 1.0, z2: -4.0 };
        rref(&mut matrix);
        assert_eq!(matrix, result);
    }
}