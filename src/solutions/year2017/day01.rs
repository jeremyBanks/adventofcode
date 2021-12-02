crate::prelude!();

pub fn solution() -> Solution {
    Solution {
        year: 2017,
        day: 1,
        code: |input| {
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
            let part_one = sum.to_string();

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
            let part_two = sum.to_string();

            (part_one, part_two)
        },
    }
}
