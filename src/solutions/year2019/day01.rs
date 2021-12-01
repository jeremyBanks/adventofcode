use crate::prelude::*;

#[derive(Default, Debug)]
pub struct Solution {}

impl crate::Solution for Solution {
    const YEAR: u32 = 2019;
    const DAY: u32 = 1;

    type PartOne = u32;
    fn part_one(input: &str) -> u32 {
        let mut required_fuel = 0;

        let masses = split_and_parse::<u32>(input, "\n");
        for mass in masses {
            let divided = (mass / 3).checked_sub(2).unwrap();
            required_fuel += divided;
        }

        required_fuel
    }
}
