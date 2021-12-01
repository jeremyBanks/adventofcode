use crate::prelude::*;

#[derive(Default, Debug)]
pub struct Solution {}

impl crate::Solution for Solution {
    const YEAR: u32 = 2019;
    const DAY: u32 = 1;

    type PartOne = u32;
    type PartTwo = u32;
    fn solve(input: &str) -> (u32, u32) {
        let mut total_fuel_one = 0;
        let mut total_fuel_two = 0;

        let masses = split_and_parse::<u32>(input, "\n");
        for mass in masses {
            let mut mass = mass;
            for i in 0.. {
                let fuel = (mass / 3).checked_sub(2).unwrap_or_default();
                if fuel > 0 {
                    if i == 0 {
                        total_fuel_one += fuel;
                    }
                    total_fuel_two += fuel;
                    mass = fuel;
                } else {
                    break;
                }
            }
        }

        (total_fuel_one, total_fuel_two)
    }
}
