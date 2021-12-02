crate::prelude!();

pub fn solution() -> Solution {
    Solution {
        year: 2021,
        day: 2,
        code: |input| {
            let commands = input.split("\n").collect::<Vec<_>>();

            let part_a = {
                let mut depth: i32 = 0;
                let mut position: i32 = 0;
                for command in commands.iter() {
                    let parts = command.split(" ").collect::<Vec<_>>();
                    let direction = parts[0];
                    let magnitude = parts[1].parse::<i32>().unwrap();

                    match direction {
                        "up" => depth -= magnitude,
                        "down" => depth += magnitude,
                        "forward" => position += magnitude,
                        _ => unreachable!(),
                    }
                }

                (position * depth).to_string()
            };

            let part_b = {
                let mut aim: i32 = 0;
                let mut depth: i32 = 0;
                let mut position: i32 = 0;

                for command in commands.iter() {
                    let parts = command.split(" ").collect::<Vec<_>>();
                    let direction = parts[0];
                    let magnitude = parts[1].parse::<i32>().unwrap();

                    match direction {
                        "up" => aim -= magnitude,
                        "down" => aim += magnitude,
                        "forward" => {
                            position += magnitude;
                            depth += magnitude * aim;
                        }
                        _ => unreachable!(),
                    }
                }

                (position * depth).to_string()
            };

            (part_a, part_b)
        },
    }
}
