mod common;

use std::collections::{HashMap, HashSet};
use common::{split_lines, total_winnings, CamelCard, CamelCardWinType, Hands};
use crate::common::CamelCardWinType::{FIVE_OF_A_KIND, HIGH_CARD, ONE_PAIR};

const POSSIBLE_LABELS: [char; 13] = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
const CARD_STRENGTH: [u8; 13] = [14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2];

#[allow(dead_code)]
fn main() {
    let input = include_str!("./input-7.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> u64 {
    let input_rule1 = parse_input_rule1(input.clone());
    let result = total_winnings(input_rule1);
    return result;
}


fn parse_input_rule1(input: Vec<&str>) -> Vec<Hands> {
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
        return Hands::new(camel_cards.clone(), bid, rule1(camel_cards.clone()));
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
                CamelCardWinType::FOUR_OF_A_KIND
            } else {
                CamelCardWinType::FULL_HOUSE
            }
        },
        3 => {
            return if values.contains(&3) {
                CamelCardWinType::THREE_OF_A_KIND
            } else {
                CamelCardWinType::TWO_PAIR
            }
        },
        4 => ONE_PAIR,
        _ => HIGH_CARD,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution(split_lines(include_str!("./sample1-7.txt")));
        assert_eq!(result, 6440);
    }
}