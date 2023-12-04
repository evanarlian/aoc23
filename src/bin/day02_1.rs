use std::{fs, path::Path};

fn solve(games: String, red: i32, green: i32, blue: i32) -> i32 {
    // parse games:
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    let mut total = 0;
    for game in games.lines() {
        let (game_id, groups) = game.split_once(":").unwrap();
        let game_id = game_id.split_once(" ").unwrap().1.parse::<i32>().unwrap();
        let mut under_constraint = true;
        for group in groups.split(";") {
            for num_color in group.split(",") {
                let (num, color) = num_color.trim().split_once(" ").unwrap();
                let num = num.parse::<i32>().unwrap();
                let is_possible = match color {
                    "red" => num <= red,
                    "green" => num <= green,
                    "blue" => num <= blue,
                    _ => {
                        println!("|{}|", color);
                        false
                    }
                };
                if !is_possible {
                    under_constraint = false
                }
            }
        }
        if under_constraint {
            total += game_id;
        }
    }
    return total;
}

fn main() {
    let input_path = Path::new("inputs/day02_1.txt");
    let content = fs::read_to_string(input_path).expect("input for day 2 part 1 is missing");
    let result = solve(content, 12, 13, 14);
    println!("day 2 part 1: {}", result);
}

mod tests {}
