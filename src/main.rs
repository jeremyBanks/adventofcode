use std::{collections::BTreeSet, env, fs};

fn main() {
    let n_str = env::args()
        .skip(1)
        .next()
        .expect("problem number must be specified as CLI argument");
    let n: u64 = n_str
        .parse()
        .expect("problem number must be a small integer");

    println!("");
    println!("  🎄⭐🎄⭐🎄⭐🎄⭐🎄⭐🎄⭐🎄⭐🎄⭐🎄⭐🎄");
    println!("  ⭐ Advent of Code 2018: Problem #{:<2} ⭐", n);
    println!("  🎄⭐🎄⭐🎄⭐🎄⭐🎄⭐🎄⭐🎄⭐🎄⭐🎄⭐🎄");
    println!("");

    let input = fs::read_to_string(format!("input/{}.txt", n)).expect("Failed to load input.");

    let solution = match n {
        1 => day_1,
        _ => panic!("No solution implemented."),
    };

    solution(&input);
}

fn day_1(input: &str) {
    let numbers: Vec<i64> = input
        .split("\n")
        .filter(|s| s.len() > 0)
        .map(|s| s.parse().expect("non-integer in input"))
        .collect();
    println!("  frequency sum: {}", numbers.iter().cloned().sum::<i64>());

    let mut seen = BTreeSet::new();
    let mut sum: i64 = 0;
    for number in numbers.iter().cycle() {
        sum += number;
        if seen.contains(&sum) {
            println!("  first repeated sum: {}", &sum);
            break;
        } else {
            seen.insert(sum.clone());
        }
    }
}
