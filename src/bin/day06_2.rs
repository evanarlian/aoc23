use std::{fs, path::Path};

#[derive(Debug)]
struct Game {
    time: i64,
    dist: i64,
}

fn parse(content: &String) -> Game {
    let mut content_iter = content.lines();
    let time = content_iter
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<i64>()
        .unwrap();
    let dist = content_iter
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<i64>()
        .unwrap();
    Game { time, dist }
}

fn calculate_number_of_ways(game: Game) -> i64 {
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

fn solve(content: String) -> i64 {
    let game = parse(&content);
    calculate_number_of_ways(game)
}

fn main() {
    let content = fs::read_to_string(Path::new("inputs/day06.txt"))
        .expect("input for day 6 is missing");
    let result = solve(content);
    println!("day 6 part 2: {}", result);
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
        assert_eq!(result, 71503);
    }
}
