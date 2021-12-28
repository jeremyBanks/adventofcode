use crate::prelude::*;

pub fn solution() -> Solution {
    Solution {
        year: 2016,
        day: 3,
        code: |input| {
            let mut valid_count = 0;
            let mut valid_count_b = 0;

            let lines = input.lines_vec();
            for line in lines.iter() {
                if line.is_empty() {
                    continue;
                }

                let sides = line
                    .split_whitespace()
                    .map(|s| s.i64())
                    .sorted()
                    .collect_vec();

                if sides[0] + sides[1] > sides[2] {
                    valid_count += 1;
                }
            }

            let mut array = vec![];
            for line in input.lines() {
                if line.is_empty() {
                    continue;
                }

                array.push(
                    line.split_whitespace()
                        .map(|s| s.i64())
                        .sorted()
                        .collect_vec(),
                );
            }

            for y in (0..lines.len()).step_by(3) {
                for x in [0, 1, 2] {
                    let sides = [
                        lines[y.usize() + 0]
                            .split_whitespace()
                            .nth(x)
                            .unwrap()
                            .i64(),
                        lines[y.usize() + 1]
                            .split_whitespace()
                            .nth(x)
                            .unwrap()
                            .i64(),
                        lines[y.usize() + 2]
                            .split_whitespace()
                            .nth(x)
                            .unwrap()
                            .i64(),
                    ]
                    .into_iter()
                    .sorted()
                    .collect_vec();

                    if sides[0] + sides[1] > sides[2] {
                        valid_count_b += 1;
                    }
                }
            }

            (valid_count.to_string(), valid_count_b.to_string())
        },
    }
}
