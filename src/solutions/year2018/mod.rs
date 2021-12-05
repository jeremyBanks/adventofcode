use crate::prelude::*;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;

pub fn solutions() -> Vec<Solution> {
    vec![
        day01::solution(),
        day02::solution(),
        day03::solution(),
        day04::solution(),
        day04::solution(),
        day05::solution(),
        day06::solution(),
    ]
}
