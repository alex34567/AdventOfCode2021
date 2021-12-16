use lazy_static::lazy_static;
use regex::Regex;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::{Rc, Weak};

#[derive(Copy, Clone, PartialEq, Eq)]
enum NodeType {
    Start,
    Lower,
    Higher,
    End,
}

struct Node {
    #[allow(dead_code)]
    name: String,
    node_type: NodeType,
    connections: RefCell<Vec<Weak<Node>>>,
    visited: Cell<u8>,
}

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"^(\w+)-(\w+)$").unwrap();
}

static INPUT: &str = include_str!("input/Day12.txt");

fn insert_node_factory(name: &str) -> impl FnOnce() -> Rc<Node> + '_ {
    move || {
        let is_capital = name.chars().next().unwrap().is_uppercase();
        let node_type = if is_capital {
            NodeType::Higher
        } else {
            NodeType::Lower
        };
        Rc::new(Node {
            name: name.to_string(),
            node_type,
            connections: RefCell::new(Vec::new()),
            visited: Cell::new(0),
        })
    }
}

fn depth_search(
    curr: &Node,
    part1: &mut usize,
    part2: &mut usize,
    double_visit_token: bool,
    #[allow(dead_code)] dbg_name_stack: &mut Vec<String>,
) {
    //dbg_name_stack.push(curr.name.clone());
    match curr.node_type {
        NodeType::Start | NodeType::Lower => {
            curr.visited.set(curr.visited.get() + 1);
        }
        NodeType::Higher => (),
        NodeType::End => {
            if double_visit_token {
                *part1 += 1;
            }
            *part2 += 1;

            //eprintln!("{:?}", dbg_name_stack);
            //dbg_name_stack.pop();
            return;
        }
    }
    for raw_connection in curr.connections.borrow().iter() {
        let mut new_double_visit = double_visit_token;
        let connected_node = raw_connection.upgrade().unwrap();
        if connected_node.visited.get() != 0 {
            if connected_node.node_type != NodeType::Start && double_visit_token {
                new_double_visit = false;
            } else {
                continue;
            }
        }
        depth_search(
            &connected_node,
            part1,
            part2,
            new_double_visit,
            dbg_name_stack,
        )
    }
    if curr.node_type == NodeType::Lower || curr.node_type == NodeType::Start {
        curr.visited.set(curr.visited.get() - 1);
    }
    //dbg_name_stack.pop();
}

pub fn day12() {
    let mut nodes = HashMap::new();
    let start = Rc::new(Node {
        name: "start".to_string(),
        node_type: NodeType::Start,
        connections: RefCell::new(Vec::new()),
        visited: Cell::new(0),
    });
    nodes.insert("start".to_string(), start.clone());

    let end = Rc::new(Node {
        name: "end".to_string(),
        node_type: NodeType::End,
        connections: RefCell::new(Vec::new()),
        visited: Cell::new(0),
    });
    nodes.insert("end".to_string(), end);

    for line in INPUT.trim().split('\n') {
        let captures = REGEX.captures(line).unwrap();
        let node_a_name = &captures[1];
        let node_b_name = &captures[2];

        let node_a = nodes
            .entry(node_a_name.to_string())
            .or_insert_with(insert_node_factory(node_a_name))
            .clone();
        let node_b = nodes
            .entry(node_b_name.to_string())
            .or_insert_with(insert_node_factory(node_b_name))
            .clone();

        let mut node_a_connections = node_a.connections.borrow_mut();
        node_a_connections.push(Rc::downgrade(&node_b));
        let mut node_b_connections = node_b.connections.borrow_mut();
        node_b_connections.push(Rc::downgrade(&node_a));
    }

    let mut part1 = 0;
    let mut part2 = 0;
    let mut dbg_name_stack = Vec::new();
    depth_search(&start, &mut part1, &mut part2, true, &mut dbg_name_stack);

    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
}
