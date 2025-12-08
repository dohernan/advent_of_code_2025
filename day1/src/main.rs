use std::fs;

#[derive(Debug, Default, PartialEq)]
enum Direction {
    #[default]
    Left,
    Right,
}

#[derive(Debug, Default)]
struct Dial {
    position: i16,
    zero_points: u16,
}

impl Dial {
    const NUMBER_OF_POSITIONS: i16 = 100;
    fn rotate(&mut self, rotation: Rotation) {
        let delta: i16 = match rotation.direction {
            Direction::Left => -rotation.positions,
            Direction::Right => rotation.positions,
        };
        let starts_in_zero = self.position == 0;
        self.position += delta;
        if self.position == 0 {
            self.zero_points += 1;
        }
        if self.position < 0 {
            while self.position < 0 {
                self.position += Dial::NUMBER_OF_POSITIONS;
                self.zero_points += 1;
            }
            if self.position == 0 {
                self.zero_points += 1; // Count the zero point if we land exactly on it
            }
            if starts_in_zero {
                self.zero_points -= 1; // The original position zero point is already counted in the last rotation
            }
        }
        while self.position >= Dial::NUMBER_OF_POSITIONS {
            self.position -= Dial::NUMBER_OF_POSITIONS;
            self.zero_points += 1;
        }
        if self.position >= Dial::NUMBER_OF_POSITIONS || self.position < 0 {
            println!("ERROR");
            return;
        }
        //dbg!(self);
    }
    fn execute_instructions(&mut self, instructions: Vec<Rotation>) {
        for rotation in instructions {
            self.rotate(rotation);
        }
    }
    fn get_zero_points(&self) -> u16 {
        self.zero_points
    }
}

#[derive(Debug, Default)]
struct Rotation {
    direction: Direction,
    positions: i16,
}

impl From<&str> for Rotation {
    fn from(value: &str) -> Self {
        let direction = match value.chars().next() {
            Some('L') => Direction::Left,
            Some('R') => Direction::Right,
            _ => Direction::Left,
        };
        let positions = value[1..].trim().parse::<i16>().unwrap();
        Rotation {
            direction,
            positions,
        }
    }
}

fn main() {
    let mut dial = Dial {
        position: 50,
        ..Dial::default()
    };
    dial.position = 50;
    let input = fs::read_to_string("data/input.txt").expect("Failed to read input.txt");

    let instructions: Vec<Rotation> = input.lines().map(|line| Rotation::from(line)).collect();
    dial.execute_instructions(instructions);
    println!("Password: {}", dial.get_zero_points());
}
