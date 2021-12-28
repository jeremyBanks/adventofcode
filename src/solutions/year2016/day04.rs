use crate::prelude::*;

pub fn solution() -> Solution {
    Solution {
        year: 2016,
        day: 4,
        code: |input| {
            let lines = input.lines_vec();

            let mut real_sector_ids_sum: u32 = 0;

            for line in lines {
                let line = line.as_bytes();
                let last_dash_index = line.iter().rposition(|b| *b == b'-').unwrap();
                let name = &line[..last_dash_index];
                let rest = &line[last_dash_index + 1..];
                let rest_split_index = rest.iter().rposition(|b| *b == b'[').unwrap();
                let sector_id = &rest[..rest_split_index];
                let checksum = &rest[rest_split_index + 1..rest.len() - 1];

                let name = str::from_utf8(name).unwrap();
                let sector_id = str::from_utf8(sector_id).unwrap();
                let sector_id: u32 = sector_id.parse().unwrap();
                let checksum = str::from_utf8(checksum).unwrap();

                let mut letter_frequency = HashMap::<char, i32>::new();
                for character in name.chars() {
                    *letter_frequency.entry(character).or_default() += 1;
                }

                let mut sorted_name_characters =
                    name.chars().unique().filter(|c| *c != '-').collect_vec();
                sorted_name_characters.sort_by_cached_key(|c| (-letter_frequency[c], *c));

                let actual_checksum = sorted_name_characters[..5].iter().join("");

                let is_real = checksum == actual_checksum;

                if is_real {
                    real_sector_ids_sum += sector_id;
                }
                dbg!(is_real, actual_checksum, checksum);
            }

            (real_sector_ids_sum.to_string(), String::new())
        },
    }
}
