use std::{collections::HashSet, fs, path::Path};

fn solve(cards: String) -> i32 {
    let mut total = 0;
    for line in cards.lines() {
        let (winning, yours) = line.split_once(":").unwrap().1.split_once("|").unwrap();
        let winning_nums = winning
            .trim()
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<HashSet<i32>>();
        let your_nums = yours
            .trim()
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<HashSet<i32>>();
        let intersection_length = winning_nums.intersection(&your_nums).count();
        if intersection_length == 0 {
            continue;
        }
        total += 2i32.pow(intersection_length as u32 - 1);
    }
    return total;
}

fn main() {
    let content = fs::read_to_string(Path::new("inputs/day04.txt"))
        .expect("input for day 4 is missing");
    let result = solve(content);
    println!("day 4 part 1: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let content = String::from(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        let result = solve(content);
        assert_eq!(result, 13);
    }
}
