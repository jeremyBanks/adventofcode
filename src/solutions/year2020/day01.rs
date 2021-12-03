use crate::prelude::*;

pub fn solution() -> Solution {
    Solution {
        year: 2020,
        day: 1,
        code: |input| {
            let numbers = input.lines_into::<i32>();

            let mut product_one: Option<i32> = None;

            for (i, x) in numbers.iter().copied().enumerate() {
                for (j, y) in numbers.iter().copied().enumerate() {
                    if i == j {
                        continue;
                    }

                    if x + y == 2020 {
                        product_one = Some(x * y);
                        break;
                    }
                }
            }

            let mut product_two: Option<i32> = None;

            for (i, x) in numbers.iter().copied().enumerate() {
                for (j, y) in numbers.iter().copied().enumerate() {
                    if i == j {
                        continue;
                    }
                    for (k, z) in numbers.iter().copied().enumerate() {
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

            (
                product_one.unwrap().to_string(),
                product_two.unwrap().to_string(),
            )
        },
    }
}
