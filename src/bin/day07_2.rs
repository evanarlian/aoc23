use std::{collections::HashMap, fs, path::Path};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Face {
    Jo, // Joker, not Jack. There is no Jack here
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    Q,
    K,
    A,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Tier {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

struct Hand {
    faces: Vec<Face>,
    bid: i32,
    tier: Tier,
}
impl Hand {
    fn new(cards: &str, bid: i32) -> Hand {
        let faces = Hand::compute_faces(cards);
        let tier = Hand::compute_tier(&faces);
        Hand { faces, bid, tier }
    }
    fn compute_faces(cards: &str) -> Vec<Face> {
        cards
            .chars()
            .map(|card| match card {
                'J' => Face::Jo,
                '2' => Face::N2,
                '3' => Face::N3,
                '4' => Face::N4,
                '5' => Face::N5,
                '6' => Face::N6,
                '7' => Face::N7,
                '8' => Face::N8,
                '9' => Face::N9,
                'T' => Face::T,
                'Q' => Face::Q,
                'K' => Face::K,
                'A' => Face::A,
                sus => {
                    println!("{}", sus);
                    unreachable!()
                }
            })
            .collect()
    }
    fn compute_tier(faces: &Vec<Face>) -> Tier {
        let mut counter: HashMap<Face, i32> = HashMap::new();
        for face in faces {
            let count = counter.entry(*face).or_insert(0);
            *count += 1;
        }
        let joker_count = counter.remove(&Face::Jo).unwrap_or(0);
        let mut counts = counter.values().copied().collect::<Vec<i32>>();
        counts.sort();
        // add joker back to the biggest count
        if counts.is_empty() {
            counts.push(joker_count);
        } else {
            *counts.last_mut().unwrap() += joker_count;
        }
        // check tier, rust cannot pattern match with vec![], so fallback to using if else
        if counts == vec![5] {
            Tier::FiveOfAKind
        } else if counts == vec![1, 4] {
            Tier::FourOfAKind
        } else if counts == vec![2, 3] {
            Tier::FullHouse
        } else if counts == vec![1, 1, 3] {
            Tier::ThreeOfAKind
        } else if counts == vec![1, 2, 2] {
            Tier::TwoPair
        } else if counts == vec![1, 1, 1, 2] {
            Tier::OnePair
        } else if counts == vec![1, 1, 1, 1, 1] {
            Tier::HighCard
        } else {
            unreachable!()
        }
    }
}

fn parse(content: &String) -> Vec<Hand> {
    content
        .lines()
        .map(|x| x.split_once(" ").unwrap())
        .map(|(cards, bid)| Hand::new(cards, bid.parse().unwrap()))
        .collect()
}

fn solve(content: String) -> i32 {
    let mut hands = parse(&content);
    // rust tuple implements lexicographic comparison
    hands.sort_by(|a, b| a.tier.cmp(&b.tier).then_with(|| a.faces.cmp(&b.faces)));
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i as i32 + 1) * hand.bid)
        .sum()
}

fn main() {
    let content = fs::read_to_string(Path::new("inputs/day07_2.txt"))
        .expect("input for day 7 part 2 is missing.");
    let result = solve(content);
    println!("day 7 part 2: {}", result);
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
        assert_eq!(result, 5905);
    }
}
