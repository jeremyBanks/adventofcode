use crate::prelude::*;

pub fn solution() -> Solution {
    Solution {
        year: 2016,
        day: 21,
        code: |_input| ("TODO".to_string(), "TODO".to_string()),
    }
}

impl StringExt for String {}

trait StringExt: std::borrow::BorrowMut<String> {
    /// `swap position X with position Y` means that the letters at indexes X
    /// and Y (counting from 0) should be swapped.
    fn swap_positions(&mut self, _x: usize, _y: usize) -> eyre::Result<()> {
        Ok(())
    }

    /// `swap letter X with letter Y` means that the letters X and Y should be
    /// swapped
    fn swap_letters(&mut self, _x: char, _y: char) -> eyre::Result<()> {
        Ok(())
    }

    /// `rotate right X steps` means that the whole string should be rotated;
    /// for example, one right rotation would turn abcd into dabc.
    fn rotate_left(&mut self, _x: usize) -> eyre::Result<()> {
        Ok(())
    }

    /// `rotate left X steps` means that the whole string should be rotated; for
    /// example, one left rotation would turn dabc into abcd.
    fn rotate_right(&mut self, _x: usize) -> eyre::Result<()> {
        Ok(())
    }

    /// `rotate based on position of letter X` means that the whole string
    /// should be rotated to the right based on the index of letter X (counting
    /// from 0) as determined before this instruction does any rotations. Once
    /// the index is determined, rotate the string to the right one time, plus a
    /// number of times equal to that index, plus one additional time if the
    /// index was at least 4.
    fn rotate_based_on_letter(&mut self, _x: char) -> eyre::Result<()> {
        Ok(())
    }

    /// `reverse positions X through Y` means that the span of letters at
    /// indexes X through Y (including the letters at X and Y) should be
    /// reversed in order.
    fn reverse_span(&mut self, _x: char, _y: char) -> eyre::Result<()> {
        Ok(())
    }

    /// `move position X to position Y` means that the letter which is at index
    /// X should be removed from the string, then inserted such that it ends up
    /// at index Y.
    fn move_letter(&mut self, _x: char, _y: char) -> eyre::Result<()> {
        Ok(())
    }
}
