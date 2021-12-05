use crate::prelude::*;

pub fn solution() -> Solution {
    Solution {
        year: 2018,
        day: 6,
        code: |input| {
            trait Manhattan {
                fn manhattan(self, other: Self) -> u32;
            }
            impl Manhattan for (u32, u32) {
                fn manhattan(self, other: Self) -> u32 {
                    (if self.0 > other.0 {
                        self.0 - other.0
                    } else {
                        other.0 - self.0
                    }) + (if self.1 > other.1 {
                        self.1 - other.1
                    } else {
                        other.1 - self.1
                    })
                }
            }

            let pattern = regex::Regex::new(r"^(\d+),\s(\d+)$").unwrap();

            let mut dangers: Vec<(u32, u32)> = Vec::new();
            for line in input.lines() {
                let pieces = pattern.captures(line).unwrap();
                dangers.push((pieces[1].parse().unwrap(), pieces[2].parse().unwrap()))
            }

            #[derive(Eq, PartialEq, Debug, Clone)]
            struct Filler {
                distance: u32,
                origin: (u32, u32),
            }
            impl Ord for Filler {
                fn cmp(&self, other: &Self) -> Ordering {
                    self.distance.cmp(&other.distance).reverse()
                }
            }
            impl PartialOrd for Filler {
                fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                    Some(self.cmp(other))
                }
            }

            let mut width = 0;
            let mut height = 0;

            let mut filling = BinaryHeap::new();
            for danger in dangers {
                if danger.0 > width {
                    width = danger.0;
                }
                if danger.1 > height {
                    height = danger.1;
                }
                filling.push(Filler {
                    distance: 0,
                    origin: danger,
                });
            }

            #[derive(Debug, Clone, Copy)]
            enum Space {
                Unvisited,
                Tied { distance: u32 },
                ClosestTo { danger: (u32, u32) },
            }
            let mut spaces = vec![Space::Unvisited; (width * height) as usize];

            let mut area_by_origin = HashMap::<(u32, u32), u32>::new();
            while !filling.is_empty() {
                let mut filler = filling.pop().unwrap();
                let mut alive = false;
                let (x0, y0) = filler.origin;
                let d = i32::try_from(filler.distance).unwrap();
                for dy in -d..=d {
                    for dx in -d..=d {
                        if !(dy == -d || dy == d || dx == -d || dx == d) {
                            continue;
                        }
                        let sx = i32::try_from(x0).unwrap() + dx;
                        let sy = i32::try_from(y0).unwrap() + dy;
                        if sx < 0 || sy < 0 {
                            continue;
                        }
                        let x = u32::try_from(sx).unwrap();
                        let y = u32::try_from(sy).unwrap();
                        if x >= width || y >= height {
                            continue;
                        }
                        let new_distance = filler.origin.manhattan((x, y));
                        match spaces[(x + y * width) as usize] {
                            Space::Unvisited => {
                                spaces[(x + y * width) as usize] = Space::ClosestTo {
                                    danger: filler.origin,
                                };
                                alive = true;
                                area_by_origin
                                    .entry(filler.origin)
                                    .and_modify(|n| *n += 1)
                                    .or_insert(1);
                            },
                            Space::ClosestTo { danger } => {
                                if danger == filler.origin {
                                    // oops
                                    continue;
                                }
                                let old_distance = danger.manhattan((x, y));
                                if old_distance == new_distance {
                                    spaces[(x + y * width) as usize] = Space::Tied {
                                        distance: old_distance,
                                    };
                                    alive = true;
                                    area_by_origin.insert(danger, area_by_origin[&danger] - 1);
                                } else if old_distance > new_distance {
                                    area_by_origin.insert(danger, area_by_origin[&danger] - 1);
                                    area_by_origin
                                        .entry(filler.origin)
                                        .and_modify(|n| *n += 1)
                                        .or_insert(1);
                                    spaces[(x + y * width) as usize] = Space::ClosestTo {
                                        danger: filler.origin,
                                    };
                                    alive = true;
                                }
                            },
                            Space::Tied { distance } =>
                                if distance > new_distance {
                                    spaces[(x + y * width) as usize] = Space::ClosestTo {
                                        danger: filler.origin,
                                    };
                                    area_by_origin
                                        .entry(filler.origin)
                                        .and_modify(|n| *n += 1)
                                        .or_insert(1);
                                    alive = true;
                                },
                        }
                    }
                }

                if alive {
                    filler.distance += 1;
                    filling.push(filler);
                }
            }

            // if they extend to the edge they extend infinitely.
            let mut infinite_origins = HashSet::new();
            for x in 0..width {
                if let Space::ClosestTo { danger } = spaces[(x) as usize] {
                    infinite_origins.insert(danger);
                }
                if let Space::ClosestTo { danger } = spaces[(x + (height - 2) * width) as usize] {
                    infinite_origins.insert(danger);
                }
            }
            for y in 0..height {
                if let Space::ClosestTo { danger } = spaces[(y * width) as usize] {
                    infinite_origins.insert(danger);
                }
                if let Space::ClosestTo { danger } = spaces[((width - 1) + y * width) as usize] {
                    infinite_origins.insert(danger);
                }
            }

            let largest_area = *area_by_origin
                .iter()
                .filter(|(k, _v)| !infinite_origins.contains(k))
                .map(|(_k, v)| v)
                .max()
                .unwrap();

            (largest_area.to_string(), "TODO".to_string())
        },
    }
}
