use std::collections::HashSet;
use std::ops::Not;

pub fn split_lines(input: &str) -> Vec<&str> {
    input.split("\n").collect()
}

pub fn read_to_card_point(input: Vec<&str>) -> Vec<CardPoint> {
    let mut card_point_list: Vec<CardPoint> = Vec::new();
    let cloned_input = input.clone();
    for (_, line) in cloned_input.iter().enumerate() {
        let cards = line.split(":").nth(1).unwrap().trim();
        let separated_cards = cards.split("|")
            .map(|c| c.trim())
            .collect::<Vec<_>>();
        let winning_numbers = separated_cards[0].split(" ")
            .filter(|c| c.is_empty().not())
            .map(|p| p.parse::<u32>().unwrap())
            .collect::<HashSet<_>>();
        let have_numbers = separated_cards[1].split(" ")
            .filter(|c| c.is_empty().not())
            .map(|p| p.parse::<u32>().unwrap())
            .collect::<HashSet<_>>();
        let matched_set = have_numbers.intersection(&winning_numbers).collect::<HashSet<_>>();
        let matched_size = matched_set.len();
        let points = if matched_size != 0 {
            2_u32.pow((matched_size - 1) as u32)
        } else {
            0
        };
        let card_point = CardPoint {
            points,
            intersect_size: matched_size as u32,
            winning_number_set: winning_numbers.clone(),
            have_number_set: have_numbers.clone(),
        };
        card_point_list.push(card_point)
    }
    return card_point_list;
}

#[derive(Debug)]
pub struct CardPoint {
    pub points: u32,
    pub intersect_size: u32,
    pub winning_number_set: HashSet<u32>,
    pub have_number_set: HashSet<u32>,
}