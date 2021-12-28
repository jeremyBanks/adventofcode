use crate::prelude::*;

pub struct Solution {
    pub year: u32,
    pub day: u32,
    pub code: fn(&str) -> (String, String),
}

pub trait DebugExt: Debug {
    fn to_debug(&self) -> String {
        format!("{:?}", self)
    }

    fn to_debug_pretty(&self) -> String {
        format!("{:#?}", self)
    }
}

impl<T: Debug> DebugExt for T {}

pub trait StrExt: AsRef<str> {
    fn parse_into<T: FromStr>(&self) -> T {
        self.as_ref()
            .parse()
            .unwrap_or_else(|_| panic!("parse of {:?} failed", self.as_ref()))
    }

    fn split_vec(&self, delimiter: &str) -> Vec<&str> {
        self.as_ref().split(delimiter).collect()
    }

    fn lines_into<T: FromStr>(&self) -> Vec<T> {
        self.as_ref()
            .lines()
            .map(|x| {
                x.parse::<T>()
                    .unwrap_or_else(|_| panic!("couldn't parse {:?}", x))
            })
            .collect()
    }

    fn split_into<T: FromStr>(&self, delimiter: &str) -> Vec<T> {
        self.as_ref()
            .split(delimiter)
            .map(|x| {
                x.parse::<T>()
                    .unwrap_or_else(|_| panic!("couldn't parse {:?}", x))
            })
            .collect()
    }

    fn lines_vec(&self) -> Vec<&str> {
        self.as_ref().lines().collect_vec()
    }

    fn partition(&self, separator: &str) -> (&str, &str) {
        let this = self.as_ref();
        match this.find(separator) {
            Some(index) => (&this[..index], &this[index + 1..]),
            None => (this, ""),
        }
    }

    fn rpartition(&self, separator: &str) -> (&str, &str) {
        let this = self.as_ref();
        match this.rfind(separator) {
            Some(index) => (&this[..index], &this[index + 1..]),
            None => (this, ""),
        }
    }
}

impl<T: AsRef<str>> StrExt for T {}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CardinalDirection {
    North,
    West,
    South,
    East,
}

impl CardinalDirection {
    pub fn x(&self) -> i32 {
        use CardinalDirection::*;
        match self {
            West => -1,
            East => 1,
            North | South => 0,
        }
    }

    pub fn y(&self) -> i32 {
        use CardinalDirection::*;
        match self {
            South => -1,
            North => 1,
            West | East => 0,
        }
    }

    pub fn left(&self) -> Self {
        use CardinalDirection::*;
        match self {
            West => North,
            South => West,
            East => South,
            North => East,
        }
    }

    pub fn right(&self) -> Self {
        use CardinalDirection::*;
        match self {
            North => West,
            West => South,
            South => East,
            East => North,
        }
    }
}

pub fn is_digit(c: &char) -> bool {
    ('0'..='9').contains(c)
}

pub fn is_not_digit(c: &char) -> bool {
    !('0'..='9').contains(c)
}
