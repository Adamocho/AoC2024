use std::{fs, vec};

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
struct Coords {
    x: usize,
    y: usize,
}

fn find_all_trailheads(map: &Vec<Vec<u32>>) -> Vec<Coords> {
    let mut trailheads = vec![];

    for (row_index, row) in map.iter().enumerate() {
        for (column_index, terrain_height) in row.iter().enumerate() {
            if *terrain_height == 0 {
                trailheads.push(Coords { x: column_index, y: row_index });
            }
        }
    }
    trailheads
}

fn find_trails(map: &Vec<Vec<u32>>, origin: Coords, level: u32) -> u32 {
    if level == 9 {
        // return vec![origin].len();
        return 1;
    }
    // let mut trails: Vec<Coords> = vec![];
    let mut sum = 0;
    let width = map[0].len();
    let height = map.len();

    // check directions
    // up
    if origin.y != 0            && map[origin.y - 1][origin.x] == level + 1 {
        // let mut found = find_trails(map, Coords { x: origin.x, y: origin.y - 1 }, level + 1);
        // trails.append(&mut found);
        sum += find_trails(map, Coords { x: origin.x, y: origin.y - 1 }, level + 1);
    }
    // down
    if origin.y != height - 1   && map[origin.y + 1][origin.x] == level + 1 {
        // let mut found = find_trails(map, Coords { x: origin.x, y: origin.y + 1 }, level + 1);
        // trails.append(&mut found);
        sum += find_trails(map, Coords { x: origin.x, y: origin.y + 1 }, level + 1);
    }
    // left
    if origin.x != 0            && map[origin.y][origin.x - 1] == level + 1 {
        // let mut found = find_trails(map, Coords { x: origin.x - 1, y: origin.y }, level + 1);
        // trails.append(&mut found);
        sum += find_trails(map, Coords { x: origin.x - 1, y: origin.y }, level + 1);
    }
    // right
    if origin.x != width - 1    && map[origin.y][origin.x + 1] == level + 1 {
        // let mut found = find_trails(map, Coords { x: origin.x + 1, y: origin.y }, level + 1);
        // trails.append(&mut found);
        sum += find_trails(map, Coords { x: origin.x + 1, y: origin.y }, level + 1);
    }
    // return trails;
    return sum;
}

fn main() {
    let lines = match fs::read_to_string("input") {
    // let lines = match fs::read_to_string("example") {
        Ok(x) => x,
        Err(_) => panic!("Couldn't read from file"),
    };

    let mut trail_map: Vec<Vec<u32>> = vec![];

    lines
        .split("\n")
        .for_each(
            |row|  {
                trail_map.push(
                    row
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
                )
            }
        );

    let trailheads = find_all_trailheads(&trail_map);

    let mut score = 0;
    for trailhead in trailheads {
        // let found_trails = HashSet::<Coords>::from_iter(find_trails(&trail_map, trailhead, 0)).iter().count();
        // score += found_trails;
        score += find_trails(&trail_map, trailhead, 0);
        // dbg!(score);
    }

    // finalize
    dbg!(score);
}
