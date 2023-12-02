use std::collections::HashMap;

mod part1;

#[allow(dead_code)]
fn main() {
    let input = include_str!("./input-1.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> u32 {
    let mut sum: u32 = 0;
    let number_table: HashMap<&str, u8> = create_number_table();

    for line in input {
        let (first_num, last_num) = part1::get_first_and_last_digit(line);
        let (first_text, last_text) = get_first_and_last_digit_with_text(line, &number_table);

        let final_first_digit = if first_num.index < first_text.index {
            first_num.value as u32
        } else {
            first_text.value as u32
        };

        let final_last_digit = if last_num.index > last_text.index {
            last_num.value as u32
        } else {
            last_text.value as u32
        };

        sum += final_first_digit * 10 + final_last_digit;
    }
    return sum;
}

fn create_number_table<'a>() -> HashMap<&'a str, u8> {
    let mut number_table: HashMap<&str, u8> = HashMap::new();
    number_table.insert("one", 1);
    number_table.insert("two", 2);
    number_table.insert("three", 3);
    number_table.insert("four", 4);
    number_table.insert("five", 5);
    number_table.insert("six", 6);
    number_table.insert("seven", 7);
    number_table.insert("eight", 8);
    number_table.insert("nine", 9);
    return number_table;
}

pub fn get_first_and_last_digit_with_text(line: &str, number_table: &HashMap<&str, u8>) -> (Digit, Digit) {
    let mut first_digit = Digit {
        value: 0,
        index: line.len() as i8,
    };
    let mut last_digit = Digit {
        value: 0,
        index: -1,
    };

    for (key, value) in number_table.into_iter() {
        let first_found_index = line.find(key);
        match first_found_index {
            None => {}
            Some(found_index) => {
                let index = found_index as i8;
                // Utilize first index
                if index < first_digit.index {
                    first_digit.index = index;
                    first_digit.value = value.clone();
                }
            }
        }
        let last_found_index = line.rfind(key);

        match last_found_index {
            None => {}
            Some(found_index) => {
                let index = found_index as i8;
                // Manage on last index
                if index > last_digit.index {
                    last_digit.index = index;
                    last_digit.value = value.clone();
                }
            }
        }
    }

    return (first_digit, last_digit);
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
        let result = solution(split_lines(include_str!("./sample2-1.txt")));
        assert_eq!(result, 281);
    }
}