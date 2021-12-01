// Functions that are

pub use std::{fmt::Debug, str::FromStr};

pub fn split_and_parse<T: FromStr + Debug>(input: &str, delimiter: &str) -> Vec<T> {
    input
        .split(delimiter)
        .map(|x| {
            x.parse::<T>()
                .unwrap_or_else(|_| panic!("couldn't parse {:?}", x))
        })
        .collect()
}
