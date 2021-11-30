#![feature(associated_type_defaults)]
#![allow(unused_imports)]

mod solutions;

pub fn main() -> () {
    solutions::run_all();
}

pub trait Solution {
    const YEAR: u32;
    const DAY: u32;

    fn run() -> (Self::PartOne, Self::PartTwo) {
        let input_path = format!("./inputs/year{:04}-day{:02}.txt", Self::YEAR, Self::DAY);

        let input = std::fs::read_to_string(&input_path).unwrap_or_else(|_| {
            let session_key = std::env::var("CURL_AOC_SESSION").unwrap();
            let input_url = format!(
                "https://adventofcode.com/{}/day/{}/input",
                Self::YEAR,
                Self::DAY
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

            input
        });
        let input = input.trim();
        let solution = Self::solve(input);

        if &solution != &Default::default() {
            println!("{:4} Day {:<2} = {:?}", Self::YEAR, Self::DAY, solution);
        }

        solution
    }

    fn solve(input: &str) -> (Self::PartOne, Self::PartTwo) {
        (Self::part_one(input), Self::part_two(input))
    }

    type PartOne: Default + std::fmt::Debug + PartialEq = ();
    fn part_one(input: &str) -> Self::PartOne {
        drop(input);
        Default::default()
    }

    type PartTwo: Default + std::fmt::Debug + PartialEq = ();
    fn part_two(input: &str) -> Self::PartTwo {
        drop(input);
        Default::default()
    }
}
