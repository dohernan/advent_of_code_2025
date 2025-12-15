extern crate itertools;
extern crate itertools_num;
extern crate lp_modeler;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

use std::fs;
use std::rc::Rc;

#[derive(Debug, Clone, Default, PartialEq)]
struct Node {
    id: u16,
    visited: bool,
    connections: Vec<Rc<RefCell<Node>>>,
    parents: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn add_connection(&mut self, node: Rc<RefCell<Node>>) {
        self.connections.push(node)
    }

    fn add_parent(&mut self, node: Rc<RefCell<Node>>) {
        self.parents.push(node)
    }

    fn get_all_paths_to_out(&self) -> usize {
        if self.connections.is_empty() {
            return 1;
        }
        self.connections
            .iter()
            .map(|connection| connection.borrow().get_all_paths_to_out())
            .sum()
    }

    fn get_all_following_nodes(&self, following_nodes: &mut HashSet<u16>) {
        following_nodes.insert(self.id);
        if self.connections.is_empty() {
            return;
        }
        self.connections.iter().for_each(|connection| {
            if !following_nodes.contains(&connection.borrow().id) {
                connection.borrow().get_all_following_nodes(following_nodes)
            }
        });
    }

    fn get_all_parent_nodes(&self, parent_nodes: &mut HashSet<u16>) {
        if self.parents.is_empty() {
            return;
        }
        parent_nodes.insert(self.id);
        self.parents.iter().for_each(|parent| {
            if !parent_nodes.contains(&parent.borrow().id) {
                parent.borrow().get_all_parent_nodes(parent_nodes)
            }
        });
    }

    fn get_all_paths_to(
        &self,
        goal: u16,
        following_nodes: &HashSet<u16>,
        parent_nodes: &HashSet<u16>,
    ) -> usize {
        if self.id == goal {
            return 1;
        }
        self.connections
            .iter()
            .map(|connection| {
                if following_nodes.contains(&connection.borrow().id)
                    && parent_nodes.contains(&connection.borrow().id)
                {
                    connection
                        .borrow()
                        .get_all_paths_to(goal, following_nodes, parent_nodes)
                } else {
                    0
                }
            })
            .sum()
    }
}

impl From<u16> for Node {
    fn from(id: u16) -> Self {
        Node {
            id,
            visited: false,
            connections: vec![],
            parents: vec![],
        }
    }
}

fn main() {
    let input_string = fs::read_to_string("data/input.txt").expect("Failed to read input.txt");
    let mut node_dict: HashMap<u16, Vec<&str>> = HashMap::new();
    let mut node_translation: HashMap<&str, u16> = HashMap::new();
    let mut id = 0;
    let mut nodes: HashMap<u16, Rc<RefCell<Node>>> = HashMap::new();
    let mut head: u16 = 0;
    let mut svr: u16 = 0;
    let mut out: u16 = 0;
    let mut fft: u16 = 0;
    let mut dac: u16 = 0;
    let mut visited: Vec<bool> = vec![];
    input_string.lines().for_each(|line| {
        let conn = line.split(':').collect::<Vec<&str>>();
        if conn[0] == "you" {
            head = id;
        }
        if conn[0] == "fft" {
            fft = id;
        }
        if conn[0] == "dac" {
            dac = id;
        }
        if conn[0] == "svr" {
            svr = id;
        }
        node_translation.insert(conn[0], id);
        nodes.insert(id, Rc::new(RefCell::new(Node::from(id))));
        visited.push(false);
        node_dict.insert(id, conn[1][1..].split(" ").collect());
        id += 1;
    });
    node_translation.insert("out", id);
    out = id;
    nodes.insert(id, Rc::new(RefCell::new(Node::from(id))));
    visited.push(false);
    for (node_key, conn_str) in node_dict {
        for conn in conn_str {
            let connection = nodes[&node_translation[conn]].clone();
            //ln!("{} -> {}", node_key, connection.borrow().id);
            nodes[&node_key]
                .borrow_mut()
                .add_connection(connection.clone());
            connection.borrow_mut().add_parent(nodes[&node_key].clone());
        }
    }
    println!("{}", nodes[&head].borrow().get_all_paths_to_out());
    let mut fft_following_nodes = HashSet::new();
    let mut dac_following_nodes = HashSet::new();
    let mut dac_parent_nodes = HashSet::new();
    let mut fft_parent_nodes = HashSet::new();
    nodes[&fft]
        .borrow()
        .get_all_following_nodes(&mut fft_following_nodes);
    nodes[&dac]
        .borrow()
        .get_all_following_nodes(&mut dac_following_nodes);
    nodes[&fft]
        .borrow()
        .get_all_parent_nodes(&mut fft_parent_nodes);
    nodes[&dac]
        .borrow()
        .get_all_parent_nodes(&mut dac_parent_nodes);
    let first_fft =
        nodes[&svr]
            .borrow()
            .get_all_paths_to(fft, &fft_parent_nodes, &fft_parent_nodes);

    let fft_dac =
        nodes[&fft]
            .borrow()
            .get_all_paths_to(dac, &fft_following_nodes, &dac_parent_nodes);
    let dac_end =
        nodes[&dac]
            .borrow()
            .get_all_paths_to(out, &dac_following_nodes, &dac_following_nodes);
    let one = first_fft * fft_dac * dac_end;
    println!("{}", one);
}
