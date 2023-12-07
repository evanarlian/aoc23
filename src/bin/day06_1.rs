use std::{fs, path::Path};

#[derive(Debug)]
struct Game {
    time: i32,
    dist: i32,
}

fn parse(content: &String) -> Vec<Game> {
    let mut content_iter = content.lines();
    let times = content_iter
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_ascii_whitespace()
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let distances = content_iter
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_ascii_whitespace()
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(time, dist)| Game { time, dist })
        .collect()
}

fn calculate_number_of_ways(game: Game) -> i32 {
    // i think looping from the middle is a good strat
    let mut wins = 0;
    for i in (0..(game.time / 2 + 1)).rev() {
        if i * (game.time - i) > game.dist {
            wins += 1;
        } else {
            break;
        }
    }
    match game.time % 2 == 0 {
        true => wins * 2 - 1,
        false => wins * 2,
    }
}

fn solve(content: String) -> i32 {
    let games = parse(&content);
    games
        .into_iter()
        .map(|game| calculate_number_of_ways(game))
        .product()
}

fn main() {
    let content = fs::read_to_string(Path::new("inputs/day06_1.txt"))
        .expect("input for day 6 part 1 is missing");
    let result = solve(content);
    println!("day 6 part 1: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let content = String::from(
            "Time:      7  15   30
Distance:  9  40  200",
        );
        let result = solve(content);
        assert_eq!(result, 288);
    }
}
