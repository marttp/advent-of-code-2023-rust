pub fn split_lines(input: &str) -> Vec<&str> {
    input.split("\n").collect()
}


pub fn parse_input(input: &Vec<&str>) -> Vec<Vec<String>> {
    let mut cloned_input = input.clone();
    let result = cloned_input
        .join("\n")
        .split("\n\n")
        .map(|s|
            s.split("\n")
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();
    return result;
}

pub fn calculate_mirror_pattern(pattern: &Vec<String>, smudge_fix_amount: u32) -> u32 {
    let horizontal_mirror = find_horizontal_mirror(pattern, smudge_fix_amount);
    match horizontal_mirror {
        None => {
            let vertical_mirror = find_vertical_mirror(pattern, smudge_fix_amount);
            match vertical_mirror {
                None => { panic!("No mirror pattern found"); }
                Some(it) => { Some(it) }
            }
        }
        Some(it) => { Some(it) }
    }
        .unwrap()
}

fn find_horizontal_mirror(pattern: &Vec<String>, smudge_fix_amount: u32) -> Option<u32> {
    let row_index_range = 0..pattern.len() - 1;
    for start_idx in row_index_range {
        let mirror_range = create_mirror_ranges(start_idx, pattern.len() - 1);
        let sum_of_difference = mirror_range.iter()
            .map(|(up, down)| {
                difference(&pattern[*up], &pattern[*down])
            })
            .sum::<u32>();
        if sum_of_difference == smudge_fix_amount {
            return Some((start_idx + 1) as u32 * 100);
        }
    }
    return None;
}

fn find_vertical_mirror(pattern: &Vec<String>, smudge_fix_amount: u32) -> Option<u32> {
    let col_index_range = 0..pattern.first().unwrap().len() - 1;
    for start_idx in col_index_range {
        let mirror_range = create_mirror_ranges(start_idx, pattern.first().unwrap().len() - 1);
        let sum_of_difference = mirror_range.iter()
            .map(|(left, right)| {
                difference(&column_to_string(pattern, *left), &column_to_string(pattern, *right))
            })
            .sum::<u32>();
        if sum_of_difference == smudge_fix_amount {
            return Some((start_idx + 1) as u32);
        }
    }
    return None;
}

fn column_to_string(pattern: &Vec<String>, col: usize) -> String {
    return pattern.iter()
        .map(|s| s.chars().nth(col).unwrap().to_string())
        .collect::<Vec<_>>()
        .join("");
}

fn difference(pattern: &String, other: &String) -> u32 {
    let mut result = 0;
    for (i, c) in pattern.chars().enumerate() {
        if c != other.chars().nth(i).unwrap() {
            result += 1;
        }
    }
    return result + (pattern.len() as i32 - other.len() as i32).abs() as u32;
}

fn create_mirror_ranges(start: usize, max: usize) -> Vec<(usize, usize)> {
    return (0..=start).rev().zip(start + 1..=max).collect();
}