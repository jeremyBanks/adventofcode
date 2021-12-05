use crate::prelude::*;

pub fn solution() -> Solution {
    Solution {
        year: 2018,
        day: 4,
        code: |input| {
            #[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
            struct Event {
                year: u32,
                month: u32,
                day: u32,
                hour: u32,
                minute: u32,
                event_type: Type,
            }

            #[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
            enum Type {
                // shift change to specified new guard id
                ShiftChange(u32),
                FallsAsleep,
                WakesUp,
            }

            // [1518-11-01 00:30] falls asleep
            // [1518-11-01 00:55] wakes up
            // [1518-11-01 23:58] Guard #99 begins shift

            let mut events = Vec::new();
            for line in input.lines() {
                let rest = line.chars();
                let rest = rest.skip_while(is_not_digit);
                let year_str: String = rest.clone().take_while(is_digit).collect();
                let rest = rest.skip_while(is_digit).skip_while(is_not_digit);
                let month_str: String = rest.clone().take_while(is_digit).collect();
                let rest = rest.skip_while(is_digit).skip_while(is_not_digit);
                let day_str: String = rest.clone().take_while(is_digit).collect();
                let rest = rest.skip_while(is_digit).skip_while(is_not_digit);
                let hour_str: String = rest.clone().take_while(is_digit).collect();
                let rest = rest.skip_while(is_digit).skip_while(is_not_digit);
                let minute_str: String = rest.clone().take_while(is_digit).collect();
                let s: String = rest.clone().collect();

                let event_type = if s.contains("falls asleep") {
                    Type::FallsAsleep
                } else if s.contains("wakes up") {
                    Type::WakesUp
                } else {
                    let rest = rest.skip_while(is_digit).skip_while(is_not_digit);
                    let guard_id_str: String = rest.clone().take_while(is_digit).collect();
                    Type::ShiftChange(guard_id_str.parse().unwrap())
                };

                events.push(Event {
                    year: year_str.parse().unwrap(),
                    month: month_str.parse().unwrap(),
                    day: day_str.parse().unwrap(),
                    hour: hour_str.parse().unwrap(),
                    minute: minute_str.parse().unwrap(),
                    event_type,
                });
            }

            events.sort();

            let mut minutes_sleep_frequencies_by_guard_id = HashMap::<u32, Vec<u32>>::new();

            let mut guard_ids: HashSet<u32> = HashSet::new();
            for event in events.iter() {
                if let Type::ShiftChange(guard_id) = event.event_type {
                    if !guard_ids.contains(&guard_id) {
                        minutes_sleep_frequencies_by_guard_id.insert(guard_id, vec![0; 60]);
                        guard_ids.insert(guard_id);
                    }
                }
            }

            let mut current_guard: Option<u32> = None;
            let mut current_sleep_start_minute: Option<u32> = None;
            for event in events.iter() {
                match event.event_type {
                    Type::ShiftChange(guard_id) => {
                        if let Some(start_minute) = current_sleep_start_minute {
                            if let Some(old_guard_id) = current_guard {
                                let mut m = start_minute;
                                loop {
                                    if m == event.minute {
                                        break;
                                    }

                                    minutes_sleep_frequencies_by_guard_id
                                        .get_mut(&old_guard_id)
                                        .unwrap()[m as usize] += 1;

                                    m = (m + 1) % 60;
                                }
                            }
                        }
                        current_guard = Some(guard_id);
                        current_sleep_start_minute = None;
                    },
                    Type::WakesUp => {
                        if let Some(start_minute) = current_sleep_start_minute {
                            if let Some(old_guard_id) = current_guard {
                                let mut m = start_minute;
                                loop {
                                    if m == event.minute {
                                        break;
                                    }

                                    minutes_sleep_frequencies_by_guard_id
                                        .get_mut(&old_guard_id)
                                        .unwrap()[m as usize] += 1;

                                    m = (m + 1) % 60;
                                }
                            }
                        }
                        current_sleep_start_minute = None;
                    },
                    Type::FallsAsleep => {
                        current_sleep_start_minute = Some(event.minute);
                    },
                }
            }

            let mut max_sum = 0;
            let mut max_sum_guard_id = 0;
            let mut max_sum_minute = 0;
            let mut total_minutes_slept_by_guard_id: HashMap<u32, u32> = HashMap::new();
            for (guard_id, minutes) in minutes_sleep_frequencies_by_guard_id.iter() {
                let sum = minutes.iter().sum();
                if sum > max_sum {
                    max_sum_guard_id = *guard_id;
                    max_sum = sum;
                    let mut max_todo_name_this = 0;
                    max_sum_minute = 0;
                    for (i, i_minutes) in minutes.iter().enumerate() {
                        if *i_minutes > max_todo_name_this {
                            max_sum_minute = i;
                            max_todo_name_this = *i_minutes;
                        }
                    }
                }
                total_minutes_slept_by_guard_id.insert(*guard_id, sum);
            }

            let solution_4a = (max_sum_guard_id as usize) * max_sum_minute;

            let mut max_sum_guard_id = 0;
            let mut max_sum_minute = 0;
            let mut max_todo_name_this = 0;
            let mut total_minutes_slept_by_guard_id: HashMap<u32, u32> = HashMap::new();
            for (guard_id, minutes) in minutes_sleep_frequencies_by_guard_id.iter() {
                let sum = minutes.iter().sum();
                for (i, i_minutes) in minutes.iter().enumerate() {
                    if *i_minutes > max_todo_name_this {
                        max_sum_guard_id = *guard_id;
                        max_sum_minute = i;
                        max_todo_name_this = *i_minutes;
                    }
                }
                total_minutes_slept_by_guard_id.insert(*guard_id, sum);
            }

            let solution_4b = (max_sum_guard_id as usize) * max_sum_minute;

            (solution_4a.to_string(), solution_4b.to_string())
        },
    }
}
