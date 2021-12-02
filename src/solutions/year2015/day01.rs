crate::prelude!();

pub fn solution() -> Solution {
    Solution {
        year: 2015,
        day: 1,
        code: |input| {
            let mut floor: i32 = 0;
            let mut instructions_to_basement = None;
            for (i, char) in input.chars().enumerate() {
                floor += match char {
                    ')' => -1,
                    '(' => 1,
                    _ => unreachable!(),
                };
                if instructions_to_basement.is_none() && floor < 0 {
                    instructions_to_basement = Some(i + 1);
                }
            }

            (floor.to_string(), instructions_to_basement.to_debug())
        },
    }
}
