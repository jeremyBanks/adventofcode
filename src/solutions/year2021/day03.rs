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
                    panic!(
                        "i don't handle the case where they're tied 'cause that's not in my input"
                    );
                }
            }

            let power_consumption = gamma * epsilon;

            let mut potential_generator_ratings: HashSet<_> = lines.iter().collect();
            for (bit_index, ones_count) in frequencies.iter().copied().enumerate() {
                if potential_generator_ratings.len() == 1 {
                    break;
                }

                let expected_bit = if ones_count >= half { '1' } else { '0' };
                potential_generator_ratings
                    .retain(|rating| rating.chars().nth(bit_index).unwrap() == expected_bit);
            }
            assert!(potential_generator_ratings.len() == 1);
            let generator_rating =
                i64::from_str_radix(potential_generator_ratings.iter().next().unwrap(), 2).unwrap();

            let mut potential_scrubber_ratings: HashSet<_> = lines.iter().collect();
            for (bit_index, ones_count) in frequencies.iter().copied().enumerate() {
                if potential_scrubber_ratings.len() == 1 {
                    break;
                }

                let expected_bit = if ones_count >= half { '0' } else { '1' };
                potential_scrubber_ratings
                    .retain(|rating| rating.chars().nth(bit_index).unwrap() == expected_bit);
            }
            assert!(potential_scrubber_ratings.len() == 1);
            let scrubber_rating =
                i64::from_str_radix(potential_scrubber_ratings.iter().next().unwrap(), 2).unwrap();

            let life_support_rating = generator_rating * scrubber_rating;

            (
                power_consumption.to_string(),
                life_support_rating.to_string(),
            )
        },
    }
}
