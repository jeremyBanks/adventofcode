use crate::prelude::*;

pub fn solution() -> Solution {
    Solution {
        year: 2018,
        day: 3,
        code: |input| {
            #[derive(Clone, Debug)]
            struct Claim {
                id: u32,
                x: u32,
                y: u32,
                width: u32,
                height: u32,
            }

            let mut claims = Vec::new();

            // example:
            // #11 @ 755,237: 24x22
            let pattern = regex::Regex::new(
                r"(?x)
                      ^\#
                      (?P<id>\d+)
                      \s@\s
                      (?P<x>\d+)
                      ,
                      (?P<y>\d+)
                      :\s
                      (?P<width>\d+)
                      x
                      (?P<height>\d+)
                      $",
            )
            .unwrap();

            for line in input.lines() {
                let pieces = pattern.captures(line).unwrap();
                claims.push(Claim {
                    id: pieces["id"].parse().unwrap(),
                    x: pieces["x"].parse().unwrap(),
                    y: pieces["y"].parse().unwrap(),
                    width: pieces["width"].parse().unwrap(),
                    height: pieces["height"].parse().unwrap(),
                });
            }

            let mut cell_counts = vec![0_u32; 1_000_000];
            let mut overcommitted_square_inches = 0;
            for claim in claims.iter() {
                for x in claim.x..(claim.x + claim.width) {
                    for y in claim.y..(claim.y + claim.height) {
                        let index = y * 1000 + x;
                        cell_counts[index as usize] += 1;
                        // count the cell only the first time it becomes overcommitted
                        if cell_counts[index as usize] == 2 {
                            overcommitted_square_inches += 1;
                        }
                    }
                }
            }
            let solution_3a = overcommitted_square_inches;

            let mut solution_3b = None;

            for claim in claims.iter() {
                let mut all_good = true;
                'out: for x in claim.x..(claim.x + claim.width) {
                    for y in claim.y..(claim.y + claim.height) {
                        let index = y * 1000 + x;
                        if cell_counts[index as usize] > 1 {
                            all_good = false;
                            break 'out;
                        }
                    }
                }
                if all_good {
                    solution_3b = Some(claim.id);
                }
            }
            let solution_3b = solution_3b.unwrap();

            (solution_3a.to_string(), solution_3b.to_string())
        },
    }
}
