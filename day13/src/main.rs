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
    let is_swapped = matrix.x1 > matrix.y1 && matrix.y1 != 0.0;
    // make setup optimal
    if is_swapped {
        swap_rows(matrix);
    }

    // top
    if matrix.x1 != 0.0 {
        matrix.x2 /= matrix.x1;
        matrix.z1 /= matrix.x1;
        matrix.x1 = 1.0;
    }

    if matrix.y1 != 0.0 && matrix.x1 != 0.0 {
        let scalar = matrix.y1 / matrix.x1;
        matrix.y1 = 0.0;
        matrix.y2 -= matrix.x2 * scalar;
        matrix.z2 -= matrix.z1 * scalar;
    }

    // bottom
    if matrix.y2 != 0.0 {
        matrix.z2 /= matrix.y2;
        matrix.y2 = 1.0;
    }

    if matrix.y2 != 0.0 {
        let scalar = matrix.x2 / matrix.y2;
        matrix.x2 = 0.0;
        matrix.z1 -= matrix.z2 * scalar;
    }

    // swap back
    // if is_swapped {
    //     swap_rows(matrix);
    // }
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

fn is_rref_int_solvable(matrix: &mut Matrix) -> bool {
    let epsilon = 0.001;

    let real = matrix.z1;
    let floored = real.floor();
    let ceiled = real.ceil();

    if real - floored > epsilon && ceiled - real > epsilon {
        return false;
    }

    matrix.z1 = if real - floored <= epsilon {
        floored
    } else {
        ceiled
    };

    let real = matrix.z2;
    let floored = real.floor();
    let ceiled = real.ceil();

    if real - floored > epsilon && ceiled - real > epsilon {
        return false;
    }

    matrix.z2 = if real - floored <= epsilon {
        floored
    } else {
        ceiled
    };
    true
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

                matrix.z1 += 10_000_000_000_000.0;
                matrix.z2 += 10_000_000_000_000.0;
            });

        rref(&mut matrix);
        has_solution = is_rref_int_solvable(&mut matrix);

        if has_solution {
            // result += 3 * matrix[0][2] as i32 + 1 * matrix[1][2] as i32;
            result += 3 * matrix.z1 as i64 + matrix.z2 as i64;
        }
        dbg!(result);
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

    #[test]
    fn rref_works_with_swap() {
        let mut matrix = Matrix { y1: 2.0, y2: 10.0, z2: 20.0, x1: 5.0, x2: 30.0, z1: 30.0 };
        let result = Matrix { x1: 1.0, x2: 0.0, z1: 30.0, y1: 0.0, y2: 1.0, z2: -4.0 };
        rref(&mut matrix);
        assert_eq!(matrix, result);
    }

    #[test]
    fn rref_works_when_empty() {
        let mut matrix = Matrix { x1: 1.0, x2: 0.0, z1: 1.0, y1: 0.0, y2: 1.0, z2: 1.0 };
        let result = Matrix { x1: 1.0, x2: 0.0, z1: 1.0, y1: 0.0, y2: 1.0, z2: 1.0 };
        rref(&mut matrix);
        assert_eq!(matrix, result);
    }

    #[test]
    fn is_solvable_with_ints() {
        let mut matrix = Matrix { x1: 0.0, x2: 0.0, y1: 0.0, y2: 0.0, z1: 1.0, z2: 1.0 };
        let is_solvable = is_rref_int_solvable(&mut matrix);
        assert!(is_solvable);
    }

    #[test]
    #[should_panic]
    fn is_not_solvable_with_float() {
        let mut matrix = Matrix { x1: 0.0, x2: 0.0, y1: 0.0, y2: 0.0, z1: 1.1, z2: 0.9 };
        let is_solvable = is_rref_int_solvable(&mut matrix);
        assert!(is_solvable);
    }

    #[test]
    fn is_be_solvable_with_float() {
        let mut matrix = Matrix { x1: 0.0, x2: 0.0, y1: 0.0, y2: 0.0, z1: 1.001, z2: 0.9999 };
        let is_solvable = is_rref_int_solvable(&mut matrix);
        assert!(is_solvable);
    }

    #[test]
    fn is_be_solvable_with_float_with_high_precision() {
        let mut matrix = Matrix { x1: 0.0, x2: 0.0, y1: 0.0, y2: 0.0, z1: 1.0000001, z2: 0.99999999 };
        let is_solvable = is_rref_int_solvable(&mut matrix);
        assert!(is_solvable);
    }

    #[test]
    fn can_swapp() {
        let mut matrix = Matrix { x1: 5.0, x2: 2.0, y1: 1.0, y2: 3.0, z1: 10.0, z2: 20.0 };
        let original = Matrix { x1: 5.0, x2: 2.0, y1: 1.0, y2: 3.0, z1: 10.0, z2: 20.0 };
        let result = Matrix { x1: 1.0, x2: 3.0, y1: 5.0, y2: 2.0, z1: 20.0, z2: 10.0 };
        swap_rows(&mut matrix);
        assert_eq!(matrix, result);
        swap_rows(&mut matrix);
        assert_eq!(matrix, original);
    }

    #[test]
    fn should_swap() {
        let mut matrix = Matrix {x1: 0.0, x2: 1.0, y1: 1.0, y2: 0.0, z1: 40.0, z2: 80.0};
        let result = Matrix {x1: 1.0, x2: 0.0, y1: 0.0, y2: 1.0, z1: 80.0, z2: 40.0};
        swap_rows(&mut matrix);
        assert_eq!(matrix, result);
    }
}
