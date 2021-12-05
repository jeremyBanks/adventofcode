use crate::prelude::*;

pub fn solution() -> Solution {
    Solution {
        year: 2018,
        day: 1,
        code: |input| {
            let numbers: Vec<i32> = input
                .lines()
                .map(|s| s.parse().expect("non-integer in input"))
                .collect();
            let frequency_sum = numbers.iter().copied().sum::<i32>();

            let mut seen = HashSet::new();
            let mut sum: i32 = 0;
            let mut first_repeated_sum = None;
            for number in numbers.iter().cycle() {
                sum += number;
                if seen.contains(&sum) {
                    first_repeated_sum = Some(sum);
                    break;
                } else {
                    seen.insert(sum);
                }
            }
            let first_repeated_sum = first_repeated_sum.unwrap();

            (frequency_sum.to_string(), first_repeated_sum.to_string())
        },
    }
}
