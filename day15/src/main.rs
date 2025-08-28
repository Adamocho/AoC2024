use std::{ops::{Add, AddAssign, Neg}, thread::sleep, time::Duration};


struct Game {
    map: Vec<Vec<char>>,
    robot: Vector2D,
}

#[derive(Debug, Clone, Copy)]
struct Vector2D {
    x: i32,
    y: i32
}

impl Add for Vector2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl AddAssign for Vector2D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Neg for Vector2D {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector2D {
            x: -self.x,
            y: -self.y
        }
    }
}

#[derive(Debug)]
enum  Instruction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

const MAP_STRING: &str =
// "########
// #..O.O.#
// ##@.O..#
// #...O..#
// #.#.O..#
// #...O..#
// #......#
// ########";

"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########";


const INSTRUCTIONS: &str = 
// "<^^>>>vv<v>>v<<";

"<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

impl Game {
    fn execute_next_cycle(&mut self, instruction: Instruction) {
        // let instruction = &self.instruction[0];
        let direction: Vector2D;
        match instruction {
            Instruction::UP => direction = Vector2D { x: 0, y: -1 },
            Instruction::RIGHT => direction = Vector2D { x: 1, y: 0 },
            Instruction::DOWN => direction = Vector2D { x: 0, y: 1 },
            Instruction::LEFT => direction = Vector2D { x: -1, y: 0 },
        }
        let future_coords = Vector2D {
            y: self.robot.y + direction.y,
            x: self.robot.x + direction.x,
        };
        let point = &self.map
        [future_coords.y as usize]
        [future_coords.x as usize];

        if *point == '#' {
            return
        }
        if *point == '.' {
            self.map[self.robot.y as usize][self.robot.x as usize] = '.';
            self.map[future_coords.y as usize][future_coords.x as usize] = '@';
            self.robot = future_coords;
            return
        }
        if *point == 'O' {
            self.find_free_space(future_coords, direction)
        }
    }

    fn find_free_space(&mut self, position: Vector2D, direction: Vector2D) {
        let c = self.map[position.y as usize][position.x as usize];
        if c == '#' {
            return
        }
        if c == '.' {
            // move them
            self.move_objects(position, self.robot, -direction);
            self.robot += direction;
            return
        }
        if c == 'O' {
            self.find_free_space(position + direction, direction);
        }
    }

    fn move_objects(&mut self, start: Vector2D, end: Vector2D, direction: Vector2D) {
        if start.x == end.x && start.y == end.y {
            return
        }

        let pointer = start;
        let temp: char = self.map[pointer.y as usize][pointer.x as usize];
        self.map[pointer.y as usize][pointer.x as usize] =
            self.map[(pointer.y + direction.y) as usize][(pointer.x + direction.x) as usize];
        self.map[(pointer.y + direction.y) as usize][(pointer.x + direction.x) as usize] = temp; 

        self.move_objects(start + direction, end, direction)
    }
}


fn count_score(map: &Vec<Vec<char>>) -> usize {
    let mut sum = 0;

    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[y][x] == 'O' {
                sum += 100 * y + x;
            }
        }
    }
   sum 
}

fn show_map(map: &Vec<Vec<char>>) {
    let mut line;
    for x in 0..map.len() {
        line = String::new();
        for y in 0..map[0].len() {
            line.push(map[x][y]);
        }
        println!("{}", line);
    }
}


fn main() {
    let mut map: Vec<Vec<char>> = vec![];

    for line in MAP_STRING.lines() {
        map.push(line.chars().collect());
    }

    show_map(&map);

    let mut instructions: Vec<Instruction> = vec![];
    for instruction in INSTRUCTIONS.chars() {
        match instruction {
            '>' => instructions.push(Instruction::RIGHT),
            'v' => instructions.push(Instruction::DOWN),
            '<' => instructions.push(Instruction::LEFT),
            '^' => instructions.push(Instruction::UP),
            _ => {}
        }
    }

    let mut game = Game {
        map: map,
        robot: Vector2D { x: 4, y: 4 },
    };

    for instruction in instructions {
        dbg!(&instruction);
        game.execute_next_cycle(instruction);
        show_map(&game.map);
        // sleep(Duration::from_millis(2000));
    }

    let score = count_score(&game.map);
    dbg!(score);
}
