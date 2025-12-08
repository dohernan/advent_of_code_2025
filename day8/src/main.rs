extern crate itertools;
extern crate itertools_num;
use std::cell::RefCell;
use std::cmp::min;
use std::rc::Rc;
use std::{fs, vec};

use itertools::{Itertools, all, join};

#[derive(Debug, Clone, Default, PartialEq)]
struct Position(f64, f64, f64);
impl Position {
    fn distance(&self, other: &Position) -> f64 {
        let dx = self.0 - other.0;
        let dy = self.1 - other.1;
        let dz = self.2 - other.2;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
    fn is_equal(&self, other: &Position) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}
#[derive(Debug, Clone, Default)]
struct Circuit {
    positions: Vec<Position>,
    connections: Vec<(Position, Position)>,
}

impl Circuit {
    fn get_min_distance(&self, other: &Circuit) -> (f64, Position, Position) {
        let mut min_distance = 9999999999.;
        let (mut position_origin, mut position_destiny) =
            (self.positions[0].clone(), self.positions[0].clone());
        for position in &self.positions {
            for other_position in &other.positions {
                let distance = position.distance(other_position);
                if distance > 0.
                    && distance < min_distance
                    && !self.is_connected(position, other_position)
                {
                    min_distance = distance;
                    (position_origin, position_destiny) = (position.clone(), other_position.clone())
                }
            }
        }
        (min_distance, position_origin, position_destiny)
    }

    fn is_connected(&self, first: &Position, other: &Position) -> bool {
        self.connections.iter().any(|connection| {
            let found1 = connection.0.is_equal(first) && connection.1.is_equal(other);
            let found2 = connection.1.is_equal(first) && connection.0.is_equal(other);

            found1 || found2
        })
    }

    fn connect(
        &mut self,
        other: &mut Circuit,
        connection: (Position, Position),
        is_different_circuit: bool,
    ) {
        if is_different_circuit {
            self.positions.append(&mut other.positions);
            self.connections.append(&mut other.connections);
        }
        self.connections.push(connection);
    }
}
#[derive(Debug, Clone, Default)]
struct Circuits(Vec<Circuit>);

impl From<Vec<Circuit>> for Circuits {
    fn from(value: Vec<Circuit>) -> Self {
        Circuits(value)
    }
}

impl Circuits {
    fn get_closest_distance_and_merge(&mut self) {
        let mut origin_circuit_index = 0;
        let mut destiny_circuit_index = 0;
        let mut origin_position = Position::default();
        let mut destiny_position = Position::default();
        let mut min_distance = 9999999999.;

        for (i, circuit) in self.0.iter().enumerate() {
            for (j, other_circuit) in self.0.iter().enumerate() {
                if i == j {
                    continue;
                }
                let (current_min_distance, current_origin_position, current_destiny_position) =
                    circuit.get_min_distance(other_circuit);
                if current_min_distance < min_distance {
                    min_distance = current_min_distance;
                    origin_circuit_index = i;
                    destiny_circuit_index = j;
                    destiny_position = current_destiny_position;
                    origin_position = current_origin_position;
                }
            }
        }
        let mut destiny = self.0[destiny_circuit_index].clone();

        self.0[origin_circuit_index].connect(
            &mut destiny,
            (origin_position, destiny_position),
            origin_circuit_index != destiny_circuit_index,
        );
        if origin_circuit_index != destiny_circuit_index {
            self.0.remove(destiny_circuit_index);
        }
    }

    fn get_size_of_three_largest(&self) -> usize {
        self.0
            .iter()
            .map(|c| c.positions.len())
            .sorted()
            .rev()
            .take(3)
            .product()
    }
}

fn main() {
    let how_many_connections = 1000;
    let input_string = fs::read_to_string("data/input.txt").expect("Failed to read input.txt");
    let mut circuits = Circuits::from(
        input_string
            .lines()
            .map(|line| {
                let nums: Vec<f64> = line
                    .split(',')
                    .map(|number| number.parse::<f64>().unwrap())
                    .collect::<Vec<f64>>();
                Circuit {
                    positions: vec![Position(nums[0], nums[1], nums[2])],
                    connections: vec![],
                }
            })
            .collect::<Vec<Circuit>>(),
    );

    //for _ in 0..how_many_connections {
    while circuits.0.len() > 1 {
        circuits.get_closest_distance_and_merge();
        let conn = circuits
            .0
            .iter()
            .map(|circuit| circuit.connections.len())
            .sum::<usize>();
        println!("total conn {}", conn);
        println!("total groups {}", circuits.0.len());
    }
    println!(
        "total conn {}",
        circuits
            .0
            .iter()
            .map(|circuit| circuit.connections.len())
            .sum::<usize>()
    );
    println!(
        "Multiply 3 together {}",
        circuits.get_size_of_three_largest()
    );
    dbg!(circuits.0.last().unwrap().connections.last());
    dbg!(circuits.0.last().unwrap().connections.last().unwrap().0.0);
    dbg!(circuits.0.last().unwrap().connections.last().unwrap().1.0);
    dbg!(
        circuits.0.last().unwrap().connections.last().unwrap().0.0
            * circuits.0.last().unwrap().connections.last().unwrap().1.0
    );
}
