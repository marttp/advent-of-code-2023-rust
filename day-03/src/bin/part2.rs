mod common;

use std::collections::{HashMap, HashSet};
use common::{split_lines};
use crate::common::{collect_number, collect_symbol, EIGHT_DIRECTIONS, EngineSchematicInfo, is_in_bound, NumberPosition};

#[allow(dead_code)]
fn main() {
    let input = include_str!("./input-3.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> u32 {
    let max_row = input.len() as i32;
    let max_col = input[0].len() as i32;
    let symbol_positions = collect_symbol(input.clone());
    let number_positions = collect_number(input.clone());

    let mut number_in_row: HashMap<i32, Vec<NumberPosition>> = HashMap::new();
    for np in number_positions {
        let row = np.row;
        number_in_row.entry(row)
            .or_insert_with(Vec::new)
            .push(np);
    }

    let engine_schematic_info = EngineSchematicInfo {
        max_row,
        max_col,
        number_in_row,
        symbol_positions,
    };

    return process_gear_ratio(engine_schematic_info);
}

fn process_gear_ratio(engine_schematic_info: EngineSchematicInfo) -> u32 {
    let mut used_set: HashSet<String> = HashSet::new();
    let star_symbols = engine_schematic_info.symbol_positions
        .iter()
        .filter(|c| c.symbol == '*')
        .collect::<Vec<_>>();
    let mut sum = 0;
    for ss in star_symbols {
        let mut pre_processor_number_positions: Vec<NumberPosition> = Vec::new();
        for (r, c) in EIGHT_DIRECTIONS {
            let next_possible_row = ss.row + r;
            let next_possible_col = ss.col + c;
            let is_in_bound = is_in_bound(
                next_possible_row,
                next_possible_col,
                engine_schematic_info.max_row,
                engine_schematic_info.max_col,
            );
            if is_in_bound {
                match engine_schematic_info.number_in_row.get(&next_possible_row) {
                    None => {}
                    Some(next_possible_number_position_list) => {
                        for npp in next_possible_number_position_list.iter() {
                            let key = npp.get_unique_key();
                            if !used_set.contains(&key) && next_possible_col >= npp.start_col && next_possible_col <= npp.end_col {
                                used_set.insert(key);
                                let new_npp = NumberPosition {
                                    number: npp.number,
                                    row: npp.row,
                                    start_col: npp.start_col,
                                    end_col: npp.end_col,
                                };
                                pre_processor_number_positions.push(new_npp);
                            }
                        }
                    }
                }
            }
        }
        let produce_result = if pre_processor_number_positions.len() == 2 {
            pre_processor_number_positions[0].number * pre_processor_number_positions[1].number
        } else {
            0
        };
        sum += produce_result;
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution(split_lines(include_str!("./sample1-3.txt")));
        assert_eq!(result, 467835);
    }
}