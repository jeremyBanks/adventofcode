#![allow(unused_imports)]

use std::time::{Duration, Instant};

pub(crate) mod prelude;
pub(crate) mod solutions;

pub(crate) use prelude::prelude;

crate::prelude!();

pub fn main() {
    let solutions = crate::solutions::solutions();

    for solution in solutions.iter() {
        run(solution);
    }
}

pub fn run(solution: &Solution) -> () {
    let input_path = format!(
        "./src/solutions/year{:04}/day{:02}-input.txt",
        solution.year, solution.day
    );

    let input = std::fs::read_to_string(&input_path).unwrap_or_else(|_| {
        let session_key =
            std::env::var("AOC_SESSION").unwrap_or_else(|_| panic!("AOC_SESSION not set"));
        let input_url = format!(
            "https://adventofcode.com/{}/day/{}/input",
            solution.year, solution.day
        );
        let mut input = reqwest::blocking::Client::new()
            .get(input_url)
            .header(reqwest::header::COOKIE, format!("session={}", session_key))
            .send()
            .unwrap()
            .error_for_status()
            .unwrap()
            .text()
            .unwrap()
            .trim()
            .to_string();
        input.push('\n');

        std::fs::write(&input_path, &input).unwrap();
        println!(
            "{:4} Day {:>2}   input downloaded",
            solution.year, solution.day
        );

        input
    });
    let input = input.trim();
    let start = Instant::now();
    let result = (solution.code)(input);
    let duration = start.elapsed();

    println!("{:4} Day {:>2} = {:?}", solution.year, solution.day, result);
    println!(
        "         Δt = {:>14}ns = {:?}",
        duration.as_nanos().separate_with_commas(),
        duration
    );
}
