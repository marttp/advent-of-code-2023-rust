use std::collections::HashMap;

pub const EIGHT_DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1), // top left
    (-1, 0), // top
    (-1, 1), // top right

    (0, -1), // left
    (0, 1), // right

    (1, -1), // bottom left
    (1, 0), // bottom
    (1, 1), // bottom right
];

pub fn split_lines(input: &str) -> Vec<&str> {
    input.split("\n").collect()
}

pub fn collect_number(input: Vec<&str>) -> Vec<NumberPosition> {
    let cloned_input = input.clone();
    let mut result: Vec<NumberPosition> = Vec::new();
    for (row, line) in cloned_input.iter().enumerate() {
        let max_col = line.len();
        let mut col = 0 as usize;
        while col < max_col {
            let c = line.chars().nth(col).unwrap();
            col += if is_not_dot(c) && c.is_digit(10) {
                let start_col = col;
                let mut end_col = col;
                while end_col < max_col &&
                    is_not_dot(line.chars().nth(end_col).unwrap()) &&
                    line.chars().nth(end_col).unwrap().is_digit(10) {
                    end_col += 1;
                }
                let slice_result = &line[start_col..end_col];
                let number_value = &slice_result
                    .parse::<u32>()
                    .unwrap();
                let number_position = NumberPosition {
                    number: *number_value,
                    row: row as i32,
                    start_col: start_col as i32,
                    end_col: (end_col - 1) as i32,
                };
                result.push(number_position);
                end_col - start_col
            } else {
                1
            }
        }
    }
    return result;
}

pub fn collect_symbol(input: Vec<&str>) -> Vec<SymbolPosition> {
    let cloned_input = input.clone();
    let mut result: Vec<SymbolPosition> = Vec::new();
    for (row, line) in cloned_input.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if is_not_dot_and_not_digit(c) {
                let symbol_position = SymbolPosition {
                    symbol: c,
                    row: row as i32,
                    col: col as i32,
                };
                result.push(symbol_position);
            }
        }
    }
    return result;
}

pub fn is_in_bound(row: i32, col: i32, max_row: i32, max_col: i32) -> bool {
    return row >= 0 && row < max_row && col >= 0 && col < max_col;
}

pub fn is_not_dot_and_not_digit(c: char) -> bool {
    return is_not_dot(c) && is_not_digit(c);
}

pub fn is_dot(c: char) -> bool {
    return c == '.';
}

pub fn is_not_dot(c: char) -> bool {
    return !is_dot(c);
}

pub fn is_not_digit(c: char) -> bool {
    return !c.is_digit(10);
}

#[derive(Debug)]
pub struct NumberPosition {
    pub number: u32,
    pub row: i32,
    pub start_col: i32,
    pub end_col: i32,
}

impl NumberPosition {
    pub fn get_unique_key(&self) -> String {
        let number = self.number;
        let row = self.row;
        let start_col = self.start_col;
        let end_col = self.end_col;
        let key = format!("{number}:{row}:{start_col}:{end_col}");
        return key;
    }
}

#[derive(Debug)]
pub struct SymbolPosition {
    pub symbol: char,
    pub row: i32,
    pub col: i32,
}

#[derive(Debug)]
pub struct EngineSchematicInfo {
    pub max_row: i32,
    pub max_col: i32,
    pub number_in_row: HashMap<i32, Vec<NumberPosition>>,
    pub symbol_positions: Vec<SymbolPosition>,
}