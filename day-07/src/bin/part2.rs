mod common;

use std::collections::{HashMap, HashSet};
use common::{split_lines, Hands, CamelCard, CamelCardWinType, total_winnings};
use crate::common::CamelCardWinType::{FIVE_OF_A_KIND, FOUR_OF_A_KIND, FULL_HOUSE, HIGH_CARD, ONE_PAIR, THREE_OF_A_KIND};

const POSSIBLE_LABELS: [char; 13] = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];
const CARD_STRENGTH: [u8; 13] = [14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2];

#[allow(dead_code)]
fn main() {
    let input = include_str!("./input-7.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> u64 {
    let input_rule2 = parse_input_rule2(input.clone());
    let result = total_winnings(input_rule2);
    return result;
}

fn parse_input_rule2(input: Vec<&str>) -> Vec<Hands> {
    return input.iter().map(|line| {
        let split_line = line.split(" ").collect::<Vec<&str>>();
        let labels = split_line[0].chars().collect::<Vec<char>>();
        let bid = split_line[1].parse::<u32>().unwrap();
        let camel_cards = labels.iter().map(|label| {
            let index = POSSIBLE_LABELS.iter().position(|&x| x == *label).unwrap();
            let strength = CARD_STRENGTH[index];
            CamelCard {
                label: *label,
                strength,
            }
        }).collect::<Vec<CamelCard>>();
        return Hands::new(camel_cards.clone(), bid, rule2(camel_cards.clone()));
    })
        .collect::<Vec<_>>();
}

fn rule1(cards: Vec<CamelCard>) -> CamelCardWinType {
    let label_count = cards.iter()
        .fold(
            HashMap::new(),
            |mut acc, card| {
                let count = acc.entry(card.label).or_insert(0);
                *count += 1;
                acc
            });
    let values = label_count.values().collect::<HashSet<&u8>>();
    match label_count.len() {
        1 => FIVE_OF_A_KIND,
        2 => {
            return if values.contains(&4) {
                FOUR_OF_A_KIND
            } else {
                FULL_HOUSE
            }
        },
        3 => {
            return if values.contains(&3) {
                THREE_OF_A_KIND
            } else {
                CamelCardWinType::TWO_PAIR
            }
        },
        4 => ONE_PAIR,
        _ => HIGH_CARD,
    }
}
fn rule2(cards: Vec<CamelCard>) -> CamelCardWinType {
    let mut label_count = cards.iter()
        .fold(
            HashMap::new(),
            |mut acc, card| {
                let count = acc.entry(card.label).or_insert(0);
                *count += 1;
                acc
            });
    let joker_count = *label_count.entry('J').or_default();
    if joker_count == 0 {
        return rule1(cards);
    }
    if joker_count == 5 {
        return FIVE_OF_A_KIND;
    }
    label_count.remove(&'J');
    let values = label_count.values().collect::<HashSet<&u8>>();
    match label_count.len() {
        1 => FIVE_OF_A_KIND,
        2 => {
            return if values.contains(&3) || (joker_count == 2 && values.contains(&2)) {
                FOUR_OF_A_KIND
            } else {
                FULL_HOUSE
            };
        }
        3 => THREE_OF_A_KIND,
        _ => ONE_PAIR,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution(split_lines(include_str!("./sample1-7.txt")));
        assert_eq!(result, 5905);
    }
}