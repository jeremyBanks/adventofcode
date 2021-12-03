#![allow(clippy::self_named_module_files)]

pub(crate) mod common;
pub(crate) mod prelude;
pub(crate) mod solutions;

use crate::prelude::*;

pub fn main() {
    let solutions = crate::solutions::solutions();

    for solution in solutions.iter() {
        run(solution);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snapshot() {
        let solutions = crate::solutions::solutions();

        for solution in solutions.iter() {
            let answer = run(solution);
            let name = format!("year{}day{:02}_answer", solution.year, solution.day);
            insta::assert_yaml_snapshot!(name, answer);
        }
    }
}

pub fn run(solution: &Solution) -> (String, String) {
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

        input
    });
    let input = input.trim();
    let start = Instant::now();
    let result = (solution.code)(input);
    let duration = start.elapsed();

    println!(
        "{:4} Day {:>2}: A = {}",
        solution.year, solution.day, result.0
    );
    println!("             B = {}", result.1);
    println!(
        "            Δt = {:<14} = {:>14}ns",
        duration.to_debug(),
        duration.as_nanos().separate_with_commas(),
    );
    println!();

    result
}
