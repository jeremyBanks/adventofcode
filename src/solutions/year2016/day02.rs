use crate::prelude::*;

pub fn solution() -> Solution {
    Solution {
        year: 2016,
        day: 2,
        code: |input| {
            let keypad = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];

            let mut digits = String::new();

            let mut position: (i32, i32) = (1, 1);

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

            let diamond_keypad = [
                [None, None, Some('1'), None, None],
                [None, Some('2'), Some('3'), Some('4'), None],
                [Some('5'), Some('6'), Some('7'), Some('8'), Some('9')],
                [None, Some('A'), Some('B'), Some('C'), None],
                [None, None, Some('D'), None, None],
            ];

            let mut diamond_digits = String::new();

            let mut position: (i32, i32) = (0, 2);

            for line in input.lines() {
                for character in line.chars() {
                    let old_position = position;

                    match character {
                        'U' => position.1 = (position.1 - 1).clamp(0, 4),
                        'D' => position.1 = (position.1 + 1).clamp(0, 4),
                        'L' => position.0 = (position.0 - 1).clamp(0, 4),
                        'R' => position.0 = (position.0 + 1).clamp(0, 4),
                        _ => panic!("invalid input?"),
                    }

                    if diamond_keypad[position.1 as usize][position.0 as usize].is_none() {
                        position = old_position;
                    }
                }

                diamond_digits
                    .push(diamond_keypad[position.1 as usize][position.0 as usize].unwrap());
            }

            (digits, diamond_digits)
        },
    }
}
