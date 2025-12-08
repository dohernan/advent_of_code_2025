use std::cmp::Reverse;
use std::fs;

#[derive(Debug, Default)]
struct BatteryBank {
    batteries: Vec<u64>,
}

impl From<&str> for BatteryBank {
    fn from(value: &str) -> Self {
        let batteries: Vec<u64> = value
            .chars()
            .map(|char| char.to_digit(10).unwrap() as u64)
            .collect();
        BatteryBank { batteries }
    }
}

impl BatteryBank {
    fn get_maximum_combined_joltage2(&self) -> u64 {
        let first_joltage_candidates = &self.batteries[..self.batteries.len() - 1];
        let first_joltage_position = first_joltage_candidates
            .iter()
            .enumerate()
            .max_by_key(|(key, value)| (*value, Reverse(*key)))
            .map(|(key, _)| key)
            .unwrap();
        let first_joltage = self.batteries[first_joltage_position];
        let second_joltage_candidates =
            &self.batteries[(first_joltage_position + 1)..self.batteries.len()];
        let second_joltage = second_joltage_candidates
            .iter()
            .enumerate()
            .max_by_key(|(key, value)| (*value, Reverse(*key)))
            .map(|(_, value)| value)
            .unwrap();
        first_joltage * 10 + second_joltage
    }

    fn get_maximum_combined_joltage12(&self) -> u64 {
        let number_of_batteries_to_turn_on = 12;
        let mut start_position = 0;
        let mut total_joltage = 0;
        for i in 0..number_of_batteries_to_turn_on {
            let candidates = &self.batteries
                [start_position..=(self.batteries.len() - number_of_batteries_to_turn_on + i)];
            let found_position = candidates
                .iter()
                .enumerate()
                .max_by_key(|(key, value)| (*value, Reverse(*key)))
                .map(|(key, _)| key)
                .unwrap()
                + start_position;
            let chosen_joltage = self.batteries[found_position];
            total_joltage +=
                chosen_joltage * 10_u64.pow((number_of_batteries_to_turn_on - 1 - i) as u32);
            start_position = found_position + 1;
        }
        total_joltage
    }
}

fn main() {
    // let mut v = vec![1, 2, 3, 4, 5, 6];
    // v.sort_by_key(|&num| (num < 3));
    // // v.sort_by_key(|&num| Reverse(num)); I know this means reverse the order.
    // dbg!(v);
    let input = fs::read_to_string("data/input.txt").expect("Failed to read input.txt");

    let battery_banks: Vec<BatteryBank> = input.lines().map(BatteryBank::from).collect();
    let result: u64 = battery_banks
        .iter()
        .map(|battery_bank| battery_bank.get_maximum_combined_joltage12())
        .sum();
    println!("All invalid Ids sum: {}", result);
}
