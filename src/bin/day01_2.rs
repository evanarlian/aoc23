use std::{collections::HashMap, fs, path::Path};

fn find_from_left(line: &str, mapping: &HashMap<String, i32>) -> i32 {
    let mut earliest_index = 9999999;
    let mut earliest_number = -1;

    // reverse everything
    for v in mapping.keys() {
        if let Some(index) = line.find(v) {
            if index < earliest_index {
                earliest_index = index;
                earliest_number = mapping[v];
            }
        };
    }
    return earliest_number;
}

fn find_from_right(line: &str, mapping: &HashMap<String, i32>) -> i32 {
    // reverse everything
    let line: String = line.chars().rev().collect();
    let mapping: HashMap<String, i32> = mapping
        .iter()
        .map(|(s, i)| (s.chars().rev().collect(), i.clone()))
        .collect();

    let mut earliest_index = 9999999;
    let mut earliest_number = -1;

    for v in mapping.keys() {
        if let Some(index) = line.find(v) {
            if index < earliest_index {
                earliest_index = index;
                earliest_number = mapping[v];
            }
        };
    }
    return earliest_number;
}

fn solve(lines: &Vec<&str>) -> i32 {
    // own everything because brain too small rn
    let lines: Vec<String> = lines.iter().map(|&line| line.to_owned()).collect();
    let mapping: HashMap<String, i32> = HashMap::from([
        (String::from("one"), 1),
        (String::from("two"), 2),
        (String::from("three"), 3),
        (String::from("four"), 4),
        (String::from("five"), 5),
        (String::from("six"), 6),
        (String::from("seven"), 7),
        (String::from("eight"), 8),
        (String::from("nine"), 9),
        (String::from("0"), 0),
        (String::from("1"), 1),
        (String::from("2"), 2),
        (String::from("3"), 3),
        (String::from("4"), 4),
        (String::from("5"), 5),
        (String::from("6"), 6),
        (String::from("7"), 7),
        (String::from("8"), 8),
        (String::from("9"), 9),
    ]);
    let mut total = 0;
    for line in lines {
        let left = find_from_left(&line, &mapping);
        let right = find_from_right(&line, &mapping);
        total += left * 10 + right;
    }
    return total;
}

fn main() {
    let filepath = Path::new("inputs/day01.txt");
    let content = fs::read_to_string(filepath).expect("day 1 file not found");
    let lines = content.split_whitespace().collect();
    let result = solve(&lines);
    println!("day 1 part 2: {}", result);
}

mod test {
    use super::*;
    #[test]
    fn test() {
        let input = vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];
        assert_eq!(solve(&input), 281);
    }
}
