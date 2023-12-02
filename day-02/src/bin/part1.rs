mod common;

use common::{split_lines, collect_stones, create_game_map_rule};

fn main() {
    let input = include_str!("./input-2.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> u32 {
    let game_rule = create_game_map_rule();
    let mut sum: u32 = 0;
    for line in input {
        let game_info = collect_stones(line);
        let stone_bag = game_info.stone_bag;
        if stone_bag.max_red <= *game_rule.get("red").unwrap() &&
            stone_bag.max_green <= *game_rule.get("green").unwrap() &&
            stone_bag.max_blue <= *game_rule.get("blue").unwrap() {
            sum += game_info.id;
        }
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution(split_lines(include_str!("./sample1-2.txt")));
        assert_eq!(result, 8);
    }
}