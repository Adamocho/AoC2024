use core::num;
use std::{collections::{HashMap, LinkedList, VecDeque}, fs, hash::Hash, vec};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn blink(cache: &mut HashMap<u64, u64>) {
    let mut length: usize;
    let mut operand: u64;

    // let numer_of_zeros = cache.entry(0).or_insert(0);
    let mut inserting: Vec<(u64, u64)> = vec![];
    let mut make_zero: Vec<u64> = vec![];

    cache.keys().for_each(|key| {
        if cache[key] == 0 || *key == 0 {
            return;
        }
        let length = (*key as f64).log10().floor() as usize + 1;
        let operand = 10_u64.pow(length as u32/ 2);
        if length % 2 == 0 {
            inserting.push((key / operand, *cache.get(key).unwrap()));
            inserting.push((key % operand, *cache.get(key).unwrap()));
            make_zero.push(*key);
        } else {
            inserting.push((key * 2024, *cache.get(key).unwrap()));    
            make_zero.push(*key);
        }
    });

    let number = cache[&0];
    // remove 0s
    cache.entry(0).and_modify(|value| { *value = 0 }).or_insert(0);

    // zero divided stones
    for stone in make_zero {
        cache.entry(stone).and_modify(|value| { *value = 0 });
    }

    // update divided stones
    for stone in inserting {
        cache.entry(stone.0).and_modify(|value| { *value += stone.1 }).or_insert(stone.1);
    }

    // update all 1-s
    cache.entry(1).and_modify(|value| { *value += number }).or_insert(number);
}

fn main() {
    // let lines = match fs::read_to_string("example") {
    let lines = match fs::read_to_string("input") {
        Ok(x) => x,
        Err(e) => panic!("Could not access file: {}", e),
    };

    let mut test = HashMap::new();
    *test.entry("some").or_insert(10) *= 10;

    let stones_strings: Vec<&str> = lines.trim().split(" ").collect();
    let stones: Vec<u64> = stones_strings.iter().map(|value| value.parse().unwrap()).collect();

    let mut cache: HashMap<u64, u64> = HashMap::new();

    stones.iter().for_each(|stone| {
        *cache.entry(*stone).or_insert(0) += 1;
    });

    let mut counter = 0;
    // make sure zero is there
    let _ = cache.entry(0).or_insert(0);

    while counter < 75 {
        blink(&mut cache);
        counter += 1;
        dbg!(counter);
    }

    // dbg!(&cache);

    // cache.iter().for_each(|(x, y)| {
    //     let repeated = (x.to_string() + " ").repeat(*y as usize);
    //     print!("{}", repeated);
    // });
    
    let sum: u64 = cache.par_iter().map(|(_, y)| *y).sum();
    dbg!(sum);
    
}
