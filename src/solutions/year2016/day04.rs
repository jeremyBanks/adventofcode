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
                dbg!(name);
            }

            (String::new(), String::new())
        },
    }
}
