use std::{fmt, fs, iter::repeat_n};

#[derive(PartialEq, Eq, Debug)]
enum DataType {
    File,
    Space
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Partition {
    File(u32),
    Space
}

impl fmt::Display for Partition {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Partition::File(number) => write!(formatter, "{}", number),
            &Partition::Space => write!(formatter, ".")
        }
    }
}

fn fill_the_gaps(disc_map: &mut [Partition]) {
    let length = disc_map.len();
    let mut back_block: &Partition;
    let mut front_block: &Partition;
    let mut back_counter: usize = length - 1;

    let range = 0..=(length - 1);

    for index in range {
        front_block = disc_map.get(index).unwrap();
        if let Partition::File(_) = *front_block {
            continue;
        }

        while back_counter > index {
            back_block = disc_map.get(back_counter).unwrap();
            if *back_block == Partition::Space {
                back_counter -= 1;
                continue;
            }

            let front_block = front_block.clone();
            let back_block = back_block.clone();

            disc_map[back_counter] = front_block;
            disc_map[index] = back_block;

            back_counter -= 1;
            break;
        }
    }
}

fn move_file(disc_map: &mut [Partition], file_start: usize, file_length: usize, location_start: usize) {
    let file = disc_map[file_start].clone();

    for index in 0..file_length {
        disc_map[location_start + index] = file.clone();
        disc_map[file_start + index] = Partition::Space;
    }
}

fn get_next_gap(disc_map: &[Partition], limit: usize, file_length: usize) -> (usize, usize) {
    let mut gap_length: usize = 0;
    let mut gap_start: usize = 0;
    let mut result;

    let mut index = 0;

    while index < limit {
        result = &disc_map[index];
        index += 1;
        match *result {
            Partition::File(_) => {
                if gap_length == 0 {
                    continue;
                }
                if file_length <= gap_length {
                    return (gap_start, gap_length);
                }
                gap_length = 0;
            }
            Partition::Space => {
                if gap_length == 0 {
                    gap_start = index - 1;
                }
                gap_length += 1;
            }
        }
    }
    (0, 0)
}

fn get_next_file(disc_map: &[Partition], back_counter: &mut usize) -> (usize, usize) {
    let mut file_length: usize = 0;
    let mut previous_id: u32 = 0;
    let mut result;

    if let Partition::File(id) = disc_map[disc_map.len() - 1] {
        previous_id = id;
    }

    while *back_counter > 0 {
        result = &disc_map[*back_counter];
        *back_counter -= 1;

        match result {
            Partition::File(id) => {
                if previous_id == *id {
                    file_length += 1;
                    continue;
                } 
                if file_length != 0 {
                    return (*back_counter + 2, file_length);
                }
                previous_id = *id;
                file_length = 1;
            }
            Partition::Space => {
                if file_length != 0 {
                    return (*back_counter + 2, file_length);
                }
            }
        }
    }
    (0, 0)
}

fn better_fill_gaps(disc_map: &mut [Partition]) {
    let length = disc_map.len();
    let mut back_counter: usize = length - 1; 
    let mut gap: (usize, usize);
    let mut file: (usize, usize);

    while back_counter > 0 {
        file = get_next_file(disc_map, &mut back_counter);
        if file == (0, 0) {
            break;
        }

        // roll back the pointer
        back_counter += 1;

        gap = get_next_gap(disc_map, back_counter, file.1); 
        if gap == (0, 0) {
            continue;
        }
        // move file
        move_file(disc_map, file.0, file.1, gap.0);
    }
}

fn calculate_checksum(disc_map: &[Partition]) -> u64 {
    let mut sum: u64 = 0;
    for (index, partition) in disc_map.iter().enumerate() {
        if let Partition::File(value) = partition {
            sum += *value as u64 * index as u64;
        }
    }
    sum
}

fn main() {
    let lines = match fs::read_to_string("input") {
    // let lines = match fs::read_to_string("example") {
        Ok(x) => x,
        Err(_) => panic!("Couldn't read from file"),
    };

    let mut disc_map: Vec<Partition> = vec![];
    let mut number: u32;
    let mut state: DataType = DataType::File;
    let mut counter: u32 = 0;

    for character in lines.trim().chars() {
        number = character.to_digit(10).unwrap();
        let mut other: Vec<Partition>;

        match state {
            DataType::File => {
                other = repeat_n(Partition::File(counter), number as usize).collect::<Vec<Partition>>();
                state = DataType::Space;
            },
            DataType::Space => {
                other = repeat_n(Partition::Space, number as usize).collect::<Vec<Partition>>();
                state = DataType::File;
                counter += 1;
            },
        }
        disc_map.append(&mut other);
    }

    // disc_map.iter().for_each(|partition| print!("{}", partition));
    // dbg!(lines);

    // fill_the_gaps(&mut disc_map);
    better_fill_gaps(&mut disc_map);
    // disc_map.iter().for_each(|partition| print!("{}", partition));
    let sum = calculate_checksum(&disc_map);
    dbg!(sum);
}
