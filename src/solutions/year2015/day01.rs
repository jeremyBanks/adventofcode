#[derive(Default, Debug)]
pub struct Solution {}

impl crate::Solution for Solution {
    const YEAR: u32 = 2015;
    const DAY: u32 = 1;

    type PartOne = i32;
    type PartTwo = usize;
    fn solve(input: &str) -> (i32, usize) {
        let mut floor = 0;
        let mut instructions_to_basement = None;
        for (i, char) in input.chars().enumerate() {
            floor += match char {
                ')' => -1,
                '(' => 1,
                _ => unreachable!(),
            };
            if instructions_to_basement.is_none() && floor < 0 {
                instructions_to_basement = Some(i + 1);
            }
        }
        (floor, instructions_to_basement.unwrap())
    }
}
