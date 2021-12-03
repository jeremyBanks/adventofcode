use crate::prelude::*;

pub fn solution() -> Solution {
    Solution {
        year: 2021,
        day: 3,
        code: |input| {
            let lines = input.lines_vec();

            let bits_per_word = lines[0].len();
            let words = lines.len();

            let mut frequencies = vec![0usize; bits_per_word];
            for line in lines {
                for (i, c) in line.chars().enumerate() {
                    match c {
                        '1' => frequencies[i] += 1,
                        '0' => {}
                        _ => unreachable!(),
                    }
                }
            }
            let half = words / 2;

            let mut gamma = 0u64;
            let mut epsilon = 064;
            for frequency in frequencies.iter().copied() {
                gamma *= 2;
                epsilon *= 2;

                if frequency > half {
                    gamma += 1;
                } else if frequency < half {
                    epsilon += 1;
                } else {
                    panic!("is this defined?");
                }
            }

            dbg!(frequencies, half);
            let power_consumption = gamma * epsilon;

            (power_consumption.to_string(), String::default())
        },
    }
}
