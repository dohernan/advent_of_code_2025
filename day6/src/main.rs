extern crate itertools;
extern crate itertools_num;
use std::cmp;
use std::fs;
use std::str::Lines;

use itertools::Itertools;

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

fn apply_operation(operation: &str, value1: &i64, value2: i64) -> i64 {
    match operation {
        "+" => value1 + value2,
        _ => value1 * value2,
    }
}

fn is_space(pos: usize, input: &String) -> bool {
    let last = input.lines().last().unwrap();
    let mut result = true;
    input.lines().enumerate().for_each(|(_, line)| {
        if line == last {
            println!("Skipping last line");
            return;
        }
        result &= line.as_bytes().get(pos) == Some(&b' ')
    });
    result
}

fn main() {
    let number_length = 4;
    let input = fs::read_to_string("data/input.txt").expect("Failed to read input.txt");
    //let mut ids: Vec<u64> = vec![];
    let operations: Vec<&str> = input.lines().last().unwrap().split(';').collect();
    let mut numbers: Vec<Vec<i64>> = vec![vec![]; operations.len()];
    let mut results: Vec<i64> = vec![0; operations.len()]
        .iter()
        .enumerate()
        .map(|(pos, _)| if operations[pos] == "+" { 0 } else { 1 })
        .collect();
    //let a = input.lines();
    input.lines().enumerate().for_each(|(pos_line, line)| {
        let mut pos_column_group = 0;
        let mut pos_inner = 0;
        if line != input.lines().last().unwrap() {
            line.chars().enumerate().for_each(|(pos, value)| {
                if !is_space(pos, &input) {
                    if value != ' ' {
                        let value_digit = value.to_digit(10).unwrap() as i64;
                        let value_to_introduce =
                            value_digit * 10_i64.pow((number_length - pos_line - 1) as u32) as i64;
                        if numbers[pos_column_group].len() < pos_inner + 1 {
                            numbers[pos_column_group].push(value_to_introduce);
                        } else {
                            numbers[pos_column_group][pos_inner] += value_to_introduce;
                        }
                    } else {
                        if numbers[pos_column_group].len() >= pos_inner + 1 {
                            numbers[pos_column_group][pos_inner] /= 10;
                        } else {
                            numbers[pos_column_group].push(0);
                        }
                    }
                    pos_inner += 1;
                } else {
                    pos_column_group += 1;
                    pos_inner = 0;
                }
            });
            //.map(|number_str| number_str.parse::<i64>().unwrap())
            //.collect();
            // numbers.iter().enumerate().for_each(|(pos, value)| {
            //     results[pos] = apply_operation(operations[pos], &results[pos], *value)
            // });
        }
    });
    numbers.iter().enumerate().for_each(|(pos, vectori)| {
        vectori.iter().for_each(|value| {
            results[pos] = apply_operation(operations[pos], &results[pos], *value)
        });
    });
    // println!("All ids: {}", unique_ids.count_unique_ids());
    println!("All invalid Ids sum: {}", results.iter().sum::<i64>());
}
