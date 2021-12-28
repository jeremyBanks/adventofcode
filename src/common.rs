#![allow(unused)]

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

pub fn int(s: &str) -> i64 {
    s.parse().unwrap()
}

pub fn float(s: &str) -> f64 {
    s.parse().unwrap()
}

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

    fn usize(&self) -> usize {
        self.as_ref()
            .parse()
            .unwrap_or_else(|_| panic!("could not parse {:?} as usize", self.as_ref()))
    }
    fn i64(&self) -> i64 {
        self.as_ref()
            .parse()
            .unwrap_or_else(|_| panic!("could not parse {:?} as i64", self.as_ref()))
    }
    fn f64(&self) -> f64 {
        self.as_ref()
            .parse()
            .unwrap_or_else(|_| panic!("could not parse {:?} as f64", self.as_ref()))
    }
    fn u64(&self) -> u64 {
        self.as_ref()
            .parse()
            .unwrap_or_else(|_| panic!("could not parse {:?} as u64", self.as_ref()))
    }
    fn u8(&self) -> u8 {
        self.as_ref()
            .parse()
            .unwrap_or_else(|_| panic!("could not parse as u8"))
    }
    fn isize(&self) -> isize {
        self.as_ref()
            .parse()
            .unwrap_or_else(|_| panic!("could not parse as isize"))
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

pub const LOWERCASE_ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz";

pub fn rot13(s: impl AsRef<str>) -> String {
    rot(s, LOWERCASE_ALPHABET, 13)
}

pub fn rot_lowercase(s: impl AsRef<str>, distance: impl Int) -> String {
    rot(s, LOWERCASE_ALPHABET, distance)
}

pub fn rot(s: impl AsRef<str>, alphabet: &str, distance: impl Int) -> String {
    let mut s = s.as_ref().chars().collect_vec();

    let distance = distance.i64();

    let alphabet_size = alphabet.len().i64();

    for c in s.iter_mut() {
        let index = alphabet.find(*c);
        match index {
            Some(index) => {
                let rotated = ((index.i64() + distance) % alphabet_size).usize();
                *c = alphabet.chars().nth(rotated.usize()).unwrap();
            },
            None =>
                if *c == '-' {
                    *c = ' ';
                } else {
                    dbg!(&c, &index);
                    panic!("should we be ignoring this?");
                },
        }
    }

    s.iter().join("")
}

pub trait Int: Sized {
    fn usize(self) -> usize;
    fn i64(self) -> i64;
    fn u64(self) -> u64;
    fn u8(self) -> u8;
    fn isize(self) -> isize;
}

impl<T: TryInto<usize> + TryInto<i64> + TryInto<u64> + TryInto<u8> + TryInto<isize>> Int for T {
    fn usize(self) -> usize {
        self.try_into()
            .unwrap_or_else(|_| panic!("could not convert to usize"))
    }
    fn i64(self) -> i64 {
        self.try_into()
            .unwrap_or_else(|_| panic!("could not convert to i64"))
    }
    fn u64(self) -> u64 {
        self.try_into()
            .unwrap_or_else(|_| panic!("could not convert to u64"))
    }
    fn u8(self) -> u8 {
        self.try_into()
            .unwrap_or_else(|_| panic!("could not convert to u8"))
    }
    fn isize(self) -> isize {
        self.try_into()
            .unwrap_or_else(|_| panic!("could not convert to isize"))
    }
}
