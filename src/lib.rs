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

/// My Advent of Code solutions in Rust.
#[derive(clap::Parser, Debug)]
#[clap(about, author)]
pub struct Args {
    /// Optionally, specify a single year to run (instead of all).
    pub year: Option<u32>,

    /// Optionally, specify a single day to run (instead of all).
    pub day: Option<u32>,
}

pub async fn main(args: Args) {
    let solutions = crate::solutions::solutions();

    for solution in solutions.iter() {
        if args.year.is_some() && args.year != Some(solution.year) {
            continue;
        }
        if args.day.is_some() && args.day != Some(solution.day) {
            continue;
        }
        run(solution).await;
    }
}

pub async fn run(solution: &Solution) -> (String, String) {
    let input_path = format!(
        "./src/solutions/year{:04}/day{:02}-input.txt",
        solution.year, solution.day
    );

    let client = reqwest::Client::new();

    let mut input = std::fs::read_to_string(&input_path);

    if input.is_err() {
        input = {
            let session_key =
                std::env::var("AOC_SESSION").unwrap_or_else(|_| panic!("AOC_SESSION not set"));
            let input_url = format!(
                "https://adventofcode.com/{}/day/{}/input",
                solution.year, solution.day
            );
            let mut input = client
                .get(input_url)
                .header(reqwest::header::COOKIE, format!("session={}", session_key))
                .send()
                .await
                .unwrap()
                .error_for_status()
                .unwrap()
                .text()
                .await
                .unwrap()
                .trim()
                .to_string();
            input.push('\n');

            std::fs::write(&input_path, &input).unwrap();

            Ok(input)
        }
    }
    let input = input.unwrap();

    let input = input.trim();

    let page_path = format!(
        "./src/solutions/year{:04}/day{:02}.md",
        solution.year, solution.day
    );

    let mut page = std::fs::read_to_string(&page_path);

    if page.is_err() || page.as_ref().unwrap().contains("*TODO*") {
        page = {
            let session_key =
                std::env::var("AOC_SESSION").unwrap_or_else(|_| panic!("AOC_SESSION not set"));
            let page_url = format!(
                "https://adventofcode.com/{}/day/{}",
                solution.year, solution.day
            );
            let page = client
                .get(&page_url)
                .header(reqwest::header::COOKIE, format!("session={}", session_key))
                .send()
                .await
                .unwrap()
                .error_for_status()
                .unwrap()
                .text()
                .await
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

            let part_one = part_one.split_once("</h2>").unwrap().1.trim();
            let part_two = part_two.map(|s| s.split_once("</h2>").unwrap().1.trim());

            let page = format!(
                "# {}\n\n{}\n\n## Part One\n\n{}\n\n## Part Two\n\n{}\n",
                title,
                &page_url,
                part_one,
                part_two.unwrap_or("*TODO*")
            );

            std::fs::write(&page_path, &page).unwrap();

            Ok(page)
        }
    }
    let page = page.unwrap();

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
