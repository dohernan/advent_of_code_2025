extern crate itertools;
extern crate itertools_num;
use std::cmp;
use std::fs;

#[derive(Debug, Default)]
struct IdRanges {
    ranges: Vec<IdRange>,
}

impl IdRanges {
    fn insert(&mut self, id_range: &mut IdRange) {
        let mut temp_ranges = vec![];
        for existing_range in &mut self.ranges {
            if existing_range.is_overlaping(id_range) {
                id_range.combine(existing_range);
            } else {
                temp_ranges.push(*existing_range);
            }
        }
        self.ranges = temp_ranges;
        self.ranges.push(*id_range);
    }

    fn count_unique_ids(&self) -> u64 {
        let mut total = 0;
        for range in &self.ranges {
            total += range.end - range.start + 1;
        }
        total
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct IdRange {
    start: u64,
    end: u64,
}

impl From<&str> for IdRange {
    fn from(value: &str) -> Self {
        let numbers: Vec<u64> = value
            .split('-')
            .map(|number_str| number_str.parse::<u64>().unwrap())
            .collect();
        IdRange {
            start: numbers[0],
            end: numbers[1],
        }
    }
}

impl IdRange {
    // fn is_inside(&self, id: u64) -> bool {
    //     id >= self.start && id <= self.end
    // }
    // fn get_all_ids(&self) -> std::ops::RangeInclusive<u64> {
    //     self.start..=self.end
    // }
    fn is_overlaping(&self, other: &IdRange) -> bool {
        other.start <= self.end && other.end >= self.start
    }
    fn combine(&mut self, other: &IdRange) {
        self.start = cmp::min(self.start, other.start);
        self.end = cmp::max(self.end, other.end);
    }
}

fn main() {
    let input = fs::read_to_string("data/input.txt").expect("Failed to read input.txt");
    //let mut ids: Vec<u64> = vec![];
    let mut unique_ids = IdRanges::default();

    input.lines().for_each(|line| {
        if !line.is_empty() && line.find('-').is_some() {
            let mut id_range = IdRange::from(line);
            unique_ids.insert(&mut id_range);
        }
        // } else {
        //     ids.push(line.parse::<u64>().unwrap())
        // }
    });
    // let mut fresh_ids = 0;
    // 'outer: for id in ids {
    //     for id_range in &id_ranges {
    //         if id_range.is_inside(id) {
    //             fresh_ids += 1;
    //             continue 'outer;
    //         }
    //     }
    // }

    // println!("Fresh ids: {}", fresh_ids);

    println!("All ids: {}", unique_ids.count_unique_ids());
    //println!("All invalid Ids sum: {}", result);
}
