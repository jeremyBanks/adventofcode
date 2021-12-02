#![allow(unused_imports)]

pub use std::{
    any::Any,
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::{Debug, Display},
    rc::Rc,
    str::FromStr,
    time::{Duration, Instant},
};

pub use boolinator::Boolinator;
pub use bytes::{Buf, BufMut, Bytes, BytesMut};
pub use crossbeam::channel::{bounded, unbounded};
pub use eyre::{eyre, WrapErr};
pub use itertools::Itertools;
pub use parking_lot::{Condvar, Mutex, Once, ReentrantMutex, RwLock};
pub use rayon::{
    iter::{
        FromParallelIterator, IndexedParallelIterator, IntoParallelIterator,
        IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelBridge, ParallelDrainFull,
        ParallelDrainRange, ParallelExtend, ParallelIterator,
    },
    slice::{ParallelSlice, ParallelSliceMut},
    str::ParallelString,
};
pub use thousands::Separable;

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
        self.as_ref().lines().collect()
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
