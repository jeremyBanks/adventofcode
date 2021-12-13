use crate::prelude::*;

pub fn solution() -> Solution {
    Solution {
        year: 2016,
        day: 2,
        code: |input| {
            let keypad = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];

            let mut digits = String::new();

            // (0, 0) is top-left (1)
            // (2, 2) is bottom-right (9)
            // (1, 1) is start position (5)
            let mut position = (1i32, 1i32);

            for line in input.lines() {
                for character in line.chars() {
                    match character {
                        'U' => position.1 = (position.1 - 1).clamp(0, 2),
                        'D' => position.1 = (position.1 + 1).clamp(0, 2),
                        'L' => position.0 = (position.0 - 1).clamp(0, 2),
                        'R' => position.0 = (position.0 + 1).clamp(0, 2),
                        _ => panic!("invalid input?"),
                    }
                }

                digits.push(keypad[position.1 as usize][position.0 as usize]);
            }

            (digits, "TODO".to_string())
        },
    }
}
