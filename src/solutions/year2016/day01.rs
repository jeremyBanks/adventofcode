crate::prelude!();

pub fn solution() -> Solution {
    Solution {
        year: 2016,
        day: 1,
        code: |input| {
            let instructions: Vec<&str> = input.split(", ").collect();

            let mut x: i32 = 0;
            let mut y: i32 = 0;
            let mut direction = Direction::North;
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

                if direction == Direction::East || direction == Direction::West {
                    for _ in 0..distance {
                        x += direction.x();
                        if distance_two.is_none() {
                            let already_visited = !visited.insert((x, y));
                            if already_visited {
                                distance_two = Some((x.abs() + y.abs()).try_into().unwrap());
                            }
                        }
                    }
                } else {
                    for _ in 0..distance {
                        y += direction.y();
                        if distance_two.is_none() {
                            let already_visited = !visited.insert((x, y));
                            if already_visited {
                                distance_two = Some((x.abs() + y.abs()).try_into().unwrap());
                            }
                        }
                    }
                }
            }

            let distance_one = x.abs() + y.abs();
            let distance_one = distance_one.try_into().unwrap();

            let distance_two = distance_two.unwrap();

            (distance_one, distance_two)
        },
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn x(&self) -> i32 {
        use Direction::*;
        match self {
            West => -1,
            East => 1,
            North | South => 0,
        }
    }

    fn y(&self) -> i32 {
        use Direction::*;
        match self {
            South => -1,
            North => 1,
            West | East => 0,
        }
    }

    fn left(&self) -> Self {
        use Direction::*;
        match self {
            West => North,
            South => West,
            East => South,
            North => East,
        }
    }

    fn right(&self) -> Self {
        use Direction::*;
        match self {
            North => West,
            West => South,
            South => East,
            East => North,
        }
    }
}
