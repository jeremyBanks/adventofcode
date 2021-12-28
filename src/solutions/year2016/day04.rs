use crate::prelude::*;

pub fn solution() -> Solution {
    Solution {
        year: 2016,
        day: 4,
        code: |input| {
            let mut real_sector_ids_sum = 0;
            let mut north_pole_sector_id = 0;

            for line in input.lines() {
                // partition the line at last `-` to get name
                let (name, rest) = line.rpartition("-");

                // partition the rest at first `[` to get the sector ID
                let (sector_id, rest) = rest.partition("[");
                let sector_id = int(sector_id);

                // remove the last character (`]`) from the rest to get the checksum
                let checksum = &rest[..rest.len() - 1];

                let correct_checksum = {
                    let mut letter_counts = HashMap::<char, i32>::new();
                    for character in name.chars() {
                        if ('a'..='z').contains(&character) {
                            *letter_counts.entry(character).or_default() += 1;
                        } else {
                            assert!(character == '-', "unexpected character");
                        }
                    }

                    // Sort letters by most-frequent, followed by alphabetically, then join the
                    // top 5 as a String to get our result.
                    letter_counts
                        .keys()
                        .sorted_by_cached_key(|c| (-letter_counts[c], *c))
                        .collect_vec()[..5]
                        .iter()
                        .join("")
                };

                if checksum == correct_checksum {
                    real_sector_ids_sum += sector_id;
                    let decoded = rot_lowercase(name, sector_id);
                    if decoded.contains("northpole") {
                        north_pole_sector_id = sector_id;
                        dbg!(decoded, sector_id);
                    }
                }
            }

            (
                real_sector_ids_sum.to_string(),
                north_pole_sector_id.to_string(),
            )
        },
    }
}
