#[derive(Default, Debug)]
pub struct Solution {}

impl crate::Solution for Solution {
    const YEAR: u32 = 2017;
    const DAY: u32 = 1;

    type PartOne = u32;
    fn part_one(input: &str) -> u32 {
        let digits = input
            .chars()
            .map(|n| n.to_string().parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        let mut sum = 0;
        for (i, digit) in digits.iter().enumerate() {
            let next = digits[(i + 1) % input.len()];
            if *digit == next {
                sum += digit;
            }
        }
        sum
    }

    type PartTwo = u32;
    fn part_two(input: &str) -> u32 {
        let digits = input
            .chars()
            .map(|n| n.to_string().parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let offset = digits.len() / 2;

        let mut sum = 0;
        for (i, digit) in digits.iter().enumerate() {
            let next = digits[(i + offset) % input.len()];
            if *digit == next {
                sum += digit;
            }
        }
        sum
    }
}
