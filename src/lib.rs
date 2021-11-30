mod solutions;

pub async fn main() -> () {}

pub trait Solution: Default {
    const YEAR: u32;
    const DAY: u32;
    type Part1 = ();
    type Part2 = ();

    fn run(input: &str) -> (Self::Part1, Self::Part2) {}
}
