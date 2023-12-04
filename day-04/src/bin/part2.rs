mod common;

use common::{split_lines};
use crate::common::read_to_card_point;

#[allow(dead_code)]
fn main() {
    let input = include_str!("./input-4.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> u32 {
    let card_point_list = read_to_card_point(input);
    let card_point_size: usize = card_point_list.len();
    let mut scratch_card_counter: Vec<u32> = vec![0; card_point_size];
    for (index, card_point) in card_point_list.iter().enumerate() {
        let matched_size = card_point.intersect_size as usize;
        let is_this_card_win = matched_size != 0;
        scratch_card_counter[index] += 1;
        if is_this_card_win {
            let next = index + matched_size;
            for i in (index + 1)..=next {
                if i < scratch_card_counter.len() {
                    scratch_card_counter[i] += scratch_card_counter[index];
                }
            }
        }
    }
    return scratch_card_counter.iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution(split_lines(include_str!("./sample2-4.txt")));
        assert_eq!(result, 30);
    }
}