#![feature(associated_type_defaults)]

mod solutions;

pub async fn main() -> () {}

pub trait Solution: Default {
    const YEAR: u32;
    const DAY: u32;

    fn solve(input: &str) -> (Self::PartOne, Self::PartTwo) {
        (Self::part_one(input), Self::part_two(input))
    }

    type PartOne: Default = ();
    fn part_one(input: &str) -> Self::PartOne {
        drop(input);
        Default::default()
    }

    type PartTwo: Default = ();
    fn part_two(input: &str) -> Self::PartTwo {
        drop(input);
        Default::default()
    }
}
