use std::{fs, path::Path};

fn find_first(s: &str) -> i32 {
    for c in s.chars() {
        if c >= '0' && c <= '9' {
            return c.to_digit(10).unwrap() as i32;
        }
    }
    return -1;
}

fn find_last(s: &str) -> i32 {
    for c in s.chars().rev() {
        if c >= '0' && c <= '9' {
            return c.to_digit(10).unwrap() as i32;
        }
    }
    return -1;
}

fn calculate_sum_calibrations(lines: Vec<&str>) -> i32 {
    let mut total = 0;
    for line in lines {
        let first = find_first(line);
        let second = find_last(line);
        total += first * 10 + second;
    }
    return total;
}

fn main() {
    let input_path = Path::new("inputs/day01.txt");
    let content = fs::read_to_string(input_path).expect("missing file for day 1");
    let splitted: Vec<&str> = content.split_whitespace().collect();
    let result = calculate_sum_calibrations(splitted);
    println!("day 1 part 1: {}", result);
}

mod tests {
    use super::*;

    #[test]
    fn test1() {
        let tc = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        assert_eq!(calculate_sum_calibrations(tc), 142);
    }
}
