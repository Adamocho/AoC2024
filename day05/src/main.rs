use std::{collections::HashMap, error::Error, fs::File, io::Read};

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
fn main() -> Result<(), Box<dyn Error>> {
    let mut joined_lines = Default::default();
    let _ = File::open("./input.txt")?
        .read_to_string(&mut joined_lines);

    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut unchecked_updates: Vec<Vec<u32>> = vec![];
    let mut bad_updates: Vec<Vec<u32>> = vec![];
    let mut fixed_updates: Vec<Vec<u32>> = vec![];

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
            } else {
                if let Some(values) = rules.get_mut(&rule[0]) {
                    values.push(rule[1]);
                }
            }
        } else {
            unchecked_updates.push(line.trim().split(',').map(|v| v.parse::<u32>().unwrap()).collect());
        }
    }

    let mut sum_of_middle_pages_correct = 0;

    // now the algorithm starts
    unchecked_updates.iter().for_each(|update| {
        if is_update_correct(&update, &rules) {
            // println!("{:?} is correct", update);
            sum_of_middle_pages_correct += update[update.len()/2];
        }
        else {
            // println!("{:?} is bad", update);
            bad_updates.push(update.to_vec());
        }
    });

    let mut sum_of_middle_pages_fixed = 0;
    // finish part 2

    dbg!(sum_of_middle_pages_correct);

    Ok(())
}
