use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn split_lines(input: &str) -> Vec<&str> {
    input.split("\n").collect()
}

type NodeRef = Rc<RefCell<Node>>;

#[derive(Debug, Clone)]
pub struct Node {
    pub position: String,
    pub left: Option<NodeRef>,
    pub right: Option<NodeRef>,
}

pub fn parse_directions(input: Vec<&str>) -> Vec<char> {
    input.first().unwrap().chars().collect()
}

pub fn build_graph(input: Vec<&str>) -> HashMap<String, Rc<RefCell<Node>>> {
    let mut graph = HashMap::new();
    let mut cloned_input = input.clone();
    for line in cloned_input.iter().skip(2) {
        let name_to_neighbors = line.split('=').map(|s| s.trim()).collect::<Vec<&str>>();
        let name = name_to_neighbors[0].to_string();
        let neighbors_format = name_to_neighbors[1].to_string();
        let neighbors = neighbors_format[1..neighbors_format.len() - 1].split(',').map(|s| s.trim()).collect::<Vec<&str>>();
        let left = neighbors[0].to_string();
        let right = neighbors[1].to_string();

        // Create or get nodes
        let node = graph.entry(name.clone()).or_insert_with(|| Rc::new(RefCell::new(Node {
            position: name.clone(),
            left: None,
            right: None,
        })));
        let left_node = graph.entry(left.clone()).or_insert_with(|| Rc::new(RefCell::new(Node {
            position: left.clone(),
            left: None,
            right: None,
        })));
        let right_node = graph.entry(right.clone()).or_insert_with(|| Rc::new(RefCell::new(Node {
            position: right.clone(),
            left: None,
            right: None,
        })));
    }


    for line in cloned_input.iter().skip(2) {
        let name_to_neighbors = line.split('=').map(|s| s.trim()).collect::<Vec<&str>>();
        let name = name_to_neighbors[0].to_string();
        let neighbors_format = name_to_neighbors[1].to_string();
        let neighbors = neighbors_format[1..neighbors_format.len() - 1].split(',').map(|s| s.trim()).collect::<Vec<&str>>();
        let left = neighbors[0].to_string();
        let right = neighbors[1].to_string();

        if let (Some(node), Some(left_node), Some(right_node)) = (graph.get(&name), graph.get(&left), graph.get(&right)) {
            let mut node_borrow = node.borrow_mut();
            node_borrow.left = Some(left_node.clone());
            node_borrow.right = Some(right_node.clone());
        }
    }

    graph
}

pub fn move_to_node(direction: char, node: &Node) -> Option<NodeRef> {
    match direction {
        'L' => node.left.clone(),
        'R' => node.right.clone(),
        _ => panic!("Current support only L and R"),
    }
}

pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

pub fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}