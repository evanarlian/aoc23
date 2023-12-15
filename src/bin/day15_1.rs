use std::fs;

fn solve(content: &String) -> i32 {
    let parsed = content
        .split(",")
        .map(str::to_string)
        .collect::<Vec<String>>();
    let mut total_hash = 0;
    for seq in parsed {
        let mut curr_hash = 0;
        for c in seq.chars() {
            curr_hash = (curr_hash + c as i32) * 17 % 256;
        }
        total_hash += curr_hash;
    }
    total_hash
}

fn main() {
    let content = fs::read_to_string("inputs/day15.txt").expect("input for day 15 is missing");
    let result = solve(&content);
    println!("day 15 part 1: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let content = String::from("HASH");
        let result = solve(&content);
        assert_eq!(result, 52);
    }
}
