#![feature(
    duration_as_u128,
    range_contains,
    associated_type_defaults,
    never_type,
    const_fn
)]
#![allow(unused_imports, non_camel_case_types)]

#[macro_use]
extern crate derive_more;

use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    env,
    fmt::{Debug, Display},
    fs,
    time::Instant,
};

use regex::Regex;

use itertools::Itertools;

type int = isize;
type uint = usize;
#[allow(dead_code)]
const INT_MAX: int = std::isize::MAX;
#[allow(dead_code)]
const INT_MIN: int = std::isize::MIN;
#[allow(dead_code)]
const UINT_MAX: uint = std::usize::MAX;
#[allow(dead_code)]
const UINT_MIN: uint = std::usize::MIN;

fn main() {
    println!("");
    println!("   🎄 🎄 🎄 🎄 🎄 🎄 🎄 🎄 ");
    println!("  🎄 Advent of Code 2018 🎄");
    println!("   🎄 🎄 🎄 🎄 🎄 🎄 🎄 🎄 ");
    println!("");

    let argv: Vec<String> = env::args().collect();
    let n: Option<uint> = match argv.len() {
        1 => None,
        2 => Some(argv[1].parse().unwrap()),
        _ => {
            println!("usage: {} [PROBLEM_NUMBER]", argv[0]);
            return;
        }
    };

    let solutions: Vec<Box<dyn Fn() -> ()>> = vec![
        Box::new(|| ChronalCalibration.run()),
        Box::new(|| InventoryManagementSystem.run()),
        Box::new(|| NoMatterHowYouSliceIt.run()),
        Box::new(|| ReposeRecord.run()),
        Box::new(|| AlchemicalReduction.run()),
    ];

    if let Some(n) = n {
        solutions[n - 1]()
    } else {
        for f in solutions {
            f()
        }
    }
}

trait Problem<SolutionA: Debug + PartialEq = (), SolutionB: Debug + PartialEq = ()> {
    fn day(&self) -> uint;
    fn name(&self) -> &'static str;
    fn known_solution(&self) -> (Option<SolutionA>, Option<SolutionB>) {
        (None, None)
    }
    fn solve(&self, input: Vec<&str>) -> (SolutionA, SolutionB);

    fn run(&self) {
        let day = self.day();
        println!("  ✨ {}", self.name());
        println!("");
        let input_full =
            fs::read_to_string(format!("input/{}.txt", day)).expect("Failed to load input.");
        let input_lines = input_full.split("\n").filter(|s| s.len() > 0).collect();

        let (expected_a, expected_b) = self.known_solution();

        let before = Instant::now();
        let (a, b) = self.solve(input_lines);
        let duration = Instant::now() - before;

        fn report<T: Debug + PartialEq>(day: uint, part: char, expected: Option<T>, actual: T) {
            if let Some(expected) = expected {
                if actual == expected {
                    println!("  {}{}. {:?}", day, part, actual);
                } else {
                    println!("❌{}{}. {:?} (expected {:?})", day, part, actual, expected);
                }
            } else {
                println!("❓{}{}. {:?}", day, part, actual);
            }
        }

        report(day, 'a', expected_a, a);
        report(day, 'b', expected_b, b);
        println!("");
        println!("  µs. {}", duration.as_micros());
        println!("");
    }
}

struct ChronalCalibration;
impl Problem<int, int> for ChronalCalibration {
    fn day(&self) -> uint {
        1
    }
    fn name(&self) -> &'static str {
        "Chronal Calibration"
    }
    fn known_solution(&self) -> (Option<int>, Option<int>) {
        (Some(580), Some(81972))
    }
    fn solve(&self, input: Vec<&str>) -> (int, int) {
        let numbers: Vec<int> = input
            .iter()
            .map(|s| s.parse().expect("non-integer in input"))
            .collect();
        let frequency_sum = numbers.iter().cloned().sum::<int>();

        let mut seen = HashSet::new();
        let mut sum: int = 0;
        let mut first_repeated_sum = None;
        for number in numbers.iter().cycle() {
            sum += number;
            if seen.contains(&sum) {
                first_repeated_sum = Some(sum);
                break;
            } else {
                seen.insert(sum.clone());
            }
        }
        let first_repeated_sum = first_repeated_sum.unwrap();

        (frequency_sum, first_repeated_sum)
    }
}

struct InventoryManagementSystem;
impl Problem<uint, String> for InventoryManagementSystem {
    fn day(&self) -> uint {
        2
    }
    fn name(&self) -> &'static str {
        "Inventory Management System"
    }
    fn known_solution(&self) -> (Option<uint>, Option<String>) {
        (Some(9139), Some("uqcidadzwtnhsljvxyobmkfyr".to_string()))
    }
    fn solve(&self, input: Vec<&str>) -> (uint, String) {
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

        (solution_2a, solution_2b)
    }
}

fn is_digit(c: &char) -> bool {
    ('0'..='9').contains(c)
}

fn is_not_digit(c: &char) -> bool {
    !('0'..='9').contains(c)
}

struct NoMatterHowYouSliceIt;
impl Problem<uint, uint> for NoMatterHowYouSliceIt {
    fn day(&self) -> uint {
        3
    }
    fn name(&self) -> &'static str {
        "No Matter How You Slice It"
    }
    fn known_solution(&self) -> (Option<uint>, Option<uint>) {
        (Some(112378), Some(603))
    }
    fn solve(&self, input: Vec<&str>) -> (uint, uint) {
        #[derive(Clone, Debug)]
        struct Claim {
            id: uint,
            x: uint,
            y: uint,
            width: uint,
            height: uint,
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
            }
        }
        let solution_3b = solution_3b.unwrap();

        (solution_3a, solution_3b)
    }
}

struct ReposeRecord;
impl Problem<uint, uint> for ReposeRecord {
    fn day(&self) -> uint {
        4
    }
    fn name(&self) -> &'static str {
        "Repose Record"
    }
    fn known_solution(&self) -> (Option<uint>, Option<uint>) {
        (Some(94542), Some(50966))
    }
    fn solve(&self, input: Vec<&str>) -> (uint, uint) {
        #[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
        struct Event {
            year: uint,
            month: uint,
            day: uint,
            hour: uint,
            minute: uint,
            event_type: Type,
        }

        #[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
        enum Type {
            // shift change to specified new guard id
            ShiftChange(uint),
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

        let mut minutes_sleep_frequencies_by_guard_id = HashMap::<uint, Vec<uint>>::new();

        let mut guard_ids: HashSet<uint> = HashSet::new();
        for event in events.iter() {
            if let Type::ShiftChange(guard_id) = event.event_type {
                if !guard_ids.contains(&guard_id) {
                    minutes_sleep_frequencies_by_guard_id.insert(guard_id, vec![0; 60]);
                    guard_ids.insert(guard_id);
                }
            }
        }

        let mut current_guard: Option<uint> = None;
        let mut current_sleep_start_minute: Option<uint> = None;
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
        let mut total_minutes_slept_by_guard_id: HashMap<uint, uint> = HashMap::new();
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

        let mut max_sum_guard_id = 0;
        let mut max_sum_minute = 0;
        let mut max_egsegesgesges = 0;
        let mut total_minutes_slept_by_guard_id: HashMap<uint, uint> = HashMap::new();
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

        (solution_4a, solution_4b)
    }
}

struct AlchemicalReduction;
impl Problem<uint, uint> for AlchemicalReduction {
    fn day(&self) -> uint {
        5
    }
    fn name(&self) -> &'static str {
        "Alchemical Reduction"
    }
    fn known_solution(&self) -> (Option<uint>, Option<uint>) {
        (Some(11298), Some(5148))
    }
    fn solve(&self, input: Vec<&str>) -> (uint, uint) {
        let input = input[0];

        fn flip_case(unit: char) -> char {
            match unit {
                'a'..='z' => unit.to_ascii_uppercase(),
                'A'..='Z' => unit.to_ascii_lowercase(),
                _ => panic!("unexpected character in input"),
            }
        }

        fn post_reaction_size(units: impl Iterator<Item = char>) -> uint {
            let mut reactor = VecDeque::new();
            let mut hopper = units.collect::<VecDeque<_>>();

            assert!(hopper.len() >= 2);

            while hopper.len() > 0 {
                let left = reactor.pop_back().or_else(|| hopper.pop_front()).unwrap();
                let right = hopper.pop_front().unwrap();

                if left == flip_case(right) {
                    // they annihilate each other and we drop them
                } else {
                    // they survive in the reaction chamber
                    reactor.push_back(left);
                    reactor.push_back(right);
                }
            }

            reactor.len()
        }

        let solution_5a = post_reaction_size(input.chars());

        let solution_5b = {
            let unit_types: HashSet<_> = input.chars().map(|c| c.to_ascii_lowercase()).collect();
            let mut best_unit_size = UINT_MAX;
            for unit_lower in unit_types.iter() {
                let unit_upper = unit_lower.to_ascii_uppercase();
                let filtered_input = input
                    .chars()
                    .filter(|c| *c != unit_upper && *c != *unit_lower);
                let size = post_reaction_size(filtered_input);
                if size < best_unit_size {
                    best_unit_size = size;
                }
            }

            best_unit_size
        };

        (solution_5a, solution_5b)
    }
}
