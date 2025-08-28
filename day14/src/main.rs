use core::panic;
use std::{fs, ops::{Add, AddAssign}};

use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Vector2D {
    x: i32,
    y: i32
}

#[derive(Debug)]
struct Robot {
    position: Vector2D,
    velocity: Vector2D
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

fn cartesian_plane(robots: &Vec<Robot>) -> [i32; 4] {
    let mut point_in_plane;
    let mut scores = [0, 0, 0, 0];

    for robot in robots {
        point_in_plane = Vector2D {
            x: robot.position.x - CENTER_X as i32,
            y: - robot.position.y + CENTER_Y as i32
        };

        // dbg!(point_in_plane);

        match point_in_plane {
            v if v.x > 0 && v.y > 0 => scores[0] += 1,
            v if v.x < 0 && v.y > 0 => scores[1] += 1,
            v if v.x < 0 && v.y < 0 => scores[2] += 1,
            v if v.x > 0 && v.y < 0 => scores[3] += 1,
            _ => {}
        }
    }

    return scores
}

fn update_map(map: &mut [[char; WIDTH]; HEIGHT], robots: &Vec<Robot>) {
    // clear map
    *map = [['.'; WIDTH]; HEIGHT];
    // set # based on robot position
    for robot in robots {
        map[robot.position.y as usize][robot.position.x as usize] = '#';
    }
}

fn show_map(map: &[[char; WIDTH]; HEIGHT]) {
    for row in map {
        println!("{:?}", row.as_slice());
    }
}

fn can_there_be_tree(map: &[[char; WIDTH]; HEIGHT]) -> bool {
    for x in 3..WIDTH - 3 {
        for y in 3..HEIGHT - 3 {
            if map[y][x] == '#'
            && map[y-1][x] == '#'
            && map[y-2][x] == '#'
            && map[y][x-1] == '#'
            && map[y][x-2] == '#'
            && map[y+1][x] == '#'
            && map[y+2][x] == '#'
            && map[y][x+1] == '#'
            && map[y][x+2] == '#' {
                return true
            }
        }
    }
    return false
}

const WIDTH: usize = 101;
const CENTER_X: usize = WIDTH / 2;
const HEIGHT: usize = 103;
const CENTER_Y: usize = HEIGHT / 2;

fn main() {
    let text = match fs::read_to_string("input") {
        Ok(v) => v,
        Err(e) => panic!("{}", e)
    };
    let lines = text.lines();

    let mut robots = vec![];
    let mut map: [[char; WIDTH]; HEIGHT] = [['.'; WIDTH]; HEIGHT];

    let query = r"p=(-?\d{1,10}),(-?\d{1,10}) v=(-?\d{1,10}),(-?\d{1,10})";
    let values_regex = Regex::new(query).unwrap();

    for line in lines {
        values_regex
            .captures_iter(line)
            .for_each(|captures| { 
            let (_, [cord_x, cord_y, vel_x, vel_y]) = captures.extract();
                robots.push(Robot {
                    position: Vector2D { x: cord_x.parse().unwrap(), y: cord_y.parse().unwrap() },
                    velocity: Vector2D { x: vel_x.parse().unwrap(), y: vel_y.parse().unwrap() }
                });
        })
    }

    let mut elapsed_seconds = 0;
    while elapsed_seconds <= 38962 {
        elapsed_seconds += 1;

        for robot in &mut robots {
            robot.position += robot.velocity;

            robot.position.x += WIDTH as i32;
            robot.position.x %= WIDTH as i32;

            robot.position.y += HEIGHT as i32;
            robot.position.y %= HEIGHT as i32;
        }


        // either a search algorithm or manually, which is not ideal
        // the tree looks like this - of course more pretty, so there are no gaps inbetween in the map
        // maybe I can use some algorithm to count neightbouring #-marks so that when there are more than 20 then output the map with time number
        //  ############
        // #############
        //  ############
        //   ###########
        //    ##########
        //     #########
        
        update_map(&mut map, &robots);
        if can_there_be_tree(&map) {
            show_map(&map);
            dbg!(elapsed_seconds);
        }
    }
    let quadrants = cartesian_plane(&robots);

    // dbg!(quadrants);
    let product = quadrants.iter().fold(1, |acc, x| acc * x);
    dbg!(product);
}
