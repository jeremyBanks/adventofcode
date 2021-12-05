use crate::prelude::*;

pub fn solution() -> Solution {
    Solution {
        year: 2018,
        day: 2,
        code: |input| {
            let mut twos = 0;
            let mut threes = 0;

            for line in input.lines() {
                let mut frequencies = HashMap::new();

                for letter in line.chars() {
                    frequencies
                        .entry(letter)
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                }

                if frequencies.values().any(|f| *f == 2) {
                    twos += 1;
                }

                if frequencies.values().any(|f| *f == 3) {
                    threes += 1;
                }
            }

            let warehouse_checksum = twos * threes;
            let solution_2a = warehouse_checksum;

            fn id_variations(id: &str) -> Vec<String> {
                let id_chars: Vec<char> = id.chars().collect();
                let mut result = Vec::new();
                for index in 0..id_chars.len() {
                    let variation_chars = id_chars.clone();
                    let variation: String = variation_chars
                        .iter()
                        .enumerate()
                        .map(|x| if x.0 != index { x.1 } else { &'_' })
                        .collect();
                    result.push(variation);
                }
                result
            }

            let mut seen_ids_by_variation = HashMap::new();
            let mut result: Option<String> = None;
            'out: for id in input.lines() {
                for variation in id_variations(id) {
                    if let Some(_adjacent_id) = seen_ids_by_variation.get(&variation) {
                        result = Some(variation.chars().filter(|c| *c != '_').collect());
                        break 'out;
                    } else {
                        seen_ids_by_variation.insert(variation, id);
                    }
                }
            }
            let solution_2b = result.unwrap();

            (solution_2a.to_string(), solution_2b)
        },
    }
}
