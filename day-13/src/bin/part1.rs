mod common;

use common::{split_lines, calculate_mirror_pattern, parse_input};

#[allow(dead_code)]
fn main() {
    let input = include_str!("./input-13.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> u32 {
    let patterns = parse_input(&input);
    return patterns.iter()
        .map(|pattern| calculate_mirror_pattern(pattern, 0))
        .sum::<u32>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let actual = solution(split_lines(include_str!("./sample1-13.txt")));
        assert_eq!(actual, 405);
    }
}