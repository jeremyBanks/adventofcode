fn main() {
    let input = include_str!("../inputs/201501.txt");
    let mut floor: i64 = 0;
    let mut first_basement_index = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => {
                floor += 1;
            }
            ')' => {
                floor -= 1;
            }
            _ => unreachable!(),
        }
        if first_basement_index == 0 && floor <= -1 {
            first_basement_index = i + 1;
        }
    }

    dbg!(floor, first_basement_index);
}
