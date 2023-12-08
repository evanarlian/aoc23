use std::{collections::HashMap, fs, path::Path};

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: i32,
    cards_type: i32,       // this is computed
    cards_power: Vec<i32>, // this is computed
}
impl Hand {
    fn new(cards: String, bid: i32) -> Hand {
        let cards_type = Hand::compute_cards_type(&cards);
        let cards_power = Hand::compute_cards_power(&cards);
        Hand {
            cards,
            bid,
            cards_type,
            cards_power,
        }
    }
    fn compute_cards_power(cards: &String) -> Vec<i32> {
        // this is extremely ineffective
        let power_mapping: HashMap<char, i32> = HashMap::from([
            ('A', 12),
            ('K', 11),
            ('Q', 10),
            ('J', 9),
            ('T', 8),
            ('9', 7),
            ('8', 6),
            ('7', 5),
            ('6', 4),
            ('5', 3),
            ('4', 2),
            ('3', 1),
            ('2', 0),
        ]);
        cards.chars().map(|c| power_mapping[&c]).collect()
    }
    fn compute_cards_type(cards: &String) -> i32 {
        // cards_type list:
        // 5 of a kind -> 6
        // 4 of a kind -> 5
        // full house -> 4
        // 3 of a kind -> 3
        // 2 pairs -> 2
        // 1 pair -> 1
        // high card -> 0

        // make counter
        let mut counter: HashMap<char, i32> = HashMap::new();
        for card in cards.chars() {
            let count = counter.entry(card).or_insert(0);
            *count += 1;
        }

        // the trick is to compare the sorted counts
        let mut counts = counter.values().copied().collect::<Vec<_>>();
        counts.sort();
        // ahh rust cannot pattern match with vec![], so fallback to using if else
        if counts == vec![5] {
            // 5 of a kind
            6
        } else if counts == vec![1, 4] {
            // 4 of a kind
            5
        } else if counts == vec![2, 3] {
            // full house
            4
        } else if counts == vec![1, 1, 3] {
            // 3 of a kind
            3
        } else if counts == vec![1, 2, 2] {
            // 2 pairs
            2
        } else if counts == vec![1, 1, 1, 2] {
            // 1 pair
            1
        } else if counts == vec![1, 1, 1, 1, 1] {
            // high card
            0
        } else {
            unreachable!()
        }
    }
}

fn parse(content: &String) -> Vec<Hand> {
    content
        .lines()
        .map(|x| x.split_once(" ").unwrap())
        .map(|(cards, bid)| Hand::new(cards.to_string(), bid.parse().unwrap()))
        .collect()
}

fn solve(content: String) -> i32 {
    let mut hands = parse(&content);
    // rust tuple implements lexicographic comparison
    hands.sort_by_key(|hand| (hand.cards_type, hand.cards_power.clone())); // oh no clone here is super ineffective
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i as i32 + 1) * hand.bid)
        .sum()
}

fn main() {
    let content = fs::read_to_string(Path::new("inputs/day07_1.txt"))
        .expect("input for day 7 part 1 is missing.");
    let result = solve(content);
    println!("day 7 part 1: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let content = String::from(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        );
        let result = solve(content);
        assert_eq!(result, 6440);
    }
}
