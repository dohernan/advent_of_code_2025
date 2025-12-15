extern crate itertools;
extern crate itertools_num;
extern crate lp_modeler;
use std::cell::RefCell;
use std::collections::HashMap;

use std::fs;
use std::hash::Hash;
use std::rc::Rc;

#[derive(Debug, Clone, Default, PartialEq)]
struct Node {
    id: String,
    connections: Vec<Rc<RefCell<Node>>>,
}

impl From<String> for Node {
    fn from(id: String) -> Self {
        Node {
            id,
            connections: vec![],
        }
    }
}

fn main() {
    let input_string = fs::read_to_string("data/input.txt").expect("Failed to read input.txt");
    let mut node_dict: HashMap<String, String> = HashMap::new();
    let mut nodes: Vec<Node> = vec![];
    input_string.lines().for_each(|line| {
        Node::from(line.split(':')[0]);
    });
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
