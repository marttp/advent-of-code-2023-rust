mod common;

use common::{split_lines};
use crate::common::{get_galaxies_info, solve_problem};

#[allow(dead_code)]
fn main() {
    let input = include_str!("./input-11.txt");
    let output = solution(split_lines(input), 1_000_000);
    dbg!(output);
}

fn solution(input: Vec<&str>, expand_time: u64) -> i64 {
    let galaxy_info = get_galaxies_info(input);
    solve_problem(&galaxy_info, expand_time)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let actual = solution(split_lines(include_str!("./sample1-11.txt")), 2);
        assert_eq!(actual, 374);
        let actual = solution(split_lines(include_str!("./sample1-11.txt")), 10);
        assert_eq!(actual, 1030);
        let actual = solution(split_lines(include_str!("./sample1-11.txt")), 100);
        assert_eq!(actual, 8410);
    }
}