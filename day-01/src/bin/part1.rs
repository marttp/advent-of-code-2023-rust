#[allow(dead_code)]
fn main() {
    let input = include_str!("./input-1.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> u32 {
    let mut sum: u32 = 0;
    for line in input {
        let (first, last) = get_first_and_last_digit(line);
        let fist_value = first.value as u32;
        let last_value = last.value as u32;
        sum += fist_value * 10 + last_value;
    }
    return sum;
}

pub fn get_first_and_last_digit(line: &str) -> (Digit, Digit) {
    let mut first = Digit {
        value: 0,
        index: line.len() as i8,
    };
    let mut last = Digit {
        value: 0,
        index: -1,
    };
    for (i, c) in line.chars().enumerate() {
        if c.is_digit(10) {
            if first.value == 0 {
                first.value = c.to_digit(10).unwrap() as u8;
                first.index = i as i8;
            }
            last.value = c.to_digit(10).unwrap() as u8;
            last.index = i as i8;
        }
    }
    return (first, last);
}

#[derive(Debug)]
pub struct Digit {
    pub value: u8,
    pub index: i8,
}

fn split_lines(input: &str) -> Vec<&str> {
    input.split("\n").collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution(split_lines(include_str!("./sample1-1.txt")));
        assert_eq!(result, 142);
    }
}