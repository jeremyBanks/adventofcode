#![feature(duration_as_u128)]
#![allow(unused_imports)]
#![allow(range_contains)]

use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    env, fs,
    time::Instant,
};

use itertools::Itertools;

fn main() {
    println!("");
    println!("     🎄 🎄 🎄 🎄 🎄 🎄 🎄 🎄 ");
    println!("    🎄 Advent of Code 2018 🎄");
    println!("     🎄 🎄 🎄 🎄 🎄 🎄 🎄 🎄 ");
    println!("");

    let argv: Vec<String> = env::args().collect();

    let n: u64 = match argv.len() {
        2 => argv[1].parse().unwrap(),
        _ => {
            println!("usage: {} $PROBLEM_NUMBER", argv[0]);
            return;
        }
    };

    macro_rules! get_solution {
        {
            Get solution number $n:tt as $solution:ident, called $name:ident, from:
            $(($number:tt) $fn:ident)*
        } => {
            let $name = match $n {
                $( $number => stringify!($fn), )*
                _ => panic!("invalid problem number"),
            };

            let $solution = match $n {
                $( $number => $fn, )*
                _ => panic!("invalid problem number"),
            };
        }
    }

    get_solution!{
        Get solution number n as solution, called solution_name, from:

          (1) chronal_calibration
          (2) inventory_management_system
          (3) no_matter_how_you_slice_it
    };

    println!("          day {} of 25", n);
    println!("");
    println!("  ✨ {:^24} ✨", solution_name);
    println!("");
    println!("");
    println!("");

    let input = fs::read_to_string(format!("input/{}.txt", n)).expect("Failed to load input.");
    let lines = input.split("\n").filter(|s| s.len() > 0).collect();

    let before = Instant::now();
    solution(lines);
    let after = Instant::now();

    let delta = (after - before).as_micros();

    println!("");
    println!("");
    println!("");
    println!("  ⏱  {:>6}µs", delta);
    println!("");
    println!("");
    println!("");
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

fn inventory_management_system(input: Vec<&str>) {
    let mut twos = 0;
    let mut threes = 0;

    for line in input.iter() {
        let mut frequencies = HashMap::new();

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

    fn id_variations(id: &str) -> Vec<String> {
        let id_chars: Vec<char> = id.chars().collect();
        let mut result = Vec::new();
        for index in 0..id_chars.len() {
            let variation_chars = id_chars.clone();
            let variation: String = variation_chars
                .iter()
                .enumerate()
                .map(|x| if x.0 != index { x.1 } else { &'_' })
                .collect();
            result.push(variation);
        }
        result
    }

    let mut seen_ids_by_variation = HashMap::new();
    let mut result: Option<String> = None;
    'out: for id in input.iter() {
        for variation in id_variations(id) {
            if let Some(_adjacent_id) = seen_ids_by_variation.get(&variation) {
                result = Some(variation.chars().filter(|c| *c != '_').collect());
                break 'out;
            } else {
                seen_ids_by_variation.insert(variation, id);
            }
        }
    }
    println!("  2b. id overlap: {}", result.unwrap());
}

fn no_matter_how_you_slice_it(input: Vec<&str>) {
    #[derive(Debug)]
    struct Claim {
        id: u64,
        x: u64,
        y: u64,
        width: u64,
        height: u64,
    }

    let mut claims = Vec::new();

    let is_digit = |c| ('0'..='9').contains;
    let is_not_digit = |c| ('0'..='9').contains(c);

    for line in input {
        let rest = line.chars();
        // example:
        // #11 @ 755,237: 24x22
        let rest = rest.skip_while(is_not_digit).iter();
        let id_str: String = rest.take_while(is_digit).collect();
        let rest = rest
            .skip_while(is_digit)
            .iter()
            .skip_while(is_not_digit)
            .iter();
        let x_str: String = rest.take_while(is_digit).collect();
        let rest = rest
            .skip_while(is_digit)
            .iter()
            .skip_while(is_not_digit)
            .iter();
        let y_str: String = rest.take_while(is_digit).collect();
        let rest = rest
            .skip_while(is_digit)
            .iter()
            .skip_while(is_not_digit)
            .iter();
        let width_str: String = rest.take_while(is_digit).collect();
        let rest = rest
            .skip_while(is_digit)
            .iter()
            .skip_while(is_not_digit)
            .iter();
        let height_str: String = rest.take_while(is_digit).collect();
        let rest = rest
            .skip_while(is_digit)
            .iter()
            .skip_while(is_not_digit)
            .iter();
        claims.push(Claim {
            id: id_str.parse().unwrap(),
            x: x_str.parse().unwrap(),
            y: y_str.parse().unwrap(),
            width: width_str.parse().unwrap(),
            height: height_str.parse().unwrap(),
        });
    }

    println!("{:?}", claims);
}
