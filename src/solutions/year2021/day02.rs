crate::prelude!();

pub fn solution() -> Solution {
    Solution {
        year: 2021,
        day: 2,
        code: |input| {
            let commands = input.split("\n").collect::<Vec<_>>();

            let mut position: i32 = 0;
            let mut depth: i32 = 0;
            for command in commands {
                let parts = command.split(" ").collect::<Vec<_>>();
                let direction = parts[0];
                let magnitude = parts[1].parse::<u32>().unwrap();

                match direction {
                    "up" => depth -= magnitude as i32,
                    "down" => depth += magnitude as i32,
                    "forward" => position += magnitude as i32,
                    "backward" => position -= magnitude as i32,
                    _ => panic!("Unknown direction: {}", direction),
                }
            }

            let part_a = (position.abs() * depth.abs()).to_string();
            let part_b = "todo".to_string();

            (part_a, part_b)
        },
    }
}
