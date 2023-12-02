#[allow(dead_code)]
use std::cmp::max;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Game {
    pub id: u32,
    pub stone_bag: StoneBag,
}

#[derive(Debug)]
pub struct StoneBag {
    pub max_red: u32,
    pub max_green: u32,
    pub max_blue: u32,
}

pub fn split_lines(input: &str) -> Vec<&str> {
    input.split("\n").collect()
}

#[allow(dead_code)]
pub fn create_game_map_rule<'a>() -> HashMap<&'a str, u32> {
    let mut table: HashMap<&str, u32> = HashMap::new();
    table.insert("red", 12);
    table.insert("green", 13);
    table.insert("blue", 14);
    return table;
}

pub fn collect_stones(line: &str) -> Game {
    let game_id_stone_split: Vec<&str> = line.clone().split(":").collect();
    let game_id = game_id_stone_split[0].trim()
        .split(" ").collect::<Vec<&str>>()[1]
        .parse::<u32>()
        .unwrap();

    let stones = game_id_stone_split[1].trim().split(";");
    let mut stone_bag = StoneBag {
        max_red: 0,
        max_green: 0,
        max_blue: 0,
    };

    for stone_pick_count in stones {
        let stone_count_by_color_list = stone_pick_count.trim().split(",");
        for stone_with_color in stone_count_by_color_list {
            let text = stone_with_color.trim().split(" ").collect::<Vec<&str>>();
            let result_amount = text[0].trim().parse::<u32>();
            let color = text[1].trim();
            match result_amount {
                Ok(amount) => {
                    match color {
                        "red" => stone_bag.max_red = max(stone_bag.max_red, amount),
                        "green" => stone_bag.max_green = max(stone_bag.max_green, amount),
                        "blue" => stone_bag.max_blue = max(stone_bag.max_blue, amount),
                        _ => {}
                    }
                }
                Err(error) => panic!("Cannot parse stone amount: {:?}", error)
            }
        }
    }

    return Game {
        id: game_id,
        stone_bag: stone_bag,
    };
}