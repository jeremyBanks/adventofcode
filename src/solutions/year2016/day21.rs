use crate::prelude::*;

lazy_static::lazy_static! {
    static ref SWAP_POSITIONS: Regex = Regex::new(r"^swap position (\d+) with position (\d+)$").unwrap();
    static ref SWAP_LETTERS: Regex = Regex::new(r"^swap letter ([a-z]) with letter ([a-z])$").unwrap();
    static ref ROTATE_LEFT: Regex = Regex::new(r"^rotate left (\d+) steps?$").unwrap();
    static ref ROTATE_RIGHT: Regex = Regex::new(r"^rotate right (\d+) steps?$").unwrap();
    static ref ROTATE_BASED_ON_LETTER: Regex = Regex::new(r"^rotate based on position of letter ([a-z]+)$").unwrap();
    static ref REVERSE_SPAN: Regex = Regex::new(r"^reverse positions (\d+) through (\d+)$").unwrap();
    static ref MOVE_LETTER: Regex = Regex::new(r"^move position (\d+) to position (\d+)$").unwrap();
}

pub fn solution() -> Solution {
    Solution {
        year: 2016,
        day: 21,
        code: |input| {
            let _buffer = b"abcdefgh";

            for line in input.lines() {
                // XXX: Can re::RegExSet improve performance?
                if let Some(captures) = SWAP_POSITIONS.captures(line) {
                    let position_a: usize = captures[1].parse().unwrap();
                    let position_b: usize = captures[2].parse().unwrap();
                    dbg!(&captures[0], position_a, position_b);
                } else if let Some(captures) = SWAP_LETTERS.captures(line) {
                    let letter_a: u8 = captures[1].as_bytes()[0];
                    let letter_b: u8 = captures[2].as_bytes()[0];
                    dbg!(&captures[0], letter_a, letter_b);
                } else if let Some(captures) = ROTATE_LEFT.captures(line) {
                    let distance: usize = captures[1].parse().unwrap();
                    dbg!(&captures[0], distance);
                } else if let Some(captures) = ROTATE_RIGHT.captures(line) {
                    let distance: usize = captures[1].parse().unwrap();
                    dbg!(&captures[0], distance);
                } else if let Some(captures) = ROTATE_BASED_ON_LETTER.captures(line) {
                    let letter: u8 = captures[1].as_bytes()[0];
                    dbg!(&captures[0], letter);
                } else if let Some(captures) = REVERSE_SPAN.captures(line) {
                    let position_a: usize = captures[1].parse().unwrap();
                    let position_b: usize = captures[2].parse().unwrap();
                    dbg!(&captures[0], position_a, position_b);
                } else if let Some(captures) = MOVE_LETTER.captures(line) {
                    let position_a: usize = captures[1].parse().unwrap();
                    let position_b: usize = captures[2].parse().unwrap();
                    dbg!(&captures[0], position_a, position_b);
                } else {
                    panic!("unexpected input? {:?}", line);
                }
            }

            ("TODO".to_string(), "TODO".to_string())
        },
    }
}
