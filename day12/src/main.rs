use std::{collections::{HashMap, HashSet}, fs, sync::atomic::fence, vec};

fn find_all_fields(
    width: &usize, height: &usize,
    x: usize, y: usize,
    field: &u32,
    fields_map: &Vec<Vec<u32>>,
    found_fields: &mut Vec<(usize, usize)>
) -> (u32, u32) {
    let mut fence_size: u32 = 0;
    let mut fields_size: u32 = 1;
    let mut result: (u32, u32);

    // 4 directions check
    found_fields.push((y, x));

    // up
    if y != 0 {
        if *field != fields_map[y - 1][x] {
            fence_size += 1;
        } else {
            if !found_fields.contains(&(y - 1, x)) {
                result = find_all_fields(width, height, x, y - 1, field, fields_map, found_fields);
                fence_size += result.0;
                fields_size += result.1;
            }
        }
    } else { // just add 1 to the fence count
        fence_size += 1;
    }

    // down
    if y < height - 1 {
        if *field != fields_map[y + 1][x] {
            fence_size += 1;
        } else {
            if !found_fields.contains(&(y + 1, x)) {
                result = find_all_fields(width, height, x, y + 1, field, fields_map, found_fields);
                fence_size += result.0;
                fields_size += result.1;
            }
        }
    } else { // just add 1 to the fence count
        fence_size += 1;
    }

    // left
    if x != 0 {
        if *field != fields_map[y][x - 1] {
            fence_size += 1;
        } else {
            if !found_fields.contains(&(y, x - 1)) {
                result = find_all_fields(width, height, x - 1, y, field, fields_map, found_fields);
                fence_size += result.0;
                fields_size += result.1;
            }
        }
    } else { // just add 1 to the fence count
        fence_size += 1;
    }

    // right
    if x < width - 1 {
        if *field != fields_map[y][x + 1] {
            fence_size += 1;
        } else {
            if !found_fields.contains(&(y, x + 1)) {
                result = find_all_fields(width, height, x + 1, y, field, fields_map, found_fields);
                fence_size += result.0;
                fields_size += result.1;
            }
        }
    } else { // just add 1 to the fence count
        fence_size += 1;
    }

    (fence_size, fields_size)
}

fn main() {
    let lines = match fs::read_to_string("input") {
        Ok(value) => value,
        Err(e) => panic!("something went wrong: {}", e)
    };

    let fields: Vec<Vec<u32>> = lines.split("\n")
        .map(|line| line.chars()
            .map(|character| u32::from(character))
            .collect())
        .collect();

    let mut uncalculated_fields = fields.clone();

    // dbg!(field);

    // (nr_of_fences, size)
    let mut fence_costs: Vec<(u32, u32)> = vec![];
    let mut discovered_fields: Vec<(usize, usize)> = vec![];

    let height: usize = fields.len();
    let width: usize = fields[0].len();


    // let mut field_types: HashMap<usize, u32> = HashMap::new();
    for (y, row) in fields.iter().enumerate() {
        for (x, field) in row.iter().enumerate() {
            // 1. create a vec with (u32, u32)
            // 2. start the loop
            // 3. add every single field to the visited_chunks(), which work with only one number
            // 4. replace calculated field with a 0 (add a 0 check afterwards)
            // 5. continue searching - reset the vec
            if uncalculated_fields[y][x] == 0 {
                continue;
            }

            fence_costs.push(find_all_fields(&width, &height, x, y, field, &fields, &mut discovered_fields));

            for (y, x) in discovered_fields {
                uncalculated_fields[y][x] = 0;
            }
            discovered_fields = vec![];
        }
    }

    let cost: u32 = fence_costs.iter().map(|(fences, size)| fences * size).sum();

    dbg!(cost);
}
