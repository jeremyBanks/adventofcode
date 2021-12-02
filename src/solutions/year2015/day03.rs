crate::prelude!();

pub fn solution() {
    Solution {
        year: 2015,
        day: 3,
        code: |input| {
            let mut visited_alone: std::collections::HashSet<(i32, i32)> =
                [(0, 0)].into_iter().collect();
            {
                let mut x = 0;
                let mut y = 0;
                for char in input.chars() {
                    match char {
                        '>' => x += 1,
                        '<' => x -= 1,
                        '^' => y -= 1,
                        'v' => y += 1,
                        _ => unreachable!(),
                    }
                    visited_alone.insert((x, y));
                }
            }
            let visited_alone = visited_alone.len().try_into().unwrap();

            let mut visited_with_robot: std::collections::HashSet<(i32, i32)> =
                [(0, 0)].into_iter().collect();
            {
                let mut santa_x = 0;
                let mut santa_y = 0;
                let mut robot_x = 0;
                let mut robot_y = 0;

                for (i, char) in input.chars().enumerate() {
                    if i % 2 == 0 {
                        match char {
                            '>' => santa_x += 1,
                            '<' => santa_x -= 1,
                            '^' => santa_y -= 1,
                            'v' => santa_y += 1,
                            _ => unreachable!(),
                        }
                        visited_with_robot.insert((santa_x, santa_y));
                    } else {
                        match char {
                            '>' => robot_x += 1,
                            '<' => robot_x -= 1,
                            '^' => robot_y -= 1,
                            'v' => robot_y += 1,
                            _ => unreachable!(),
                        }
                        visited_with_robot.insert((robot_x, robot_y));
                    }
                }
            }
            let visited_with_robot = visited_with_robot.len().try_into().unwrap();

            (visited_alone, visited_with_robot)
        },
    }
}
