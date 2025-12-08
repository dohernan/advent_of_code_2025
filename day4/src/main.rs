use std::fs;

#[derive(Debug, Default)]
struct RollGrid(Vec<Vec<i32>>);

impl From<String> for RollGrid {
    fn from(input: String) -> Self {
        RollGrid(
            input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c: char| match c {
                            '@' => 1,
                            _ => 0,
                        })
                        .collect()
                })
                .collect(),
        )
    }
}

impl RollGrid {
    fn get_adjacent_sum(&self, row: usize, column: usize) -> i32 {
        let columns = self.0.len();
        let rows = self.0[0].len();
        let max_column = if column >= columns - 1 {
            column
        } else {
            column + 1
        };
        let min_column = if column <= 0 { column } else { column - 1 };
        let max_row = if row >= rows - 1 { row } else { row + 1 };
        let min_row = if row <= 0 { row } else { row - 1 };

        let mut total: i32 = 0;
        total -= self.0[row][column];
        for row_it in min_row..=max_row {
            for column_it in min_column..=max_column {
                total += self.0[row_it][column_it];
            }
        }
        total
    }
    fn is_roll_of_paper(&self, row: usize, column: usize) -> bool {
        self.0[row][column] > 0
    }
    // fn get_sum_of_accessible_rolls(&self) -> i32 {
    //     let mut sum = 0;
    //     for row in 0..self.0.len() {
    //         for column in 0..self.0[0].len() {
    //             if self.is_roll_of_paper(row, column) && self.get_adjacent_sum(row, column) < 4 {
    //                 sum += 1;
    //             }
    //         }
    //     }
    //     dbg!(sum)
    // }

    fn get_accessible_rolls(&self) -> Vec<(usize, usize)> {
        let mut accesible_rolls = vec![];
        for row in 0..self.0.len() {
            for column in 0..self.0[0].len() {
                if self.is_roll_of_paper(row, column) && self.get_adjacent_sum(row, column) < 4 {
                    accesible_rolls.push((row, column))
                }
            }
        }
        accesible_rolls
    }

    fn remove_rolls(&mut self) {
        for roll_position in self.get_accessible_rolls() {
            self.0[roll_position.0][roll_position.1] = 0;
        }
    }

    fn get_maximum_removed(&mut self) -> usize {
        let mut total = 0;
        while !self.get_accessible_rolls().is_empty() {
            total += self.get_accessible_rolls().len();
            self.remove_rolls();
        }
        total
    }
}

fn main() {
    let input = fs::read_to_string("data/input.txt").expect("Failed to read input.txt");

    let mut battery_banks: RollGrid = RollGrid::from(input);

    println!(
        "All removed rolls sum: {}",
        battery_banks.get_maximum_removed()
    );
}
