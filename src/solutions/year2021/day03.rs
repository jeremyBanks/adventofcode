use crate::prelude::*;

pub fn solution() -> Solution {
    Solution {
        year: 2021,
        day: 3,
        code: |input| {
            let lines = input.lines_vec();

            let bits_per_line = lines[0].len();

            let mut frequencies = vec![0_usize; bits_per_line];
            for line in lines.iter() {
                for (i, c) in line.chars().enumerate() {
                    match c {
                        '1' => frequencies[i] += 1,
                        '0' => {},
                        _ => unreachable!(),
                    }
                }
            }
            let half = lines.len() / 2;

            let mut gamma = 0_u64;
            let mut epsilon = 0_u64;
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

            let power_consumption = gamma * epsilon;

            let potential_generator_ratings: HashSet<_> = lines.iter().collect();
                for (bit_index, ones_count) in frequencies.iter().copied().enumerate() {
                    let expected_bit_value = if ones_count 

                    if potential_generator_ratings.len() == 1 {
                        break;
                    }
                }
            assert!(potential_generator_ratings.len() == 1);
            let generator_rating =
                i64::from_str_radix(potential_generator_ratings.iter().next().unwrap(), 2).unwrap();

            let potential_scrubber_ratings: HashSet<_> = lines.iter().collect();
            while potential_scrubber_ratings.len() > 1 {
                for (bit_index, ones_count) in frequencies.iter().copied().enumerate() {
                    if potential_scrubber_ratings.len() == 1 {
                        break;
                    }
                }
            }
            assert!(potential_scrubber_ratings.len() == 1);
            let scrubber_rating =
                i64::from_str_radix(potential_generator_ratings.iter().next().unwrap(), 2).unwrap();

            let life_support_rating = generator_rating * scrubber_rating;

            (
                power_consumption.to_string(),
                life_support_rating.to_string(),
            )
        },
    }
}
