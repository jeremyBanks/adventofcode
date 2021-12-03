use nom::{
    character::complete::digit1,
    combinator::{map, map_res, recognize},
    Parser,
};

use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum RelativeDirection {
    Right,
    Left,
}

pub fn parse_direction(
    input: &str,
) -> nom::IResult<&str, (RelativeDirection, u32), nom_supreme::error::ErrorTree<&str>> {
    let (input, direction) = nom::branch::alt((
        map(nom_supreme::tag::complete::tag("L"), |_| {
            RelativeDirection::Left
        }),
        map(nom_supreme::tag::complete::tag("R"), |_| {
            RelativeDirection::Right
        }),
    ))(input)?;

    let (input, magnitude) = map_res(recognize(digit1), str::parse::<u32>)(input)?;

    Ok((input, (direction, magnitude)))
}

pub fn solution() -> Solution {
    Solution {
        year: 2016,
        day: 1,
        code: |input| {
            let instructions = input.split_vec(", ");

            let mut x: i32 = 0;
            let mut y: i32 = 0;
            let mut direction = CardinalDirection::North;
            let mut visited: HashSet<(i32, i32)> = [(0, 0)].into_iter().collect();
            let mut distance_two: Option<u32> = None;

            for instruction in instructions {
                let turn = instruction.chars().next().unwrap();
                let distance = instruction.chars().skip(1).collect::<String>();
                let distance = distance.parse::<i32>().unwrap();

                direction = match turn {
                    'L' => direction.left(),
                    'R' => direction.right(),
                    _ => unreachable!(),
                };

                if direction == CardinalDirection::East || direction == CardinalDirection::West {
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
            let distance_one = distance_one.to_string();

            let distance_two = distance_two.unwrap().to_string();

            (distance_one, distance_two)
        },
    }
}
