crate::prelude!();

pub fn solution() {
    Solution {
        year: 2015,
        day: 2,
        code: |input| {
            let lines = Vec::from_iter(input.split('\n'));
            let mut paper_area = 0;
            let mut ribbon_length = 0;

            for line in lines {
                let mut dimensions =
                    Vec::from_iter(line.split('x').map(|n| n.parse::<u64>().unwrap()));

                dimensions.sort();
                let a = dimensions[0];
                let b = dimensions[1];
                let c = dimensions[2];

                paper_area += (a * b * 3) + (b * c * 2) + (a * c * 2);
                ribbon_length += (a + b) * 2 + (a * b * c);
            }

            (paper_area.to_string(), ribbon_length.to_string())
        },
    }
}
