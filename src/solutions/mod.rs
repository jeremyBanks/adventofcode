use crate::prelude::*;

pub mod year2015;
pub mod year2016;
pub mod year2017;
pub mod year2018;
pub mod year2019;
pub mod year2020;
pub mod year2021;

pub fn solutions() -> Vec<Solution> {
    let mut solutions = vec![];
    solutions.append(&mut year2015::solutions());
    solutions.append(&mut year2016::solutions());
    solutions.append(&mut year2017::solutions());
    solutions.append(&mut year2018::solutions());
    solutions.append(&mut year2019::solutions());
    solutions.append(&mut year2020::solutions());
    solutions.append(&mut year2021::solutions());
    solutions
}
