extern crate itertools;
extern crate itertools_num;
use std::cell::{Ref, RefCell};
use std::fs;
use std::rc::Rc;
use std::str::Lines;

use itertools::{Itertools, all};

#[derive(Debug, Clone, Default)]

struct Node {
    row: usize,
    column: usize,
    right: Option<Rc<RefCell<Node>>>,
    left: Option<Rc<RefCell<Node>>>,
    value: u64,
}

impl Node {
    fn calculate_value(&mut self) {
        self.value = self.left.as_ref().unwrap().borrow().value
            + self.right.as_ref().unwrap().borrow().value;
    }
    fn new(row: usize, column: usize) -> Self {
        let new_left_node: Node = Node::default(100000000, column - 1);
        let left = Some(Rc::new(RefCell::new(new_left_node)));
        let new_right_node: Node = Node::default(100000000, column + 1);
        let right = Some(Rc::new(RefCell::new(new_right_node)));

        Node {
            row,
            column,
            right,
            left,
            value: 1,
        }
    }
    fn default(row: usize, column: usize) -> Self {
        Node {
            row,
            column,
            right: None,
            left: None,
            value: 1,
        }
    }
    fn set_right(&mut self, node: Rc<RefCell<Node>>) {
        self.right = Some(node);
    }
    fn set_left(&mut self, node: Rc<RefCell<Node>>) {
        self.left = Some(node);
    }
}
// fn try_direction(line_iterator: Lines<'_>, ways: &mut usize, max: &mut usize) {
//     let start_beam_column = line_iterator
//         .clone()
//         .next()
//         .unwrap()
//         .chars()
//         .enumerate()
//         .find(|(pos, char)| *char == 'S')
//         .unwrap()
//         .0;
//     let mut row_hist: Vec<(usize, usize)> = vec![(1, start_beam_column)];
//     let base_node = Node::default();
//     let mut nodes: Vec<Rc<RefCell<Node>>> = vec![Rc::new(RefCell::new(base_node))];
//     while let Some((row, beam_column)) = row_hist.clone().last() {
//         let mut row_search = *row;
//         let mut next_split_found = false;
//         let current_node = nodes.last().unwrap().clone();
//         nodes.remove(nodes.len() - 1);
//         row_hist.remove(row_hist.len() - 1);
//         while let Some(line) = line_iterator.clone().nth(row_search) {
//             if line.chars().nth(*beam_column).unwrap() == '^' {
//                 next_split_found = true;
//                 row_hist.push((row_search + 1, beam_column + 1));
//                 let new_node = Node::default();
//                 let reference = Rc::new(RefCell::new(new_node));
//                 current_node.clone().borrow_mut().right = Some(reference.clone());
//                 nodes.push(reference.clone());
//                 let new_node = Node::default();
//                 let reference = Rc::new(RefCell::new(new_node));
//                 current_node.clone().borrow_mut().left = Some(reference.clone());
//                 nodes.push(reference.clone());
//                 row_hist.push((row_search + 1, beam_column - 1));
//                 break;
//             }
//             row_search += 1;
//         }

//         if !next_split_found {
//             if *beam_column > *max {
//                 *max = *beam_column;
//                 println!("{}", beam_column);
//             }
//             *ways += 1;
//         }
//     }
// }

fn main() {
    let mut active_beam_columns: Vec<usize> = vec![];
    let input = fs::read_to_string("data/input.txt").expect("Failed to read input.txt");
    let start_beam = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .enumerate()
        .find(|(pos, char)| *char == 'S')
        .unwrap()
        .0;
    active_beam_columns.push(start_beam);

    let mut all_nodes: Vec<Rc<RefCell<Node>>> = vec![];
    let mut line_iterator = input.lines().rev().enumerate();
    let mut splits = 0;
    let mut ways = 0;
    let mut max = 0;
    //try_direction(line_iterator, &mut ways, &mut max);
    while let Some(line) = line_iterator.next()
        && !active_beam_columns.is_empty()
    {
        dbg!(line);
        for character in line.1.chars().enumerate() {
            if character.1 == '^' {
                let in_row = input.lines().count() - line.0;
                let in_column = character.0;
                let new_node: Node = Node::new(in_row, in_column);
                let new_node_ref = Rc::new(RefCell::new(new_node));

                let found_left: Option<Rc<RefCell<Node>>> = None;
                let found_right: Option<Rc<RefCell<Node>>> = None;
                for existing_node in &all_nodes {
                    let left_matches = (existing_node.borrow().column
                        == new_node_ref.borrow().column - 1)
                        && (found_left.is_none()
                            || found_left.as_ref().unwrap().borrow().row
                                > existing_node.borrow().row);

                    if left_matches {
                        new_node_ref.borrow_mut().left = Some(existing_node.clone());
                    }

                    let right_matches = (existing_node.borrow().column
                        == new_node_ref.borrow().column + 1)
                        && (found_right.is_none()
                            || found_right.as_ref().unwrap().borrow().row
                                > existing_node.borrow().row);

                    if right_matches {
                        new_node_ref.borrow_mut().right = Some(existing_node.clone());
                    }
                }
                new_node_ref.borrow_mut().calculate_value();
                all_nodes.push(new_node_ref);
            }
        }
    }

    // while let Some(line) = line_iterator.next()
    //     && !active_beam_columns.is_empty()
    // {
    //     for beam in active_beam_columns.clone() {
    //         if line.1.chars().nth(beam).unwrap() == '^' {
    //             let new_node: Node = Node::new(line.0, beam);
    //             let new_node_ref = Rc::new(RefCell::new(new_node));

    //             for existing_node in &all_nodes {
    //                 let mut existing_node_borrowed = existing_node.borrow_mut();

    //                 let left_matches = existing_node_borrowed.left.as_ref().is_some_and(|left| {
    //                     left.borrow().column == beam && left.borrow().row > line.0
    //                 });

    //                 if left_matches {
    //                     existing_node_borrowed.left = Some(new_node_ref.clone());
    //                 }

    //                 let right_matches =
    //                     existing_node_borrowed.right.as_ref().is_some_and(|right| {
    //                         right.borrow().column == beam && right.borrow().row > line.0
    //                     });

    //                 if right_matches {
    //                     existing_node_borrowed.right = Some(new_node_ref.clone());
    //                 }
    //             }
    //             all_nodes.push(new_node_ref.clone());

    //             if let None = active_beam_columns
    //                 .iter()
    //                 .find(|active_beam| **active_beam == beam + 1)
    //             {
    //                 active_beam_columns.push(beam + 1);
    //             }
    //             if let None = active_beam_columns
    //                 .iter()
    //                 .find(|active_beam| **active_beam == beam - 1)
    //             {
    //                 active_beam_columns.push(beam - 1);
    //             }
    //             active_beam_columns.remove(
    //                 active_beam_columns
    //                     .iter()
    //                     .enumerate()
    //                     .find(|(_, active_beam)| **active_beam == beam)
    //                     .unwrap()
    //                     .0,
    //             );
    //             splits += 1;
    //         }
    //     }
    // }
    println!("All ways: {}", ways);
    println!("Value: {}", all_nodes.last().unwrap().borrow().value);
}
