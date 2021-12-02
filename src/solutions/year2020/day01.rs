crate::prelude!();

pub fn solution() -> Solution {
    Solution {
        year: 2020,
        day: 1,
        code: |input| {
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

            (
                product_one.unwrap().to_string(),
                product_two.unwrap().to_string(),
            )
        },
    }
}
