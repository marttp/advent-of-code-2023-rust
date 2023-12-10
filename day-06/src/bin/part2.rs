mod common;

use common::{split_lines, read_input_fix_kerning_record};

#[allow(dead_code)]
fn main() {
    let input = include_str!("./input-6.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> u128 {
    let max_record = read_input_fix_kerning_record(input);
    return max_record.possible_ways_to_beat();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution(split_lines(include_str!("./sample1-6.txt")));
        assert_eq!(result, 71503);
    }
}