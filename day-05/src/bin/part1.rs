mod common;

use common::{split_lines, parse_mapper_to_layer};

#[allow(dead_code)]
fn main() {
    let input = include_str!("./input-5.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> u64 {
    let seeds = parse_seeds(&input);
    let layers = parse_mapper_to_layer(&input);
    let mut minimum_location = u64::MAX;
    for seed in seeds.iter() {
        let next = Box::new(*seed);
        let location = layers.iter()
            .fold(
                next,
                |current_state, level| {
                    let range_result = level.iter()
                        .find(|l| l.is_contains_in_source(*current_state));
                    match range_result {
                        None => current_state,
                        Some(layer) => Box::new(layer.transform(*current_state))
                    }
                },
            );
        if minimum_location > *location {
            minimum_location = *location
        }
    }
    return minimum_location;
}

fn parse_seeds(input: &Vec<&str>) -> Vec<u64> {
    let cloned_input = input.clone();
    let result = cloned_input.first().unwrap()
        .strip_prefix("seeds:").unwrap()
        .trim()
        .split(" ")
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution(split_lines(include_str!("./sample1-5.txt")));
        assert_eq!(result, 35);
    }
}