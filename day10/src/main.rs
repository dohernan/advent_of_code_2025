extern crate itertools;
extern crate itertools_num;
use good_lp::*;
extern crate lp_modeler;

use std::error::Error;
use std::{fs, vec};

use itertools::Itertools;

#[derive(Debug, Clone, Default, PartialEq)]
struct IndicatorLight {
    lights_goal: Vec<u16>,
    wiring_schematics_transformed: Vec<u128>,
    wiring_schematics: Vec<Vec<u16>>,
    joltages: Vec<u16>,
    joltages_transformed: u128,
}

impl IndicatorLight {
    fn is_goal(&self, lights: Vec<u16>) -> bool {
        lights
            .iter()
            .enumerate()
            .all(|(pos, light)| *light == self.lights_goal[pos])
    }

    fn is_group_reaches_goal(&self, group: Vec<Vec<u16>>) -> bool {
        let mut combination: Vec<u16> = vec![0; self.lights_goal.len()];
        for wiring in group.iter() {
            for &wire in wiring {
                combination[wire as usize] = (combination[wire as usize] + 1) % 2;
            }
        }
        self.is_goal(combination)
    }
    fn is_transformed_group_reaches_goal(&self, group: Vec<u128>) -> bool {
        let mut total_wiring = group.iter().sum::<u128>();
        let expt = 10_u128.pow(3 * (self.joltages.len() as u32 - 1 as u32));
        if total_wiring < expt {
            return false;
        }
        while total_wiring > 0 {
            if total_wiring % 1000 == 0 {
                return false;
            }
            total_wiring /= 1000;
        }

        true
    }

    fn joltage_position_is_done(&self, position: u16, old: u16) -> bool {
        position > old
    }

    fn recursive_check(
        &self,
        max_values: &Vec<u16>,
        current_combination: &mut Vec<u16>,
        index: usize,
        remaining: u16,
        total: u128,
    ) -> usize {
        if index == current_combination.len() {
            // if remaining != 0
            //     || current_combination.iter().sum::<u16>() < *max_values.iter().max().unwrap()
            // {
            //     return 0;
            // }
            if self.joltages_transformed == total {
                println!("Group found: {:?}", current_combination);
                return index;
            }
            return 0;
        }
        if index > 0
            && self.joltage_position_is_done(
                self.wiring_schematics[index][0],
                self.wiring_schematics[index - 1][0],
            )
        {
            let expt = 10_u128.pow(
                3 * (self.joltages.len() as u32 - 1 - self.wiring_schematics[index - 1][0] as u32),
            );
            if total / expt < self.joltages_transformed / expt {
                return 0;
            }
        }

        // Try all possible values for current_combination position (0 to min(remaining, max_values[index]))
        for value in 0..=remaining.min(max_values[index]) {
            current_combination[index] = value;
            let mut max_values_rec = max_values.clone();

            if max_values_rec[index] >= value {
                max_values_rec[index] -= value;
            } else {
                max_values_rec[index] = 0;
            }
            let total_rec: u128 = total + value as u128 * self.wiring_schematics_transformed[index];
            let recurs = self.recursive_check(
                &max_values_rec,
                current_combination,
                index + 1,
                remaining - value,
                total_rec,
            );
            if recurs > 0 {
                return recurs;
            }
        }

        0
    }

    fn check_groups(&self, of: usize) -> usize {
        let groups = &mut self
            .wiring_schematics
            .clone()
            .into_iter()
            .combinations(of)
            .collect::<Vec<Vec<Vec<u16>>>>();
        for group in groups {
            if self.is_group_reaches_goal(group.clone()) {
                println!("Group found: {}", group.len());
                return group.len();
            }
        }
        0
    }
    fn check_groups2(&mut self) -> usize {
        let mut max_of_this_wiring_schema = vec![0; self.wiring_schematics.len()];
        let mut wirings_for_joltage = vec![vec![]; self.joltages.len()];
        for (pos, wiring) in self.wiring_schematics.iter().enumerate() {
            let mut max_times_used = 9999;
            for joltage_used in wiring.iter() {
                wirings_for_joltage[*joltage_used as usize].push(pos);
                max_times_used = max_times_used.min(self.joltages[*joltage_used as usize])
            }
            max_of_this_wiring_schema[pos] = max_times_used;
        }
        let mut variables: Vec<VariableDefinition> = vec![];
        let mut problem = ProblemVariables::new();
        for i in 0..self.wiring_schematics.len() {
            variables.push(variable().min(0).integer());
        }
        let y: Vec<Variable> = problem.add_all(variables);

        let mut solution = problem
            .minimise(y.iter().sum::<Expression>())
            .using(default_solver); // IBM's coin_cbc by default
        // for (i, &y_i) in y.iter().enumerate() {
        //     solution = solution.with(constraint!(y_i <= max_of_this_wiring_schema[i]));
        // }
        for (i, wirings_for_joltage) in wirings_for_joltage.iter().enumerate() {
            let mut sum_of_wirings: Expression = Expression::default();
            for &wiring_that_uses_that_joltage in wirings_for_joltage.iter() {
                sum_of_wirings += y[wiring_that_uses_that_joltage];
            }
            solution = solution.with(constraint!(sum_of_wirings == self.joltages[i]));
        }
        let s = solution.solve().unwrap();
        let mut result: usize = 0;
        for y_i in y {
            println!("a={} ", s.value(y_i));
            result += s.value(y_i) as usize;
        }
        result
        // let mut current_combination = vec![0u16; self.wiring_schematics_transformed.len()];
        // self.recursive_check(
        //     &max_of_this_wiring_schema.clone(),
        //     &mut current_combination,
        //     0,
        //     of,
        //     0,
        // )
    }

    fn get_min_group(&self) -> usize {
        let length = self.wiring_schematics[0].len();
        for of in 1..=length {
            if self.check_groups(of) > 0 {
                return of;
            }
        }
        println!("ERROR: NOT FOUND");
        9999999999
    }

    fn rest_min_group(&mut self) -> usize {
        let length = self.wiring_schematics[0].len();
        for of in 1..=length {
            if self.check_groups(of) > 0 {
                return of;
            }
        }
        println!("ERROR: NOT FOUND");
        9999999999
    }

    fn rest_min_all_group(&mut self) -> usize {
        self.lights_goal.fill(1);
        let length = self.wiring_schematics[0].len();
        for of in 1..=length {
            if self.check_all_groups(of) {
                return of;
            }
        }
        println!("ERROR: NOT FOUND");
        9999999999
    }

    fn check_all_groups(&mut self, of: usize) -> bool {
        let groups = &mut self
            .wiring_schematics_transformed
            .clone()
            .into_iter()
            .combinations(of)
            .collect::<Vec<Vec<u128>>>();
        for group in groups {
            if self.is_transformed_group_reaches_goal(group.clone()) {
                //println!("Group found: {}", group.len());
                // resta del total cada grupo multiplicado
                for &mut wiring in group {
                    self.joltages_transformed -= wiring;
                }
                return true;
            }
        }
        false
    }

    fn get_min_group2(&mut self) -> usize {
        let mut res_min_all: usize = 0;
        let mut a = 0;
        // 'outer: loop {
        //     let mut joltages_transformed = self.joltages_transformed;
        //     while joltages_transformed > 0 {
        //         if joltages_transformed % 1000 < 9999 {
        //             break 'outer;
        //         }
        //         joltages_transformed /= 1000;
        //     }
        //     a = self.rest_min_all_group() as u128;

        //     res_min_all += a as usize;
        // }

        let mut first_of = 0;
        let mut joltages_transformed = self.joltages_transformed;
        while joltages_transformed > 0 {
            first_of = first_of.max(joltages_transformed % 1000);
            joltages_transformed /= 1000;
        }

        self.check_groups2()
    }
}

impl From<Vec<Vec<char>>> for IndicatorLight {
    fn from(indicator: Vec<Vec<char>>) -> Self {
        let exponent = 3;

        let lights_goal: Vec<u16> = indicator
            .first()
            .unwrap()
            .iter()
            .filter(|&&char| char == '#' || char == '.')
            .map(|lights_vector| match lights_vector {
                '.' => 0,
                '#' => 1,
                _ => 0,
            })
            .collect();
        let joltages: Vec<u16> = indicator
            .last()
            .unwrap()
            .iter()
            .copied()
            .collect::<String>()
            .split(',')
            .filter_map(|number| number.replace("{", "").replace("}", "").parse::<u16>().ok())
            .collect();
        let mut joltages_transformed = 0;
        for (pos, &joltage_val) in joltages.iter().enumerate() {
            joltages_transformed += joltage_val as u128
                * 10_u128.pow(exponent * (joltages.len() as u32 - 1 - pos as u32));
        }

        let mut wiring_data: Vec<(Vec<u16>, u128)> = indicator[1..indicator.len() - 1]
            .iter()
            .map(|wiring| {
                let schema_vec: Vec<u16> = wiring
                    .iter()
                    .copied()
                    .collect::<String>()
                    .split(',')
                    .filter_map(|number| {
                        number.replace("(", "").replace(")", "").parse::<u16>().ok()
                    })
                    .collect();

                let mut schema_transformed = 0;
                for char in wiring {
                    if let Some(pos) = char.to_digit(10) {
                        schema_transformed +=
                            10_u128.pow(exponent * (joltages.len() as u32 - 1 - pos as u32));
                    }
                }

                (schema_vec, schema_transformed)
            })
            .collect();

        // Sort by first element of schema_vec
        wiring_data.sort_by_key(|(schema_vec, _)| schema_vec.get(0).copied().unwrap_or(0));

        // Separate back into two vectors while maintaining sorted order
        let (wiring_schematics, wiring_schematics_transformed): (Vec<Vec<u16>>, Vec<u128>) =
            wiring_data.into_iter().unzip();

        IndicatorLight {
            lights_goal,
            wiring_schematics,
            wiring_schematics_transformed,
            joltages,
            joltages_transformed,
        }
    }
}

fn main() {
    let input_string = fs::read_to_string("data/input.txt").expect("Failed to read input.txt");

    let mut indicator_lights: Vec<IndicatorLight> = input_string
        .lines()
        .map(|line| {
            IndicatorLight::from(
                line.split(' ')
                    .map(|element| element.chars().collect())
                    .collect::<Vec<Vec<char>>>(),
            )
        })
        .collect();

    // println!("{}", indicator_light);
    let mut suma = 0;
    let mut suma_jolt = 0;
    for indicator_light in &mut indicator_lights {
        //suma += indicator_light.get_min_group();
        suma_jolt += dbg!(indicator_light.get_min_group2());
    }
    println!("Sum: {}", suma);
    println!("Sumjolt: {}", suma_jolt);
}
