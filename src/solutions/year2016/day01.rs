#[derive(Default, Debug)]
pub struct Solution {}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Directions {
    North,
    West,
    South,
    East,
}

impl Directions {
    fn x(&self) -> i32 {
        use Directions::*;
        match self {
            West => -1,
            East => 1,
            North | South => 0,
        }
    }

    fn y(&self) -> i32 {
        use Directions::*;
        match self {
            South => -1,
            North => 1,
            West | East => 0,
        }
    }

    fn left(&self) -> Self {
        use Directions::*;
        match self {
            West => North,
            South => West,
            East => South,
            North => East,
        }
    }

    fn right(&self) -> Self {
        use Directions::*;
        match self {
            North => West,
            West => South,
            South => East,
            East => North,
        }
    }
}

impl crate::Solution for Solution {
    const YEAR: u32 = 2016;
    const DAY: u32 = 1;

    type PartOne = u32;
    type PartTwo = u32;
    fn solve(input: &str) -> (u32, u32) {
        let instructions: Vec<&str> = input.split(", ").collect();

        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut direction = Directions::North;
        let mut visited: std::collections::HashSet<(i32, i32)> = [(0, 0)].into_iter().collect();
        let mut distance_two: Option<u32> = None;

        for instruction in instructions {
            let turn = instruction.chars().nth(0).unwrap();
            let distance = instruction.chars().skip(1).collect::<String>();
            let distance = distance.parse::<i32>().unwrap();

            direction = match turn {
                'L' => direction.left(),
                'R' => direction.right(),
                _ => unreachable!(),
            };

            x += direction.x() * distance;
            y += direction.y() * distance;
            // we don't just check the endpoints, we need to check every intermediate value?

            if distance_two.is_none() {
                let already_visited = !visited.insert((x, y));
                if already_visited {
                    distance_two = Some((x.abs() + y.abs()).try_into().unwrap());
                }
            }
        }

        let distance_one = x.abs() + y.abs();
        let distance_one = distance_one.try_into().unwrap();

        let distance_two = distance_two.unwrap();

        (distance_one, distance_two)
    }
}
