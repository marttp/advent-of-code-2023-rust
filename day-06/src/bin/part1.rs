mod common;

use common::{split_lines, read_input_to_max_record_list};

#[allow(dead_code)]
fn main() {
    let input = include_str!("./input-6.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> u128 {
    let max_record_list = read_input_to_max_record_list(input);
    return max_record_list.iter()
        .map(|max_record| max_record.possible_ways_to_beat())
        .reduce(|a, b| a * b)
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution(split_lines(include_str!("./sample1-6.txt")));
        assert_eq!(result, 288);
    }
}