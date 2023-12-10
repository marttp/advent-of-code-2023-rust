mod common;

use common::{split_lines};
use crate::common::{build_graph, move_to_node, parse_directions, lcm};

const START_LOCATION: &str = "AAA";
const END_LOCATION: &str = "ZZZ";
const SUFFIX_START: char = 'A';
const SUFFIX_END: char = 'Z';

#[allow(dead_code)]
fn main() {
    let input = include_str!("./input-8.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> u64 {
    let directions = parse_directions(input.clone());
    let graph = build_graph(input.clone());
    let start_positions = graph.keys()
        .filter(|k| k.ends_with(SUFFIX_START))
        .map(|k| graph.get(k).unwrap().clone())
        .collect::<Vec<_>>();
    let mut steps_all_result: Vec<u64> = Vec::new();

    for start_position in start_positions {
        let mut current = start_position;
        let mut steps = 0;
        let mut current_direction_idx = 0;
        while !current.borrow().position.ends_with(SUFFIX_END) {
            let direction_order = &directions[current_direction_idx];
            let next = move_to_node(*direction_order, &current.borrow());
            // Directly assign the unwrapped value to `current`
            current = next.unwrap();
            steps += 1;
            current_direction_idx = (current_direction_idx + 1) % directions.len();
        }
        steps_all_result.push(steps);
    }

    steps_all_result.iter()
        .cloned()
        .reduce(|a, b| lcm(a, b))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution(split_lines(include_str!("./sample3-8.txt")));
        assert_eq!(result, 6);
    }
}