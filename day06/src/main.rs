use std::{collections::HashMap, error::Error, fs::File, io::Read};

#[derive(PartialEq, Clone, Copy, Debug)]
enum GuardDirection {
    Up,
    Down,
    Left,
    Right
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum SquareType {
    Air,
    Path,
    Block,
    Guard(GuardDirection)
}

#[derive(Debug)]
struct Map {
    start_coords: (usize, usize), // (y, x)
    start_direction: GuardDirection,
    start_squares: Vec<Vec<SquareType>>,

    guard_coords: (usize, usize), // (y, x)
    guard_direction: GuardDirection,
    squares: Vec<Vec<SquareType>>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(lines: String) -> Map {
        let mut guard_coords: (usize, usize) = (0, 0);
        let mut guard_direction = GuardDirection::Up;
        let mut squares: Vec<Vec<SquareType>> = Default::default();

        let mut column_counter = 0;
        let mut row_counter: usize = 0;

        let width = lines.find("\n").unwrap_or(0);

        lines.split("\n")
            .for_each(|line| { 
                squares.push(vec![]);
                column_counter = 0;
                line.chars()
                .for_each(|char_square| {
                    let decoded_square = Self::char_to_square(char_square);
                    if let SquareType::Guard(ref direction) = decoded_square {
                        guard_coords = (row_counter, column_counter);
                        guard_direction = *direction;
                    }
                    squares[row_counter].push(decoded_square);
                    column_counter += 1;
                });
                row_counter += 1;
            });

        Map {
            width,
            height: row_counter - 1,
            guard_coords,
            guard_direction,
            squares: squares.clone(),

            start_coords: guard_coords,
            start_direction: guard_direction,
            start_squares: squares
        } 
    }

    fn char_to_square(character: char) -> SquareType {
        match character {
            '^' => SquareType::Guard(GuardDirection::Up),
            '>' => SquareType::Guard(GuardDirection::Right),
            '<' => SquareType::Guard(GuardDirection::Left),
            'v' => SquareType::Guard(GuardDirection::Down),
            '#' => SquareType::Block,
            '.' => SquareType::Air,
            _ => SquareType::Air
        }
    }

    fn simulate_guard_moves(&mut self) {
        loop {
            // boundary checks
            if  (self.guard_direction == GuardDirection::Up && self.guard_coords.0 == 0) ||
                (self.guard_direction == GuardDirection::Down && self.guard_coords.0 == self.height - 1) ||
                (self.guard_direction == GuardDirection::Left && self.guard_coords.1 == 0) ||
                (self.guard_direction == GuardDirection::Right && self.guard_coords.1 == self.width - 1) {
                self.squares[self.guard_coords.0][self.guard_coords.1] = SquareType::Path;
                return;
            }

            let next_move: (i32, i32) = match self.guard_direction {
                GuardDirection::Up => (-1, 0),
                GuardDirection::Down => (1, 0),
                GuardDirection::Left => (0, -1),
                GuardDirection::Right => (0, 1),
            };

            let next_square = &self.squares
                [(self.guard_coords.0 as i32 + next_move.0) as usize]
                [(self.guard_coords.1 as i32 + next_move.1) as usize];

            let is_blocked = *next_square == SquareType::Block;
                
            // block checks
            if is_blocked {
                // turn 90 deg
                self.guard_direction = match self.guard_direction {
                    GuardDirection::Up => GuardDirection::Right,
                    GuardDirection::Right => GuardDirection::Down,
                    GuardDirection::Down => GuardDirection::Left,
                    GuardDirection::Left => GuardDirection::Up,
                };
                continue;
            }

            let y = (self.guard_coords.0 as i32 + next_move.0) as usize;
            let x = (self.guard_coords.1 as i32 + next_move.1) as usize;

            // finally make a move
            self.squares
                [(self.guard_coords.0 as i32 + next_move.0) as usize]
                [(self.guard_coords.1 as i32 + next_move.1) as usize] = SquareType::Guard(self.guard_direction);
            self.squares[self.guard_coords.0][self.guard_coords.1] = SquareType::Path;
            self.guard_coords = (
                y,
                x
            );
        }
    }

    fn simulate_guard_moves_with_loop_detection(&mut self) -> bool {
        let mut loop_detector: HashMap<(usize, usize), GuardDirection> = HashMap::new();

        loop {
            // boundary checks
            if  (self.guard_direction == GuardDirection::Up && self.guard_coords.0 == 0) ||
                (self.guard_direction == GuardDirection::Down && self.guard_coords.0 == self.height - 1) ||
                (self.guard_direction == GuardDirection::Left && self.guard_coords.1 == 0) ||
                (self.guard_direction == GuardDirection::Right && self.guard_coords.1 == self.width - 1) {
                self.squares[self.guard_coords.0][self.guard_coords.1] = SquareType::Path;
                return false;
            }

            let next_move: (i32, i32) = match self.guard_direction {
                GuardDirection::Up => (-1, 0),
                GuardDirection::Down => (1, 0),
                GuardDirection::Left => (0, -1),
                GuardDirection::Right => (0, 1),
            };

            let next_square = &self.squares
                [(self.guard_coords.0 as i32 + next_move.0) as usize]
                [(self.guard_coords.1 as i32 + next_move.1) as usize];

            let is_blocked = *next_square == SquareType::Block;
                
            // block checks
            if is_blocked {
                // turn 90 deg
                self.guard_direction = match self.guard_direction {
                    GuardDirection::Up => GuardDirection::Right,
                    GuardDirection::Right => GuardDirection::Down,
                    GuardDirection::Down => GuardDirection::Left,
                    GuardDirection::Left => GuardDirection::Up,
                };
                continue;
            }

            if let Some(direction) = loop_detector.get(&self.guard_coords) {
                if self.guard_direction == *direction {
                    return true;
                }
            }

            loop_detector.insert(self.guard_coords, self.guard_direction);


            let y = (self.guard_coords.0 as i32 + next_move.0) as usize;
            let x = (self.guard_coords.1 as i32 + next_move.1) as usize;

            // finally make a move
            self.squares
                [(self.guard_coords.0 as i32 + next_move.0) as usize]
                [(self.guard_coords.1 as i32 + next_move.1) as usize] = SquareType::Guard(self.guard_direction);
            self.squares[self.guard_coords.0][self.guard_coords.1] = SquareType::Path;
            self.guard_coords = (
                y,
                x
            );
        }
    }

    fn count_guard_moves(&self) -> usize {
        self.squares.iter()
            .map(|row| row.iter()
                .filter(|square| **square == SquareType::Path)
                .count())
            .sum::<usize>()
    }

    fn get_coords_of_possible_obstructions(&self) -> Vec<(usize, usize)> {
        let mut possible_obstruction_points = vec![];

        for (y_coord, row) in self.squares.iter().enumerate() {
            for (x_coord, square) in row.iter().enumerate() {
                if *square == SquareType::Path && (y_coord, x_coord) != self.start_coords {
                    possible_obstruction_points.push((y_coord, x_coord));
                }
            }
        }
        possible_obstruction_points
    }

    fn simulate_and_count_obstructions(&mut self) -> u32 {
        let possible_obstructions = self.get_coords_of_possible_obstructions();
        
        // clear map
        // place obstruction
        // run simulation
        // check whether it finished in IDK 10 000 steps or not
        //
        // OR detect loops by keeping track of all positions and directions and then just look if
        // this position has been noted before
    

        // dbg!(&possible_obstructions);
        

        let results = possible_obstructions.iter().map(|obstruction| {
            self.squares = self.start_squares.clone();
            self.guard_direction = self.start_direction;
            self.guard_coords = self.start_coords;

            self.squares[obstruction.0][obstruction.1] = SquareType::Block;

            self.simulate_guard_moves_with_loop_detection()
        });

        results.filter(|result| *result).count() as u32
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let mut lines = Default::default();
    // let _ = File::open("example.txt")?
    let _ = File::open("input.txt")?
        .read_to_string(&mut lines);

    // dbg!(&lines);
    // dbg!(&map);

    let mut map = Map::new(lines);
    map.simulate_guard_moves();
    let result = map.count_guard_moves();

    let result = map.simulate_and_count_obstructions();
    
    dbg!(result);

    Ok(())
}
