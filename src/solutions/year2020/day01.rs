#[derive(Default, Debug)]
pub struct Solution {}

impl crate::Solution for Solution {
    const YEAR: u32 = 2020;
    const DAY: u32 = 1;

    type PartOne = u64;
    type PartTwo = u64;
    fn solve(input: &str) -> (u64, u64) {
        let numbers = Vec::from_iter(input.split("\n").map(|n| n.parse::<u64>().unwrap()));

        let mut product_one: Option<u64> = None;

        for (i, x) in numbers.iter().cloned().enumerate() {
            for (j, y) in numbers.iter().cloned().enumerate() {
                if i == j {
                    continue;
                }

                if x + y == 2020 {
                    product_one = Some(x * y);
                    break;
                }
            }
        }

        let mut product_two: Option<u64> = None;

        for (i, x) in numbers.iter().cloned().enumerate() {
            for (j, y) in numbers.iter().cloned().enumerate() {
                if i == j {
                    continue;
                }
                for (k, z) in numbers.iter().cloned().enumerate() {
                    if i == k || j == k {
                        continue;
                    }
                    if x + y + z == 2020 {
                        product_two = Some(x * y * z);
                        break;
                    }
                }
            }
        }

        (product_one.unwrap(), product_two.unwrap())
    }
}
