mod common;

use common::{split_lines};

#[allow(dead_code)]
fn main() {
    let input = include_str!("./input-9.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> i64 {
    let mut answer = 0;

    for line in input.into_iter() {
        let num_list = line.split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        answer += calculate_with_all_zeroes_rule(&num_list);
    }

    return answer;
}

fn calculate_with_all_zeroes_rule(nums: &Vec<i64>) -> i64 {
    let mut is_all_zeroes = false;
    let mut cloned_nums = nums.clone();

    let mut stored_results: Vec<Vec<i64>> = Vec::new();
    stored_results.push(cloned_nums.clone()); // Store first line

    while !is_all_zeroes {
        is_all_zeroes = true;
        let mut tmp_nums = Vec::new();
        for i in 1..cloned_nums.len() {
            let diff = cloned_nums[i] - cloned_nums[i - 1];
            if diff == 0 {
                tmp_nums.push(0);
            } else {
                is_all_zeroes = false;
                tmp_nums.push(diff);
            }
        }
        cloned_nums = tmp_nums;
        stored_results.push(cloned_nums.clone());
    }

    let reverse_cloned_nums = stored_results.iter()
        .rev()
        .collect::<Vec<_>>();

    return reverse_cloned_nums.iter().fold(0, |acc, list| *list.first().unwrap() - acc);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let actual = solution(split_lines(include_str!("./sample1-9.txt")));
        assert_eq!(actual, 2);
    }
}