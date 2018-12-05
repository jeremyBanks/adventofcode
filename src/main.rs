#![feature(duration_as_u128)]
#![allow(unused_imports)]
#![feature(range_contains)]

#[macro_use]
extern crate derive_more;

use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    env, fs,
    time::Instant,
};

use regex::Regex;

use itertools::Itertools;

fn main() {
    println!("");
    println!("     🎄 🎄 🎄 🎄 🎄 🎄 🎄 🎄 ");
    println!("    🎄 Advent of Code 2018 🎄");
    println!("     🎄 🎄 🎄 🎄 🎄 🎄 🎄 🎄 ");
    println!("");

    let argv: Vec<String> = env::args().collect();

    let n: usize = match argv.len() {
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
                _ => unimplemented!("unrecognized problem number"),
            };

            let $solution = match $n {
                $( $number => $fn, )*
                _ => unimplemented!("unrecognized problem number"),
            };
        }
    }

    get_solution!{
        Get solution number n as solution, called solution_name, from:

          (1) chronal_calibration
          (2) inventory_management_system
          (3) no_matter_how_you_slice_it
          (4) repose_record
          (5) alchemical_reduction
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
    let expected_1a = 580;
    let expected_1b = 81972;

    let numbers: Vec<isize> = input
        .iter()
        .map(|s| s.parse().expect("non-integer in input"))
        .collect();
    let solution_1a = numbers.iter().cloned().sum::<isize>();

    println!("  1a. frequency sum: {}", solution_1a);

    let mut seen = HashSet::new();
    let mut sum: isize = 0;
    let mut solution_1b = None;
    for number in numbers.iter().cycle() {
        sum += number;
        if seen.contains(&sum) {
            solution_1b = Some(sum);
            println!("  1b. first repeated sum: {}", &sum);
            break;
        } else {
            seen.insert(sum.clone());
        }
    }
    let solution_1b = solution_1b.unwrap();

    assert_eq!(expected_1a, solution_1a);
    assert_eq!(expected_1b, solution_1b);
}

fn inventory_management_system(input: Vec<&str>) {
    let expected_2a = 9139;
    let expected_2b = "uqcidadzwtnhsljvxyobmkfyr";

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
    let solution_2a = warehouse_checksum;

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
    let solution_2b = result.unwrap();
    println!("  2b. id overlap: {}", solution_2b);

    assert_eq!(expected_2a, solution_2a);
    assert_eq!(expected_2b, solution_2b);
}

fn is_digit(c: &char) -> bool {
    ('0'..='9').contains(c)
}

fn is_not_digit(c: &char) -> bool {
    !('0'..='9').contains(c)
}

fn no_matter_how_you_slice_it(input: Vec<&str>) {
    let expected_3a = 112378;
    let expected_3b = 603;

    #[derive(Clone, Debug)]
    struct Claim {
        id: usize,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    }

    let mut claims = Vec::new();

    // example:
    // #11 @ 755,237: 24x22
    let pattern = Regex::new(
        r"(?x)
        ^\#
        (?P<id>\d+)
        \s@\s
        (?P<x>\d+)
        ,
        (?P<y>\d+)
        :\s
        (?P<width>\d+)
        x
        (?P<height>\d+)
        $",
    )
    .unwrap();

    for line in input {
        let pieces = pattern.captures(line).unwrap();
        claims.push(Claim {
            id: pieces["id"].parse().unwrap(),
            x: pieces["x"].parse().unwrap(),
            y: pieces["y"].parse().unwrap(),
            width: pieces["width"].parse().unwrap(),
            height: pieces["height"].parse().unwrap(),
        });
    }

    let mut cell_counts = vec![0u32; 1_000_000];
    let mut overcommitted_square_inches = 0;
    for claim in claims.iter() {
        for x in claim.x..(claim.x + claim.width) {
            for y in claim.y..(claim.y + claim.height) {
                let index = y * 1000 + x;
                cell_counts[index] += 1;
                // count the cell only the first time it becomes overcommitted
                if cell_counts[index] == 2 {
                    overcommitted_square_inches += 1;
                }
            }
        }
    }
    println!(
        "  3a. overcommitted square inches: {}",
        overcommitted_square_inches
    );
    let solution_3a = overcommitted_square_inches;

    let mut solution_3b = None;

    for claim in claims.iter() {
        let mut all_good = true;
        'out: for x in claim.x..(claim.x + claim.width) {
            for y in claim.y..(claim.y + claim.height) {
                let index = y * 1000 + x;
                if cell_counts[index] > 1 {
                    all_good = false;
                    break 'out;
                }
            }
        }
        if all_good {
            solution_3b = Some(claim.id.clone());
            println!("  3b. uncontested claim: #{}", claim.id);
        }
    }

    let solution_3b = solution_3b.unwrap();

    assert_eq!(expected_3a, solution_3a);
    assert_eq!(expected_3b, solution_3b);
}

fn repose_record(input: Vec<&str>) {
    let expected_4a = 94542;
    let expected_4b = 50966;

    #[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
    struct Event {
        year: usize,
        month: usize,
        day: usize,
        hour: usize,
        minute: usize,
        event_type: Type,
    }

    #[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
    enum Type {
        // shift change to specified new guard id
        ShiftChange(usize),
        FallsAsleep,
        WakesUp,
    }

    // [1518-11-01 00:30] falls asleep
    // [1518-11-01 00:55] wakes up
    // [1518-11-01 23:58] Guard #99 begins shift

    let mut events = Vec::new();
    for line in input {
        let rest = line.chars();
        let rest = rest.skip_while(is_not_digit);
        let year_str: String = rest.clone().take_while(is_digit).collect();
        let rest = rest.skip_while(is_digit).skip_while(is_not_digit);
        let month_str: String = rest.clone().take_while(is_digit).collect();
        let rest = rest.skip_while(is_digit).skip_while(is_not_digit);
        let day_str: String = rest.clone().take_while(is_digit).collect();
        let rest = rest.skip_while(is_digit).skip_while(is_not_digit);
        let hour_str: String = rest.clone().take_while(is_digit).collect();
        let rest = rest.skip_while(is_digit).skip_while(is_not_digit);
        let minute_str: String = rest.clone().take_while(is_digit).collect();
        let s: String = rest.clone().collect();

        let event_type = if s.contains("falls asleep") {
            Type::FallsAsleep
        } else if s.contains("wakes up") {
            Type::WakesUp
        } else {
            let rest = rest.skip_while(is_digit).skip_while(is_not_digit);
            let guard_id_str: String = rest.clone().take_while(is_digit).collect();
            Type::ShiftChange(guard_id_str.parse().unwrap())
        };

        events.push(Event {
            year: year_str.parse().unwrap(),
            month: month_str.parse().unwrap(),
            day: day_str.parse().unwrap(),
            hour: hour_str.parse().unwrap(),
            minute: minute_str.parse().unwrap(),
            event_type: event_type,
        });
    }

    events.sort();

    let mut minutes_sleep_frequencies_by_guard_id = HashMap::<usize, Vec<usize>>::new();

    let mut guard_ids: HashSet<usize> = HashSet::new();
    for event in events.iter() {
        if let Type::ShiftChange(guard_id) = event.event_type {
            if !guard_ids.contains(&guard_id) {
                minutes_sleep_frequencies_by_guard_id.insert(guard_id, vec![0; 60]);
                guard_ids.insert(guard_id);
            }
        }
    }

    let mut current_guard: Option<usize> = None;
    let mut current_sleep_start_minute: Option<usize> = None;
    for event in events.iter() {
        match event.event_type {
            Type::ShiftChange(guard_id) => {
                if let Some(start_minute) = current_sleep_start_minute {
                    if let Some(old_guard_id) = current_guard {
                        let mut m = start_minute;
                        loop {
                            if m == event.minute {
                                break;
                            }

                            minutes_sleep_frequencies_by_guard_id
                                .get_mut(&old_guard_id)
                                .unwrap()[m] += 1;

                            m = (m + 1) % 60;
                        }
                    }
                }
                current_guard = Some(guard_id);
                current_sleep_start_minute = None;
            }
            Type::WakesUp => {
                if let Some(start_minute) = current_sleep_start_minute {
                    if let Some(old_guard_id) = current_guard {
                        let mut m = start_minute;
                        loop {
                            if m == event.minute {
                                break;
                            }

                            minutes_sleep_frequencies_by_guard_id
                                .get_mut(&old_guard_id)
                                .unwrap()[m] += 1;

                            m = (m + 1) % 60;
                        }
                    }
                }
                current_sleep_start_minute = None;
            }
            Type::FallsAsleep => {
                current_sleep_start_minute = Some(event.minute);
            }
        }
    }

    let mut max_sum = 0;
    let mut max_sum_guard_id = 0;
    let mut max_sum_minute = 0;
    let mut total_minutes_slept_by_guard_id: HashMap<usize, usize> = HashMap::new();
    for (guard_id, minutes) in minutes_sleep_frequencies_by_guard_id.iter() {
        let sum = minutes.iter().sum();
        if sum > max_sum {
            max_sum_guard_id = *guard_id;
            max_sum = sum;
            let mut max_egsegesgesges = 0;
            max_sum_minute = 0;
            for (i, i_minutes) in minutes.iter().enumerate() {
                if *i_minutes > max_egsegesgesges {
                    max_sum_minute = i;
                    max_egsegesgesges = *i_minutes;
                }
            }
        }
        total_minutes_slept_by_guard_id.insert(*guard_id, sum);
    }

    let solution_4a = max_sum_guard_id * max_sum_minute;
    println!("  4a. {}", solution_4a);

    let mut max_sum_guard_id = 0;
    let mut max_sum_minute = 0;
    let mut max_egsegesgesges = 0;
    let mut total_minutes_slept_by_guard_id: HashMap<usize, usize> = HashMap::new();
    for (guard_id, minutes) in minutes_sleep_frequencies_by_guard_id.iter() {
        let sum = minutes.iter().sum();
        for (i, i_minutes) in minutes.iter().enumerate() {
            if *i_minutes > max_egsegesgesges {
                max_sum_guard_id = *guard_id;
                max_sum_minute = i;
                max_egsegesgesges = *i_minutes;
            }
        }
        total_minutes_slept_by_guard_id.insert(*guard_id, sum);
    }

    let solution_4b = max_sum_guard_id * max_sum_minute;
    println!("  4b. {}", solution_4b);

    assert_eq!(expected_4a, solution_4a);
    assert_eq!(expected_4b, solution_4b);
}

fn alchemical_reduction(input: Vec<&str>) {
    // let expected_5a = None;
    // let expected_5b = None;

    unimplemented!();

    input.len();

    // let solution_5a;
    // let solution_5b;
    //
    // assert_eq!(expected_5a, solution_5a);
    // assert_eq!(expected_5b, solution_5b);
}
