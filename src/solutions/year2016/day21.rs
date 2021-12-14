use crate::prelude::*;

pub fn solution() -> Solution {
    Solution {
        year: 2016,
        day: 21,
        code: |input| {
            let iv = b"abcdefgh";
            let mut buffer = VecDeque::from_iter(*iv);

            lazy_static! {
                static ref SWAP_POSITIONS: Regex =
                    Regex::new(r"^swap position (\d+) with position (\d+)$").unwrap();
                static ref SWAP_LETTERS: Regex =
                    Regex::new(r"^swap letter ([a-z]) with letter ([a-z])$").unwrap();
                static ref ROTATE_LEFT: Regex = Regex::new(r"^rotate left (\d+) steps?$").unwrap();
                static ref ROTATE_RIGHT: Regex =
                    Regex::new(r"^rotate right (\d+) steps?$").unwrap();
                static ref ROTATE_BASED_ON_LETTER: Regex =
                    Regex::new(r"^rotate based on position of letter ([a-z]+)$").unwrap();
                static ref REVERSE_SPAN: Regex =
                    Regex::new(r"^reverse positions (\d+) through (\d+)$").unwrap();
                static ref MOVE_LETTER: Regex =
                    Regex::new(r"^move position (\d+) to position (\d+)$").unwrap();
                static ref COMMANDS: RegexSet = RegexSet::new(&[
                    SWAP_POSITIONS.to_string(),
                    SWAP_LETTERS.to_string(),
                    ROTATE_LEFT.to_string(),
                    ROTATE_RIGHT.to_string(),
                    ROTATE_BASED_ON_LETTER.to_string(),
                    REVERSE_SPAN.to_string(),
                    MOVE_LETTER.to_string(),
                ])
                .unwrap();
            }

            for line in input.lines() {
                if let Some(captures) = SWAP_POSITIONS.captures(line) {
                    let position_a: usize = captures[1].parse().unwrap();
                    let position_b: usize = captures[2].parse().unwrap();
                    buffer.swap(position_a, position_b);
                } else if let Some(captures) = SWAP_LETTERS.captures(line) {
                    let letter_a: u8 = captures[1].as_bytes()[0];
                    let letter_b: u8 = captures[2].as_bytes()[0];
                    for byte in buffer.iter_mut() {
                        if *byte == letter_a {
                            *byte = letter_b;
                        } else if *byte == letter_b {
                            *byte = letter_a;
                        }
                    }
                } else if let Some(captures) = ROTATE_LEFT.captures(line) {
                    let distance: usize = captures[1].parse().unwrap();
                    buffer.rotate_left(distance);
                } else if let Some(captures) = ROTATE_RIGHT.captures(line) {
                    let distance: usize = captures[1].parse().unwrap();
                    buffer.rotate_right(distance);
                } else if let Some(captures) = ROTATE_BASED_ON_LETTER.captures(line) {
                    let letter: u8 = captures[1].as_bytes()[0];
                    let index = line.as_bytes().iter().position(|l| *l == letter).unwrap();
                    let distance = 1 + index + if index >= 4 { 1 } else { 0 };
                    buffer.rotate_right(distance % buffer.len());
                } else if let Some(captures) = REVERSE_SPAN.captures(line) {
                    let position_a: usize = captures[1].parse().unwrap();
                    let position_b: usize = captures[2].parse().unwrap();
                    buffer.make_contiguous()[position_a..=position_b].reverse();
                } else if let Some(captures) = MOVE_LETTER.captures(line) {
                    let position_a: usize = captures[1].parse().unwrap();
                    let position_b: usize = captures[2].parse().unwrap();
                    let value = buffer.remove(position_a).unwrap();
                    buffer.insert(position_b, value);
                } else {
                    panic!("unexpected input? {:?}", line);
                }
            }

            (
                String::from_utf8(Vec::from_iter(buffer)).unwrap(),
                "TODO".to_string(),
            )
        },
    }
}
