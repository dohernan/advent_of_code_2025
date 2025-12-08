use std::fs;

fn is_invalid(id: u64) -> bool {
    let mut divisor: u64 = 10;
    while id / divisor > 0 {
        if id % divisor == id / divisor && id / divisor >= divisor / 10 {
            return true;
        }
        divisor *= 10;
    }
    false
}

fn is_invalid2(id: u64) -> bool {
    let mut id_helper = id;
    let mut divisor: u64 = 10;
    'outer: while id / divisor > 0 {
        let id_ref = id_helper % divisor;
        if id_ref < divisor / 10 {
            divisor *= 10;
            continue;
        }
        id_helper /= divisor;
        if id_helper > 0 {
            while id_helper > 0 {
                let id_part = id_helper % divisor;
                if id_part != id_ref {
                    divisor *= 10;
                    id_helper = id;
                    continue 'outer;
                }
                id_helper /= divisor;
            }
            return true;
        }
    }
    false
}

#[derive(Debug, Default)]
struct IdRange {
    start: u64,
    end: u64,
}

impl From<&str> for IdRange {
    fn from(value: &str) -> Self {
        let numbers: Vec<u64> = value
            .split('-')
            .map(|number_str| dbg!(number_str).parse::<u64>().unwrap())
            .collect();
        IdRange {
            start: numbers[0],
            end: numbers[1],
        }
    }
}

impl IdRange {
    fn accumulated_invalid_ids(&self) -> u64 {
        let mut accumulate = 0;
        for id in self.start..=self.end {
            if is_invalid2(id) {
                accumulate += id;
            }
        }
        accumulate
    }
}

fn main() {
    let input = fs::read_to_string("data/input.txt").expect("Failed to read input.txt");

    let instructions: Vec<IdRange> = input.split(',').map(IdRange::from).collect();
    let result: u64 = instructions
        .iter()
        .map(|range| range.accumulated_invalid_ids())
        .sum();
    println!("All invalid Ids sum: {}", result);
}
