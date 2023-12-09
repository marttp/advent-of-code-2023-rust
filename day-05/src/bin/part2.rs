mod common;

use std::ops::Range;
use common::{split_lines};
use crate::common::parse_mapper_to_layer;

#[allow(dead_code)]
fn main() {
    let input = include_str!("./input-5.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> u64 {
    let seeds = parse_seeds(&input);
    let layers_reverse = parse_mapper_to_layer(&input)
        .iter()
        .map(|level|
            level.iter()
                .map(|layer| layer.flip())
                .collect::<Vec<_>>()
        )
        .rev()
        .collect::<Vec<_>>();
    for location in 0..u64::MAX {
        let next = Box::new(location);
        let target_seed = layers_reverse.iter()
            .fold(next, |curr, level| {
                let range_result = level.iter()
                    .find(|l| l.is_contains_in_source(*curr));
                match range_result {
                    None => curr,
                    Some(layer) => Box::new(layer.transform(*curr))
                }
            });
        let is_contain_target_seed = seeds.iter()
            .any(|s| s.contains(&target_seed));
        match is_contain_target_seed {
            true => return location,
            false => {}
        }
    }
    return u64::MAX;
}

fn parse_seeds(input: &Vec<&str>) -> Vec<Range<u64>> {
    let cloned_input = input.clone();
    let seed_input = cloned_input.first().unwrap()
        .strip_prefix("seeds:").unwrap()
        .trim()
        .split(" ")
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    return seed_input.chunks(2)
        .map(|c| Range {
            start: *c.get(0).unwrap(),
            end: c.get(0).unwrap() + c.get(1).unwrap(),
        })
        .collect::<Vec<_>>();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution(split_lines(include_str!("./sample1-5.txt")));
        assert_eq!(result, 46);
    }
}