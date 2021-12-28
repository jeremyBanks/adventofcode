use crate::prelude::*;

pub mod day01;
pub mod day02;
pub mod day04;
pub mod day21;

pub fn solutions() -> Vec<Solution> {
    vec![
        day01::solution(),
        day02::solution(),
        day04::solution(),
        day21::solution(),
    ]
}
