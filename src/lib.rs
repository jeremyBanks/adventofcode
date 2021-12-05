#![allow(
    clippy::comparison_chain,
    clippy::cargo,
    clippy::pedantic,
    clippy::trivially_copy_pass_by_ref
)]
#![warn(
    clippy::self_named_module_files,
    clippy::unseparated_literal_suffix,
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::cloned_instead_of_copied,
    clippy::create_dir,
    clippy::wildcard_imports,
    clippy::default_trait_access
)]

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

    let page_path = format!(
        "./src/solutions/year{:04}/day{:02}.txt",
        solution.year, solution.day
    );

    let page = std::fs::read_to_string(&page_path).unwrap_or_else(|_| {
        let session_key =
            std::env::var("AOC_SESSION").unwrap_or_else(|_| panic!("AOC_SESSION not set"));
        let page_url = format!(
            "https://adventofcode.com/{}/day/{}",
            solution.year, solution.day
        );
        let page = reqwest::blocking::Client::new()
            .get(page_url)
            .header(reqwest::header::COOKIE, format!("session={}", session_key))
            .send()
            .unwrap()
            .error_for_status()
            .unwrap()
            .text()
            .unwrap()
            .trim()
            .to_string();

        let page = page
            .split('\n')
            .skip_while(|line| !line.starts_with("<main"))
            .skip(1)
            .take_while(|line| !line.starts_with("</main"))
            .join("\n");

        let parts = page
            .split("<article class=\"day-desc\">")
            .skip(1)
            .map(|chunk| chunk.split("</article>").next().unwrap())
            .collect::<Vec<_>>();

        let part_one = parts[0].to_string();
        let part_two = parts.get(1);

        let title = part_one
            .split("<h2>--- ")
            .nth(1)
            .unwrap()
            .split(" ---</h2>")
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap();

        let page = format!(
            "# {}\n\n## Part One\n\n{}\n\n## Part Two\n\n{}\n",
            title,
            part_one,
            part_two.unwrap_or(&"*unknown*")
        );

        if part_two.is_some() {
            std::fs::write(&page_path, &page).unwrap();
        } else {
            tracing::warn!("not saving problem because we only have part one");
        }

        page
    });

    let title = page.split('\n').next().unwrap().strip_prefix("# ").unwrap();

    let start = Instant::now();
    let result = (solution.code)(input);
    let duration = start.elapsed();

    println!("{:4} Day {:>2}: {}", solution.year, solution.day, title);
    println!("             A = {}", result.0);
    println!("             B = {}", result.1);
    println!(
        "            Δt = {:<14} = {:>14}ns",
        duration.to_debug(),
        duration.as_nanos().separate_with_commas(),
    );
    println!();

    result
}
