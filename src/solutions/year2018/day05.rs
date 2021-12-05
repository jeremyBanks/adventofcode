use crate::prelude::*;

pub fn solution() -> Solution {
    Solution {
        year: 2018,
        day: 5,
        code: |input| {
            fn flip_case(unit: char) -> char {
                match unit {
                    'a'..='z' => unit.to_ascii_uppercase(),
                    'A'..='Z' => unit.to_ascii_lowercase(),
                    _ => panic!("unexpected character in input"),
                }
            }

            fn post_reaction_size(units: impl Iterator<Item = char>) -> u32 {
                let mut reactor = VecDeque::new();
                let mut hopper = units.collect::<VecDeque<_>>();

                assert!(hopper.len() >= 2);

                while !hopper.is_empty() {
                    let left = reactor.pop_back().or_else(|| hopper.pop_front()).unwrap();
                    let right = hopper.pop_front().unwrap();

                    if left == flip_case(right) {
                        // they annihilate each other and we drop them
                    } else {
                        // they survive in the reaction chamber
                        reactor.push_back(left);
                        reactor.push_back(right);
                    }
                }

                reactor.len() as u32
            }

            let solution_5a = post_reaction_size(input.chars());

            let solution_5b = {
                let unit_types: HashSet<_> =
                    input.chars().map(|c| c.to_ascii_lowercase()).collect();
                let mut best_unit_size = u32::MAX;
                for unit_lower in unit_types.iter() {
                    let unit_upper = unit_lower.to_ascii_uppercase();
                    let filtered_input = input
                        .chars()
                        .filter(|c| *c != unit_upper && *c != *unit_lower);
                    let size = post_reaction_size(filtered_input);
                    if size < best_unit_size {
                        best_unit_size = size;
                    }
                }

                best_unit_size
            };

            (solution_5a.to_string(), solution_5b.to_string())
        },
    }
}
