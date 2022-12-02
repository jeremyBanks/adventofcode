use crate::prelude::*;

pub mod day01;
pub mod day02;

pub fn solutions() -> Vec<Solution> {
    vec![day01::solution(), day02::solution()]
}
