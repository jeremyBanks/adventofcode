crate::prelude!();

pub fn solution() {
    Solution {
        year: 2019,
        day: 1,
        code: |input| {
            let mut total_fuel_one = 0;
            let mut total_fuel_two = 0;

            let masses = split_and_parse::<u32>(input, "\n");
            for mass in masses {
                let mut mass = mass;
                for i in 0.. {
                    let fuel = (mass / 3).checked_sub(2).unwrap_or_default();
                    if fuel > 0 {
                        if i == 0 {
                            total_fuel_one += fuel;
                        }
                        total_fuel_two += fuel;
                        mass = fuel;
                    } else {
                        break;
                    }
                }
            }

            (total_fuel_one.to_string(), total_fuel_two.to_string())
        },
    }
}
