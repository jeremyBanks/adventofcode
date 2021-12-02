pub use std::{
    any::Any,
    fmt::{Debug, Display},
    rc::Rc,
    str::FromStr,
    sync::Arc,
    time::{Duration, Instant},
};

pub use eyre::{Report, Result};
pub use thousands::Separable;

macro_rules! prelude {
    () => {
        use crate::prelude::*;
    };
}

pub(crate) use prelude;

pub struct Solution {
    pub year: u32,
    pub day: u32,
    pub code: fn(&str) -> (String, String),
}

pub const UNIMPLEMENTED: &'static str = "unimplemented!()";

pub trait DebugExt: Debug {
    fn to_debug(&self) -> String {
        format!("{:?}", self)
    }

    fn to_debug_pretty(&self) -> String {
        format!("{:#?}", self)
    }
}

impl<T: Debug> DebugExt for T {}

pub fn split_and_parse<T: FromStr + Debug>(input: &str, delimiter: &str) -> Vec<T> {
    input
        .split(delimiter)
        .map(|x| {
            x.parse::<T>()
                .unwrap_or_else(|_| panic!("couldn't parse {:?}", x))
        })
        .collect()
}
