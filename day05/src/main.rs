use std::{collections::HashMap, error::Error, fs::File, io::Read, vec};

fn is_update_correct(update: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> bool {
    for (index, page) in update.iter().enumerate() {
        if let Some(page_rules) = rules.get(page) {
            for i in 0..index {
                if page_rules.contains(&update[i]) {
                    return false;
                }
            }
        }
    }
    true
}

fn fix_bad_update(mut update: Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> Option<Vec<u32>> {
    let mut incorrect_index: usize;
    let mut helper: u32;
    let mut loop_limiter = LIMIT;
    const LIMIT: u32 = 15;
    // let mut difference: i32;
    while loop_limiter > 0 {
        if is_update_correct(&update, rules) { return Some(update); }

        incorrect_index = return_incorrect_index(&update, rules).unwrap_or(0);

        dbg!(&update, incorrect_index);

        // difference = if incorrect_index == 0 { 1 } else { -1 };
        if incorrect_index == 0 { break; }

        helper = update[incorrect_index];
        update[incorrect_index] = update[incorrect_index - 1];
        update[incorrect_index - 1] = helper;
         
        loop_limiter -= 1;
    }
    None
}

fn return_incorrect_index(update: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> Option<usize> {
    for (index, page) in update.iter().enumerate() {
        if let Some(page_rules) = rules.get(page) {
            for i in 0..index {
                if page_rules.contains(&update[i]) {
                    return Some(index);
                }
            }
        }
    }
    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut joined_lines = Default::default();
    let _ = File::open("./example.txt")?
    // let _ = File::open("./input.txt")?
        .read_to_string(&mut joined_lines);

    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut unchecked_updates: Vec<Vec<u32>> = vec![];
    let mut correct_updates: Vec<Vec<u32>> = vec![];
    let mut bad_updates: Vec<Vec<u32>> = vec![];
    let fixed_updates: Vec<Vec<u32>>;

    let mut is_rules = true;
    for line in joined_lines.split('\n') {
        if line == "\n" || line.trim() == "" {
            is_rules = false;
            continue;
        }

        if is_rules {
            let rule: Vec<u32> = line.trim().split('|').map(|v| v.parse::<u32>().unwrap()).collect();
            if rules.get(&rule[0]).is_none() {
                rules.insert(rule[0], vec![rule[1]]);
            } else if let Some(values) = rules.get_mut(&rule[0]) {
                values.push(rule[1]);
            }
        } else {
            unchecked_updates.push(line.trim().split(',').map(|v| v.parse::<u32>().unwrap()).collect());
        }
    }

    let mut sum_of_middle_pages_correct = 0;

    // now the algorithm starts
    unchecked_updates.iter().for_each(|update| {
        if is_update_correct(&update, &rules) {
            correct_updates.push(update.to_vec());
            sum_of_middle_pages_correct += update[update.len()/2];
        }
        else {
            bad_updates.push(update.to_vec());
        }
    });

    // part 2
    fixed_updates = bad_updates.iter()
        .map(|update| fix_bad_update(update.to_vec(), &rules))
        .filter(|optional_update| optional_update.is_some())
        .map(|update| update.unwrap())
        .collect();

    dbg!(correct_updates.len());
    dbg!(bad_updates.len());
    dbg!(fixed_updates.len());

    let sum_of_middle_pages_fixed: u32 = fixed_updates.iter().map(|update| update[update.len()/2]).sum();

    dbg!(sum_of_middle_pages_correct);
    dbg!(sum_of_middle_pages_fixed);

    Ok(())
}
