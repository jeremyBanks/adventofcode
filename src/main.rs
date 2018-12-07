#![feature(
    duration_as_u128,
    range_contains,
    associated_type_defaults,
    never_type,
    try_from,
    const_fn
)]
#![allow(unused_imports)]

#[macro_use]
extern crate derive_more;

use std::{
    cmp::{Ord, Ordering},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    convert::TryFrom,
    env,
    fmt::{Debug, Display},
    fs,
    time::{Duration, Instant},
};

use itertools::Itertools;
use regex::Regex;
use typed_arena::Arena;

#[allow(non_camel_case_types)]
type int = isize;
#[allow(non_camel_case_types)]
type uint = usize;
#[allow(dead_code)]
const INT_MAX: int = std::isize::MAX;
#[allow(dead_code)]
const INT_MIN: int = std::isize::MIN;
#[allow(dead_code)]
const UINT_MAX: uint = std::usize::MAX;
#[allow(dead_code)]
const UINT_MIN: uint = std::usize::MIN;

fn main() {
    println!("🎄 Advent of Code 2018");
    println!();

    let argv: Vec<String> = env::args().collect();
    let n: Option<uint> = match argv.len() {
        1 => None,
        2 => Some(argv[1].parse().unwrap()),
        _ => {
            println!("usage: {} [PROBLEM_NUMBER]", argv[0]);
            return;
        }
    };

    let solutions: Vec<Box<dyn Fn() -> ()>> = vec![
        Box::new(|| ChronalCalibration.run()),
        Box::new(|| InventoryManagementSystem.run()),
        Box::new(|| NoMatterHowYouSliceIt.run()),
        Box::new(|| ReposeRecord.run()),
        Box::new(|| AlchemicalReduction.run()),
        Box::new(|| ChronalCoordinates.run()),
        Box::new(|| TheSumOfItsParts.run()),
    ];

    if let Some(n) = n {
        solutions[n - 1]()
    } else {
        for f in solutions {
            f()
        }
    }
}

trait Problem<SolutionA: Debug + PartialEq = (), SolutionB: Debug + PartialEq = ()> {
    fn day(&self) -> uint;
    fn name(&self) -> &'static str;
    fn known_solution(&self) -> (Option<SolutionA>, Option<SolutionB>) {
        (None, None)
    }
    fn solve(&self, input: &[&str]) -> (SolutionA, SolutionB);

    fn run(&self) {
        let day = self.day();
        println!("✨ {}", self.name());
        println!();
        let input_full =
            fs::read_to_string(format!("input/{}.txt", day)).expect("Failed to load input.");
        let input_lines: Vec<_> = input_full.split('\n').filter(|s| !s.is_empty()).collect();

        let (expected_a, expected_b) = self.known_solution();

        let (a, b) = self.solve(&input_lines);

        fn report<T: Debug + PartialEq>(day: uint, part: char, expected: &Option<T>, actual: &T) {
            if let Some(expected) = expected {
                if *actual == *expected {
                    println!("{}{}. {:?}", day, part, actual);
                } else {
                    println!(
                        "{}{}. {:<8?}❌ (expected {:?})",
                        day, part, actual, expected
                    );
                }
            } else {
                println!("{}{}. {:<8?}❓", day, part, actual);
            }
        }

        report(day, 'a', &expected_a, &a);
        report(day, 'b', &expected_b, &b);

        // Don't benchmark unless it produces the right answer.
        if expected_a == Some(a) && expected_b == Some(b) {
            let before = Instant::now();
            let mut iterations: u128 = 0;
            let mut total_duration;
            loop {
                self.solve(&input_lines);

                iterations += 1;
                total_duration = Instant::now() - before;

                if total_duration > Duration::from_secs(2) || iterations >= 128 {
                    break;
                }
            }
            let duration_micros = (total_duration.as_nanos() / iterations) / 1000;

            println!("{}µ. {}", day, duration_micros);
        }

        println!();
    }
}

/*
--- Day 1: Chronal Calibration ---
"We've detected some temporal anomalies," one of Santa's Elves at the Temporal Anomaly Research and Detection Instrument Station tells you. She sounded pretty worried when she called you down here. "At 500-year intervals into the past, someone has been changing Santa's history!"

"The good news is that the changes won't propagate to our time stream for another 25 days, and we have a device" - she attaches something to your wrist - "that will let you fix the changes with no such propagation delay. It's configured to send you 500 years further into the past every few days; that was the best we could do on such short notice."

"The bad news is that we are detecting roughly fifty anomalies throughout time; the device will indicate fixed anomalies with stars. The other bad news is that we only have one device and you're the best person for the job! Good lu--" She taps a button on the device and you suddenly feel like you're falling. To save Christmas, you need to get all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

After feeling like you've been falling for a few minutes, you look at the device's tiny screen. "Error: Device must be calibrated before first use. Frequency drift detected. Cannot maintain destination lock." Below the message, the device shows a sequence of changes in frequency (your puzzle input). A value like +6 means the current frequency increases by 6; a value like -3 means the current frequency decreases by 3.

For example, if the device displays frequency changes of +1, -2, +3, +1, then starting from a frequency of zero, the following changes would occur:

Current frequency  0, change of +1; resulting frequency  1.
Current frequency  1, change of -2; resulting frequency -1.
Current frequency -1, change of +3; resulting frequency  2.
Current frequency  2, change of +1; resulting frequency  3.
In this example, the resulting frequency is 3.

Here are other example situations:

+1, +1, +1 results in  3
+1, +1, -2 results in  0
-1, -2, -3 results in -6
Starting with a frequency of zero, what is the resulting frequency after all of the changes in frequency have been applied?

--- Part Two ---
You notice that the device repeats the same frequency change list over and over. To calibrate the device, you need to find the first frequency it reaches twice.

For example, using the same list of changes above, the device would loop as follows:

Current frequency  0, change of +1; resulting frequency  1.
Current frequency  1, change of -2; resulting frequency -1.
Current frequency -1, change of +3; resulting frequency  2.
Current frequency  2, change of +1; resulting frequency  3.
(At this point, the device continues from the start of the list.)
Current frequency  3, change of +1; resulting frequency  4.
Current frequency  4, change of -2; resulting frequency  2, which has already been seen.
In this example, the first frequency reached twice is 2. Note that your device might need to repeat its list of frequency changes many times before a duplicate frequency is found, and that duplicates might be found while in the middle of processing the list.

Here are other examples:

+1, -1 first reaches 0 twice.
+3, +3, +4, -2, -4 first reaches 10 twice.
-6, +3, +8, +5, -6 first reaches 5 twice.
+7, +7, -2, -7, -4 first reaches 14 twice.
What is the first frequency your device reaches twice?
*/
struct ChronalCalibration;
impl Problem<int, int> for ChronalCalibration {
    fn day(&self) -> uint {
        1
    }
    fn name(&self) -> &'static str {
        "Chronal Calibration"
    }
    fn known_solution(&self) -> (Option<int>, Option<int>) {
        (Some(580), Some(81972))
    }
    fn solve(&self, input: &[&str]) -> (int, int) {
        let numbers: Vec<int> = input
            .iter()
            .map(|s| s.parse().expect("non-integer in input"))
            .collect();
        let frequency_sum = numbers.iter().cloned().sum::<int>();

        let mut seen = HashSet::new();
        let mut sum: int = 0;
        let mut first_repeated_sum = None;
        for number in numbers.iter().cycle() {
            sum += number;
            if seen.contains(&sum) {
                first_repeated_sum = Some(sum);
                break;
            } else {
                seen.insert(sum);
            }
        }
        let first_repeated_sum = first_repeated_sum.unwrap();

        (frequency_sum, first_repeated_sum)
    }
}

/*
--- Day 2: Inventory Management System ---
You stop falling through time, catch your breath, and check the screen on the device. "Destination reached. Current Year: 1518. Current Location: North Pole Utility Closet 83N10." You made it! Now, to find those anomalies.

Outside the utility closet, you hear footsteps and a voice. "...I'm not sure either. But now that so many people have chimneys, maybe he could sneak in that way?" Another voice responds, "Actually, we've been working on a new kind of suit that would let him fit through tight spaces like that. But, I heard that a few days ago, they lost the prototype fabric, the design plans, everything! Nobody on the team can even seem to remember important details of the project!"

"Wouldn't they have had enough fabric to fill several boxes in the warehouse? They'd be stored together, so the box IDs should be similar. Too bad it would take forever to search the warehouse for two similar box IDs..." They walk too far away to hear any more.

Late at night, you sneak to the warehouse - who knows what kinds of paradoxes you could cause if you were discovered - and use your fancy wrist device to quickly scan every box and produce a list of the likely candidates (your puzzle input).

To make sure you didn't miss any, you scan the likely candidate boxes again, counting the number that have an ID containing exactly two of any letter and then separately counting those with exactly three of any letter. You can multiply those two counts together to get a rudimentary checksum and compare it to what your device predicts.

For example, if you see the following box IDs:

abcdef contains no letters that appear exactly two or three times.
bababc contains two a and three b, so it counts for both.
abbcde contains two b, but no letter appears exactly three times.
abcccd contains three c, but no letter appears exactly two times.
aabcdd contains two a and two d, but it only counts once.
abcdee contains two e.
ababab contains three a and three b, but it only counts once.
Of these box IDs, four of them contain a letter which appears exactly twice, and three of them contain a letter which appears exactly three times. Multiplying these together produces a checksum of 4 * 3 = 12.

What is the checksum for your list of box IDs?

--- Part Two ---
Confident that your list of box IDs is complete, you're ready to find the boxes full of prototype fabric.

The boxes will have IDs which differ by exactly one character at the same position in both strings. For example, given the following box IDs:

abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz
The IDs abcde and axcye are close, but they differ by two characters (the second and fourth). However, the IDs fghij and fguij differ by exactly one character, the third (h and u). Those must be the correct boxes.

What letters are common between the two correct box IDs? (In the example above, this is found by removing the differing character from either ID, producing fgij.)
*/
struct InventoryManagementSystem;
impl Problem<uint, String> for InventoryManagementSystem {
    fn day(&self) -> uint {
        2
    }
    fn name(&self) -> &'static str {
        "Inventory Management System"
    }
    fn known_solution(&self) -> (Option<uint>, Option<String>) {
        (Some(9139), Some("uqcidadzwtnhsljvxyobmkfyr".to_string()))
    }
    fn solve(&self, input: &[&str]) -> (uint, String) {
        let mut twos = 0;
        let mut threes = 0;

        for line in input.iter() {
            let mut frequencies = HashMap::new();
            use std::collections::hash_map::Entry::{Occupied, Vacant};

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
        'out: for id in input.iter() {
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

        (solution_2a, solution_2b)
    }
}

fn is_digit(c: &char) -> bool {
    ('0'..='9').contains(c)
}

fn is_not_digit(c: &char) -> bool {
    !('0'..='9').contains(c)
}

/*
--- Day 3: No Matter How You Slice It ---
The Elves managed to locate the chimney-squeeze prototype fabric for Santa's suit (thanks to someone who helpfully wrote its box IDs on the wall of the warehouse in the middle of the night). Unfortunately, anomalies are still affecting them - nobody can even agree on how to cut the fabric.

The whole piece of fabric they're working on is a very large square - at least 1000 inches on each side.

Each Elf has made a claim about which area of fabric would be ideal for Santa's suit. All claims have an ID and consist of a single rectangle with edges parallel to the edges of the fabric. Each claim's rectangle is defined as follows:

The number of inches between the left edge of the fabric and the left edge of the rectangle.
The number of inches between the top edge of the fabric and the top edge of the rectangle.
The width of the rectangle in inches.
The height of the rectangle in inches.
A claim like #123 @ 3,2: 5x4 means that claim ID 123 specifies a rectangle 3 inches from the left edge, 2 inches from the top edge, 5 inches wide, and 4 inches tall. Visually, it claims the square inches of fabric represented by # (and ignores the square inches of fabric represented by .) in the diagram below:

...........
...........
...#####...
...#####...
...#####...
...#####...
...........
...........
...........
The problem is that many of the claims overlap, causing two or more claims to cover part of the same areas. For example, consider the following claims:

#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2
Visually, these claim the following areas:

........
...2222.
...2222.
.11XX22.
.11XX22.
.111133.
.111133.
........
The four square inches marked with X are claimed by both 1 and 2. (Claim 3, while adjacent to the others, does not overlap either of them.)

If the Elves all proceed with their own plans, none of them will have enough fabric. How many square inches of fabric are within two or more claims?

--- Part Two ---
Amidst the chaos, you notice that exactly one claim doesn't overlap by even a single square inch of fabric with any other claim. If you can somehow draw attention to it, maybe the Elves will be able to make Santa's suit after all!

For example, in the claims above, only claim 3 is intact after all claims are made.

What is the ID of the only claim that doesn't overlap?
*/
struct NoMatterHowYouSliceIt;
impl Problem<uint, uint> for NoMatterHowYouSliceIt {
    fn day(&self) -> uint {
        3
    }
    fn name(&self) -> &'static str {
        "No Matter How You Slice It"
    }
    fn known_solution(&self) -> (Option<uint>, Option<uint>) {
        (Some(112_378), Some(603))
    }
    fn solve(&self, input: &[&str]) -> (uint, uint) {
        #[derive(Clone, Debug)]
        struct Claim {
            id: uint,
            x: uint,
            y: uint,
            width: uint,
            height: uint,
        }

        let mut claims = Vec::new();

        // example:
        // #11 @ 755,237: 24x22
        let pattern = Regex::new(
            r"(?x)
        ^\#
        (?P<id>\d+)
        \s@\s
        (?P<x>\d+)
        ,
        (?P<y>\d+)
        :\s
        (?P<width>\d+)
        x
        (?P<height>\d+)
        $",
        )
        .unwrap();

        for line in input {
            let pieces = pattern.captures(line).unwrap();
            claims.push(Claim {
                id: pieces["id"].parse().unwrap(),
                x: pieces["x"].parse().unwrap(),
                y: pieces["y"].parse().unwrap(),
                width: pieces["width"].parse().unwrap(),
                height: pieces["height"].parse().unwrap(),
            });
        }

        let mut cell_counts = vec![0u32; 1_000_000];
        let mut overcommitted_square_inches = 0;
        for claim in claims.iter() {
            for x in claim.x..(claim.x + claim.width) {
                for y in claim.y..(claim.y + claim.height) {
                    let index = y * 1000 + x;
                    cell_counts[index] += 1;
                    // count the cell only the first time it becomes overcommitted
                    if cell_counts[index] == 2 {
                        overcommitted_square_inches += 1;
                    }
                }
            }
        }
        let solution_3a = overcommitted_square_inches;

        let mut solution_3b = None;

        for claim in claims.iter() {
            let mut all_good = true;
            'out: for x in claim.x..(claim.x + claim.width) {
                for y in claim.y..(claim.y + claim.height) {
                    let index = y * 1000 + x;
                    if cell_counts[index] > 1 {
                        all_good = false;
                        break 'out;
                    }
                }
            }
            if all_good {
                solution_3b = Some(claim.id);
            }
        }
        let solution_3b = solution_3b.unwrap();

        (solution_3a, solution_3b)
    }
}

/*
--- Day 4: Repose Record ---
You've sneaked into another supply closet - this time, it's across from the prototype suit manufacturing lab. You need to sneak inside and fix the issues with the suit, but there's a guard stationed outside the lab, so this is as close as you can safely get.

As you search the closet for anything that might help, you discover that you're not the first person to want to sneak in. Covering the walls, someone has spent an hour starting every midnight for the past few months secretly observing this guard post! They've been writing down the ID of the one guard on duty that night - the Elves seem to have decided that one guard was enough for the overnight shift - as well as when they fall asleep or wake up while at their post (your puzzle input).

For example, consider the following records, which have already been organized into chronological order:

[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up
Timestamps are written using year-month-day hour:minute format. The guard falling asleep or waking up is always the one whose shift most recently started. Because all asleep/awake times are during the midnight hour (00:00 - 00:59), only the minute portion (00 - 59) is relevant for those events.

Visually, these records show that the guards are asleep at these times:

Date   ID   Minute
            000000000011111111112222222222333333333344444444445555555555
            012345678901234567890123456789012345678901234567890123456789
11-01  #10  .....####################.....#########################.....
11-02  #99  ........................................##########..........
11-03  #10  ........................#####...............................
11-04  #99  ....................................##########..............
11-05  #99  .............................................##########.....
The columns are Date, which shows the month-day portion of the relevant day; ID, which shows the guard on duty that day; and Minute, which shows the minutes during which the guard was asleep within the midnight hour. (The Minute column's header shows the minute's ten's digit in the first row and the one's digit in the second row.) Awake is shown as ., and asleep is shown as #.

Note that guards count as asleep on the minute they fall asleep, and they count as awake on the minute they wake up. For example, because Guard #10 wakes up at 00:25 on 1518-11-01, minute 25 is marked as awake.

If you can figure out the guard most likely to be asleep at a specific time, you might be able to trick that guard into working tonight so you can have the best chance of sneaking in. You have two strategies for choosing the best guard/minute combination.

Strategy 1: Find the guard that has the most minutes asleep. What minute does that guard spend asleep the most?

In the example above, Guard #10 spent the most minutes asleep, a total of 50 minutes (20+25+5), while Guard #99 only slept for a total of 30 minutes (10+10+10). Guard #10 was asleep most during minute 24 (on two days, whereas any other minute the guard was asleep was only seen on one day).

While this example listed the entries in chronological order, your entries are in the order you found them. You'll need to organize them before they can be analyzed.

What is the ID of the guard you chose multiplied by the minute you chose? (In the above example, the answer would be 10 * 24 = 240.)

--- Part Two ---
Strategy 2: Of all guards, which guard is most frequently asleep on the same minute?

In the example above, Guard #99 spent minute 45 asleep more than any other guard or minute - three times in total. (In all other cases, any guard spent any minute asleep at most twice.)

What is the ID of the guard you chose multiplied by the minute you chose? (In the above example, the answer would be 99 * 45 = 4455.)
*/
struct ReposeRecord;
impl Problem<uint, uint> for ReposeRecord {
    fn day(&self) -> uint {
        4
    }
    fn name(&self) -> &'static str {
        "Repose Record"
    }
    fn known_solution(&self) -> (Option<uint>, Option<uint>) {
        (Some(94542), Some(50966))
    }
    fn solve(&self, input: &[&str]) -> (uint, uint) {
        #[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
        struct Event {
            year: uint,
            month: uint,
            day: uint,
            hour: uint,
            minute: uint,
            event_type: Type,
        }

        #[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
        enum Type {
            // shift change to specified new guard id
            ShiftChange(uint),
            FallsAsleep,
            WakesUp,
        }

        // [1518-11-01 00:30] falls asleep
        // [1518-11-01 00:55] wakes up
        // [1518-11-01 23:58] Guard #99 begins shift

        let mut events = Vec::new();
        for line in input {
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

        let mut minutes_sleep_frequencies_by_guard_id = HashMap::<uint, Vec<uint>>::new();

        let mut guard_ids: HashSet<uint> = HashSet::new();
        for event in events.iter() {
            if let Type::ShiftChange(guard_id) = event.event_type {
                if !guard_ids.contains(&guard_id) {
                    minutes_sleep_frequencies_by_guard_id.insert(guard_id, vec![0; 60]);
                    guard_ids.insert(guard_id);
                }
            }
        }

        let mut current_guard: Option<uint> = None;
        let mut current_sleep_start_minute: Option<uint> = None;
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
                                    .unwrap()[m] += 1;

                                m = (m + 1) % 60;
                            }
                        }
                    }
                    current_guard = Some(guard_id);
                    current_sleep_start_minute = None;
                }
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
                                    .unwrap()[m] += 1;

                                m = (m + 1) % 60;
                            }
                        }
                    }
                    current_sleep_start_minute = None;
                }
                Type::FallsAsleep => {
                    current_sleep_start_minute = Some(event.minute);
                }
            }
        }

        let mut max_sum = 0;
        let mut max_sum_guard_id = 0;
        let mut max_sum_minute = 0;
        let mut total_minutes_slept_by_guard_id: HashMap<uint, uint> = HashMap::new();
        for (guard_id, minutes) in minutes_sleep_frequencies_by_guard_id.iter() {
            let sum = minutes.iter().sum();
            if sum > max_sum {
                max_sum_guard_id = *guard_id;
                max_sum = sum;
                let mut max_egsegesgesges = 0;
                max_sum_minute = 0;
                for (i, i_minutes) in minutes.iter().enumerate() {
                    if *i_minutes > max_egsegesgesges {
                        max_sum_minute = i;
                        max_egsegesgesges = *i_minutes;
                    }
                }
            }
            total_minutes_slept_by_guard_id.insert(*guard_id, sum);
        }

        let solution_4a = max_sum_guard_id * max_sum_minute;

        let mut max_sum_guard_id = 0;
        let mut max_sum_minute = 0;
        let mut max_egsegesgesges = 0;
        let mut total_minutes_slept_by_guard_id: HashMap<uint, uint> = HashMap::new();
        for (guard_id, minutes) in minutes_sleep_frequencies_by_guard_id.iter() {
            let sum = minutes.iter().sum();
            for (i, i_minutes) in minutes.iter().enumerate() {
                if *i_minutes > max_egsegesgesges {
                    max_sum_guard_id = *guard_id;
                    max_sum_minute = i;
                    max_egsegesgesges = *i_minutes;
                }
            }
            total_minutes_slept_by_guard_id.insert(*guard_id, sum);
        }

        let solution_4b = max_sum_guard_id * max_sum_minute;

        (solution_4a, solution_4b)
    }
}

/*
--- Day 5: Alchemical Reduction ---
You've managed to sneak in to the prototype suit manufacturing lab. The Elves are making decent progress, but are still struggling with the suit's size reduction capabilities.

While the very latest in 1518 alchemical technology might have solved their problem eventually, you can do better. You scan the chemical composition of the suit's material and discover that it is formed by extremely long polymers (one of which is available as your puzzle input).

The polymer is formed by smaller units which, when triggered, react with each other such that two adjacent units of the same type and opposite polarity are destroyed. Units' types are represented by letters; units' polarity is represented by capitalization. For instance, r and R are units with the same type but opposite polarity, whereas r and s are entirely different types and do not react.

For example:

In aA, a and A react, leaving nothing behind.
In abBA, bB destroys itself, leaving aA. As above, this then destroys itself, leaving nothing.
In abAB, no two adjacent units are of the same type, and so nothing happens.
In aabAAB, even though aa and AA are of the same type, their polarities match, and so nothing happens.
Now, consider a larger example, dabAcCaCBAcCcaDA:

dabAcCaCBAcCcaDA  The first 'cC' is removed.
dabAaCBAcCcaDA    This creates 'Aa', which is removed.
dabCBAcCcaDA      Either 'cC' or 'Cc' are removed (the result is the same).
dabCBAcaDA        No further actions can be taken.
After all possible reactions, the resulting polymer contains 10 units.

How many units remain after fully reacting the polymer you scanned? (Note: in this puzzle and others, the input is large; if you copy/paste your input, make sure you get the whole thing.)

--- Part Two ---
Time to improve the polymer.

One of the unit types is causing problems; it's preventing the polymer from collapsing as much as it should. Your goal is to figure out which unit type is causing the most problems, remove all instances of it (regardless of polarity), fully react the remaining polymer, and measure its length.

For example, again using the polymer dabAcCaCBAcCcaDA from above:

Removing all A/a units produces dbcCCBcCcD. Fully reacting this polymer produces dbCBcD, which has length 6.
Removing all B/b units produces daAcCaCAcCcaDA. Fully reacting this polymer produces daCAcaDA, which has length 8.
Removing all C/c units produces dabAaBAaDA. Fully reacting this polymer produces daDA, which has length 4.
Removing all D/d units produces abAcCaCBAcCcaA. Fully reacting this polymer produces abCBAc, which has length 6.
In this example, removing all C/c units was best, producing the answer 4.

What is the length of the shortest polymer you can produce by removing all units of exactly one type and fully reacting the result?
*/
struct AlchemicalReduction;
impl Problem<uint, uint> for AlchemicalReduction {
    fn day(&self) -> uint {
        5
    }
    fn name(&self) -> &'static str {
        "Alchemical Reduction"
    }
    fn known_solution(&self) -> (Option<uint>, Option<uint>) {
        (Some(11298), Some(5148))
    }
    fn solve(&self, input: &[&str]) -> (uint, uint) {
        let input = input[0];

        fn flip_case(unit: char) -> char {
            match unit {
                'a'..='z' => unit.to_ascii_uppercase(),
                'A'..='Z' => unit.to_ascii_lowercase(),
                _ => panic!("unexpected character in input"),
            }
        }

        fn post_reaction_size(units: impl Iterator<Item = char>) -> uint {
            let mut reactor = VecDeque::new();
            let mut hopper = units.collect::<VecDeque<_>>();

            assert!(hopper.len() >= 2);

            while !hopper.is_empty() {
                let left = reactor.pop_back().or_else(|| hopper.pop_front()).unwrap();
                let right = hopper.pop_front().unwrap();

                if left == flip_case(right) {
                    // they annihilate each other and we drop them
                } else {
                    // they survive in the reaction chamber
                    reactor.push_back(left);
                    reactor.push_back(right);
                }
            }

            reactor.len()
        }

        let solution_5a = post_reaction_size(input.chars());

        let solution_5b = {
            let unit_types: HashSet<_> = input.chars().map(|c| c.to_ascii_lowercase()).collect();
            let mut best_unit_size = UINT_MAX;
            for unit_lower in unit_types.iter() {
                let unit_upper = unit_lower.to_ascii_uppercase();
                let filtered_input = input
                    .chars()
                    .filter(|c| *c != unit_upper && *c != *unit_lower);
                let size = post_reaction_size(filtered_input);
                if size < best_unit_size {
                    best_unit_size = size;
                }
            }

            best_unit_size
        };

        (solution_5a, solution_5b)
    }
}

/*
--- Day 6: Chronal Coordinates ---
The device on your wrist beeps several times, and once again you feel like you're falling.

"Situation critical," the device announces. "Destination indeterminate. Chronal interference detected. Please specify new target coordinates."

The device then produces a list of coordinates (your puzzle input). Are they places it thinks are safe or dangerous? It recommends you check manual page 729. The Elves did not give you a manual.

If they're dangerous, maybe you can minimize the danger by finding the coordinate that gives the largest distance from the other points.

Using only the Manhattan distance, determine the area around each coordinate by counting the number of integer X,Y locations that are closest to that coordinate (and aren't tied in distance to any other coordinate).

Your goal is to find the size of the largest area that isn't infinite. For example, consider the following list of coordinates:

1, 1
1, 6
8, 3
3, 4
5, 5
8, 9
If we name these coordinates A through F, we can draw them on a grid, putting 0,0 at the top left:

..........
.A........
..........
........C.
...D......
.....E....
.B........
..........
..........
........F.
This view is partial - the actual grid extends infinitely in all directions. Using the Manhattan distance, each location's closest coordinate can be determined, shown here in lowercase:

aaaaa.cccc
aAaaa.cccc
aaaddecccc
aadddeccCc
..dDdeeccc
bb.deEeecc
bBb.eeee..
bbb.eeefff
bbb.eeffff
bbb.ffffFf
Locations shown as . are equally far from two or more coordinates, and so they don't count as being closest to any.

In this example, the areas of coordinates A, B, C, and F are infinite - while not shown here, their areas extend forever outside the visible grid. However, the areas of coordinates D and E are finite: D is closest to 9 locations, and E is closest to 17 (both including the coordinate's location itself). Therefore, in this example, the size of the largest area is 17.

What is the size of the largest area that isn't infinite?
*/
struct ChronalCoordinates;
impl Problem<uint, ()> for ChronalCoordinates {
    fn day(&self) -> uint {
        6
    }
    fn name(&self) -> &'static str {
        "Chronal Coordinates"
    }
    fn known_solution(&self) -> (Option<uint>, Option<()>) {
        (None, None)
    }
    fn solve(&self, input: &[&str]) -> (uint, ()) {
        trait Manhattan {
            fn manhattan(self, other: Self) -> uint;
        }
        impl Manhattan for (uint, uint) {
            fn manhattan(self, other: Self) -> uint {
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

        let pattern = Regex::new(r"^(\d+),\s(\d+)$").unwrap();

        let mut dangers: Vec<(uint, uint)> = Vec::new();
        for line in input {
            let pieces = pattern.captures(line).unwrap();
            dangers.push((pieces[1].parse().unwrap(), pieces[2].parse().unwrap()))
        }

        #[derive(Eq, PartialEq, Debug, Clone)]
        struct Filler {
            distance: uint,
            origin: (uint, uint),
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

        const SIZE: uint = 384;

        #[derive(Debug, Clone, Copy)]
        enum Space {
            Unvisited,
            Tied { distance: uint },
            ClosestTo { danger: (uint, uint) },
        }
        let mut spaces = [Space::Unvisited; SIZE * SIZE];

        let mut filling = BinaryHeap::new();

        for danger in dangers {
            filling.push(Filler {
                distance: 0,
                origin: danger,
            });
        }

        let mut empty: uint = SIZE * SIZE;
        while !filling.is_empty() {
            let mut filler = filling.pop().unwrap();
            let mut alive = false;
            let (x0, y0) = filler.origin;
            let d = int::try_from(filler.distance).unwrap();
            for dy in -d..=d {
                for dx in -d..=d {
                    let sx = int::try_from(x0).unwrap() + dx;
                    let sy = int::try_from(y0).unwrap() + dy;
                    if sx < 0 || sy < 0 {
                        continue;
                    }
                    let x = uint::try_from(sx).unwrap();
                    let y = uint::try_from(sy).unwrap();
                    if x >= SIZE || y >= SIZE {
                        continue;
                    }
                    match spaces[x + y * SIZE] {
                        Space::Unvisited => {
                            spaces[x + y * SIZE] = Space::ClosestTo {
                                danger: filler.origin,
                            };
                            alive = true;
                            empty -= 1;
                        }
                        Space::ClosestTo { danger } => {
                            if danger == filler.origin {
                                // oops, double-accounting
                                continue;
                            }
                            let old_distance = danger.manhattan((x, y));
                            if old_distance == filler.distance {
                                spaces[x + y * SIZE] = Space::Tied {
                                    distance: filler.distance,
                                };
                                alive = true;
                            } else if old_distance > filler.distance {
                                spaces[x + y * SIZE] = Space::ClosestTo {
                                    danger: filler.origin,
                                };
                                alive = true;
                            }
                        }
                        Space::Tied { distance } => {
                            if distance > filler.distance {
                                spaces[x + y * SIZE] = Space::ClosestTo {
                                    danger: filler.origin,
                                };
                                alive = true;
                            }
                        }
                    }
                }
            }

            if alive {
                filler.distance += 1;
                filling.push(filler);
            }
        }

        // assert_eq!(empty, 0);

        for y in 0..SIZE {
            for x in 0..SIZE {
                let c = match spaces[x + y * SIZE] {
                    Space::Unvisited => "?".to_string(),
                    Space::Tied { distance: _ } => " ".to_string(),
                    Space::ClosestTo { danger } => {
                        let distance = danger.manhattan((x, y));
                        if distance <= 9 {
                            format!("{}", distance)
                        } else {
                            "+".to_string()
                        }
                    }
                };
                print!("{}", c);
            }
            println!();
        }
        // println!("{:?}", );

        // #[derive(Copy, Clone, Debug)]
        // struct FrontierPoint {
        //     coordinate: (uint, uint),
        //     nearest_danger_distance: uint,
        //     nearest_danger: (uint, uint),
        // }

        // let dangers = Arena::new();
        // dangers.alloc(Danger { coordinate: (0, 0) });

        // let _width = 1000;
        // let _height = 1000;

        (0, ())
    }
}

/*
--- Day 7: The Sum of Its Parts ---

You find yourself standing on a snow-covered coastline; apparently, you landed a little off course. The region is too hilly to see the North Pole from here, but you do spot some Elves that seem to be trying to unpack something that washed ashore. It's quite cold out, so you decide to risk creating a paradox by asking them for directions.

"Oh, are you the search party?" Somehow, you can understand whatever Elves from the year 1018 speak; you assume it's Ancient Nordic Elvish. Could the device on your wrist also be a translator? "Those clothes don't look very warm; take this." They hand you a heavy coat.

"We do need to find our way back to the North Pole, but we have higher priorities at the moment. You see, believe it or not, this box contains something that will solve all of Santa's transportation problems - at least, that's what it looks like from the pictures in the instructions." It doesn't seem like they can read whatever language it's in, but you can: "Sleigh kit. Some assembly required."

"'Sleigh'? What a wonderful name! You must help us assemble this 'sleigh' at once!" They start excitedly pulling more parts out of the box.

The instructions specify a series of steps and requirements about which steps must be finished before others can begin (your puzzle input). Each step is designated by a single letter. For example, suppose you have the following instructions:

Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.

Visually, these requirements look like this:


  -->A--->B--
 /    \      \
C      -->D----->E
 \           /
  ---->F-----

Your first goal is to determine the order in which the steps should be completed. If more than one step is ready, choose the step which is first alphabetically. In this example, the steps would be completed as follows:

    Only C is available, and so it is done first.
    Next, both A and F are available. A is first alphabetically, so it is done next.
    Then, even though F was available earlier, steps B and D are now also available, and B is the first alphabetically of the three.
    After that, only D and F are available. E is not available because only some of its prerequisites are complete. Therefore, D is completed next.
    F is the only choice, so it is done next.
    Finally, E is completed.

So, in this example, the correct order is CABDFE.

In what order should the steps in your instructions be completed?
*/

struct TheSumOfItsParts;
impl Problem<(), ()> for TheSumOfItsParts {
    fn day(&self) -> uint {
        7
    }
    fn name(&self) -> &'static str {
        "The Sum of Its Parts"
    }
    fn known_solution(&self) -> (Option<()>, Option<()>) {
        (None, None)
    }
    fn solve(&self, _input: &[&str]) -> ((), ()) {
        ((), ())
    }
}
