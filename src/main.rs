use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
};

use itertools::Itertools;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let n: u64 = argv
        .get(1)
        .expect("problem number must be specified as CLI argument")
        .parse()
        .expect("problem number must be a small integer");

    println!("");
    println!("  🎄⭐🎄⭐🎄⭐🎄⭐🎄⭐🎄⭐🎄⭐🎄⭐🎄⭐🎄");
    println!("  ⭐ Advent of Code 2018: Problem #{:<2} ⭐", n);
    println!("  🎄⭐🎄⭐🎄⭐🎄⭐🎄⭐🎄⭐🎄⭐🎄⭐🎄⭐🎄");
    println!("");

    let input = fs::read_to_string(format!("input/{}.txt", n)).expect("Failed to load input.");
    let lines = input.split("\n").filter(|s| s.len() > 0).collect();

    let solution = match n {
        1 => chronal_calibration,
        2 => inventory_management_system,
        _ => panic!("No solution implemented."),
    };

    solution(lines);
}

fn inventory_management_system(input: Vec<&str>) {
    let mut twos = 0;
    let mut threes = 0;

    for line in input.iter() {
        let mut frequencies = BTreeMap::new();

        for letter in line.chars() {
            if frequencies.contains_key(&letter) {
                frequencies.insert(letter, frequencies.get(&letter).unwrap() + 1);
            } else {
                frequencies.insert(letter, 1);
            }
        }

        if frequencies.values().any(|f| *f == 2) {
            twos += 1;
        }

        if frequencies.values().any(|f| *f == 3) {
            threes += 1;
        }
    }

    let warehouse_checksum = twos * threes;
    println!("  2a. warehouse checksum: {}", warehouse_checksum);
    
    println!("  2b. hmm well how many do we have? {}", input.len());
}

fn chronal_calibration(input: Vec<&str>) {
    let numbers: Vec<i64> = input
        .iter()
        .map(|s| s.parse().expect("non-integer in input"))
        .collect();
    println!(
        "  1a. frequency sum: {}",
        numbers.iter().cloned().sum::<i64>()
    );

    let mut seen = BTreeSet::new();
    let mut sum: i64 = 0;
    for number in numbers.iter().cycle() {
        sum += number;
        if seen.contains(&sum) {
            println!("  1b. first repeated sum: {}", &sum);
            break;
        } else {
            seen.insert(sum.clone());
        }
    }
}
