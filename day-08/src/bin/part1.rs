mod common;

use std::ops::Deref;
use common::{split_lines, parse_directions, build_graph};
use crate::common::move_to_node;

const START_LOCATION: &str = "AAA";
const END_LOCATION: &str = "ZZZ";

#[allow(dead_code)]
fn main() {
    let input = include_str!("./input-8.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> u64 {
    let directions = parse_directions(input.clone());
    let graph = build_graph(input.clone());
    let start_position = graph.get(START_LOCATION).unwrap().clone();
    let mut current = start_position;
    let mut steps = 0;
    let mut current_direction_idx = 0;
    while current.borrow().position != END_LOCATION {
        let direction_order = &directions[current_direction_idx];
        let next = move_to_node(*direction_order, &current.borrow());
        // Directly assign the unwrapped value to `current`
        current = next.unwrap();
        steps += 1;
        current_direction_idx = (current_direction_idx + 1) % directions.len();
    }
    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let actual1 = solution(split_lines(include_str!("./sample1-8.txt")));
        assert_eq!(actual1, 2);
        let actual2 = solution(split_lines(include_str!("./sample2-8.txt")));
        assert_eq!(actual2, 6);
    }
}