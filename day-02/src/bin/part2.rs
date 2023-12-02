mod common;

use common::{split_lines, collect_stones};

#[allow(dead_code)]
fn main() {
    let input = include_str!("./input-2.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> u32 {
    let mut sum: u32 = 0;
    for line in input {
        let game_info = collect_stones(line);
        let stone_bag = game_info.stone_bag;
        sum += stone_bag.max_blue * stone_bag.max_red * stone_bag.max_green
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution(split_lines(include_str!("./sample1-2.txt")));
        assert_eq!(result, 2286);
    }
}