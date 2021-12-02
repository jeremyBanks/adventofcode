crate::prelude!();

pub fn solution() {
    Solution {
        year: 2021,
        day: 1,
        code: |input| {
            let depths: Vec<u32> = input.split('\n').map(|s| s.parse().unwrap()).collect();

            let mut deeper_depths = 0;
            for i in 1..depths.len() {
                let previous = depths[i - 1];
                let current = depths[i];
                if current > previous {
                    deeper_depths += 1;
                }
            }

            let mut sums = Vec::<u32>::new();
            for i in 2..depths.len() {
                let first = depths[i - 2];
                let second = depths[i - 1];
                let third = depths[i];
                let sum = first + second + third;
                sums.push(sum);
            }

            let mut deeper_sums = 0;
            for i in 1..sums.len() {
                let previous = sums[i - 1];
                let current = sums[i];
                if current > previous {
                    deeper_sums += 1;
                }
            }

            (deeper_depths, deeper_sums)
        },
    }
}
