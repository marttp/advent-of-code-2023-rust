use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub fn split_lines(input: &str) -> Vec<&str> {
    input.split("\n").collect()
}

pub fn total_winnings(hands: Vec<Hands>) -> u64 {
    // Max Heap by default
    let mut leaderboard: BinaryHeap<Hands> = BinaryHeap::new();
    let cloned_hands = hands.clone();
    for hand in cloned_hands.into_iter() {
        leaderboard.push(hand);
    }
    let mut size = leaderboard.len();
    let mut result = 0_u64;
    while size > 0 {
        let hand = leaderboard.pop().unwrap();
        result += hand.bid as u64 * size as u64;
        size -= 1;
    }
    return result;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum CamelCardWinType {
    HIGH_CARD = 1,
    ONE_PAIR = 2,
    TWO_PAIR = 3,
    THREE_OF_A_KIND = 4,
    FULL_HOUSE = 5,
    FOUR_OF_A_KIND = 6,
    // FOUR_OF_A_KIND
    FIVE_OF_A_KIND = 7, // STRAIGHT_FLUSH
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct CamelCard {
    pub label: char,
    pub strength: u8,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Hands {
    pub cards: Vec<CamelCard>,
    pub bid: u32,
    pub win_type: CamelCardWinType,
}

impl Hands {
    pub fn new(cards: Vec<CamelCard>, bid: u32, win_type: CamelCardWinType) -> Self {
        Hands {
            cards,
            bid,
            win_type,
        }
    }
}


impl PartialOrd for Hands {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hands {
    fn cmp(&self, other: &Self) -> Ordering {
        self.win_type.cmp(&other.win_type)
            .then_with(|| self.cards[0].strength.cmp(&other.cards[0].strength))
            .then_with(|| self.cards[1].strength.cmp(&other.cards[1].strength))
            .then_with(|| self.cards[2].strength.cmp(&other.cards[2].strength))
            .then_with(|| self.cards[3].strength.cmp(&other.cards[3].strength))
            .then_with(|| self.cards[4].strength.cmp(&other.cards[4].strength))
            .then_with(|| self.bid.cmp(&other.bid))
    }
}
