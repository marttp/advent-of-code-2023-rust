use std::collections::HashSet;
use std::str::Chars;

pub fn split_lines(input: &str) -> Vec<&str> {
    input.split("\n").collect()
}

pub fn solve_problem(galaxy_info: &GalaxyInfo, expand_time: u64) -> i64 {
    let empty_row_set = galaxy_info.empty_row_list.iter()
        .cloned().collect::<HashSet<_>>();
    let empty_column_set = galaxy_info.empty_column_list.iter()
        .cloned().collect::<HashSet<_>>();

    let new_coordinate_calculate_points = galaxy_info.galaxies.clone().iter().map(|galaxy| {
        let lower_row_amount = empty_row_set.iter()
            .filter(|row| **row < galaxy.row)
            .count();
        let lower_column_amount = empty_column_set.iter()
            .filter(|col| **col < galaxy.column)
            .count();

        Point {
            row: galaxy.row + (lower_row_amount * (expand_time - 1) as usize),
            column: galaxy.column + (lower_column_amount * (expand_time - 1) as usize),
        }
    })
        .collect::<Vec<_>>();

    let mut answer = 0;

    for i in 0..new_coordinate_calculate_points.len() - 1 {
        for j in i + 1..new_coordinate_calculate_points.len() {
            let distance = manhattan_distance(&new_coordinate_calculate_points[i], &new_coordinate_calculate_points[j]);
            answer += distance;
        }
    }

    return answer;
}

fn manhattan_distance(p1: &Point, p2: &Point) -> i64 {
    (p1.row as i64 - p2.row as i64).abs() + (p1.column as i64 - p2.column as i64).abs()
}

pub fn get_galaxies_info(input: Vec<&str>) -> GalaxyInfo {
    let mut space_observation: Vec<Chars> = Vec::new();
    input.iter().for_each(|row| {
        space_observation.push(row.chars());
    });

    let mut galaxies: Vec<Point> = Vec::new();
    space_observation.iter().enumerate().for_each(|(row_idx, row)| {
        row.clone().enumerate().for_each(|(col_idx, col)| {
            if col == '#' {
                galaxies.push(Point {
                    row: row_idx,
                    column: col_idx,
                });
            }
        });
    });

    let mut empty_row_list: Vec<usize> = collect_empty_row(&space_observation);
    let mut empty_column_list: Vec<usize> = collect_empty_column(&space_observation);

    GalaxyInfo {
        galaxies,
        empty_row_list,
        empty_column_list,
    }
}

fn collect_empty_row(space_observation: &Vec<Chars>) -> Vec<usize> {
    let mut empty_row_list: Vec<usize> = Vec::new();
    space_observation.iter().enumerate().for_each(|(row_idx, mut row)| {
        if row.clone().all(|col| col == '.') {
            empty_row_list.push(row_idx);
        }
    });
    empty_row_list
}

fn collect_empty_column(space_observation: &Vec<Chars>) -> Vec<usize> {
    let mut empty_col_list: Vec<usize> = Vec::new();
    let max_col_size = space_observation[0].clone().count();
    for col_idx in 0..max_col_size {
        let column_from_all_row = space_observation.iter()
            .map(|row| row.clone().nth(col_idx).unwrap())
            .collect::<Vec<_>>();
        if column_from_all_row.iter().all(|col| col == &'.') {
            empty_col_list.push(col_idx);
        }
    }
    empty_col_list
}

#[derive(Debug, Clone)]
pub struct Point {
    pub row: usize,
    pub column: usize,
}

pub struct GalaxyInfo {
    pub galaxies: Vec<Point>,
    pub empty_row_list: Vec<usize>,
    pub empty_column_list: Vec<usize>,
}
