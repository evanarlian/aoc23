use std::{cmp, fs, path::Path};

use regex::Regex;

fn solve(games: String) -> i32 {
    let mut total = 0;
    let re = Regex::new(r"(\d+) (red|green|blue)").unwrap();
    for game in games.lines() {
        let groups = game.split_once(":").unwrap().1.trim();
        let (mut red, mut green, mut blue) = (0, 0, 0);
        for item in re.captures_iter(groups) {
            let (_, [count, color]) = item.extract();
            // println!("{} {}", count, color);
            let count = count.parse::<i32>().unwrap();
            match color {
                "red" => red = cmp::max(red, count),
                "green" => green = cmp::max(green, count),
                "blue" => blue = cmp::max(blue, count),
                _ => {}
            }
        }
        // println!("{} {} {}", red, green, blue);
        total += red * green * blue;
    }

    return total;
}

fn main() {
    let content = fs::read_to_string(Path::new("inputs/day02.txt"))
        .expect("input for day 2 is missing");
    let result = solve(content);
    println!("day 2 part 2: {}", result);
}

mod tests {
    use super::*;
    #[test]
    fn test() {
        let games = String::from(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        let result = solve(games);
        assert_eq!(result, 2286);
    }
}
