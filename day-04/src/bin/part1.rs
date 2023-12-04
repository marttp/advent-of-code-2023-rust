mod common;

use common::{split_lines, read_to_card_point};

#[allow(dead_code)]
fn main() {
    let input = include_str!("./input-4.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> u32 {
    return read_to_card_point(input)
        .iter()
        .map(|cp| cp.points)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution(split_lines(include_str!("./sample1-4.txt")));
        assert_eq!(result, 13);
    }
}