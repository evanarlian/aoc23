use std::{fs, path::Path, vec};

fn parse(content: &String) -> Vec<Vec<i32>> {
    content
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn predict_leftmost(nums: &Vec<i32>) -> i32 {
    // idk man i felt that recursive algo is very natural
    if nums.iter().all(|&x| x == 0) {
        // this assumes that eventually vectors will be all 0 before running out of elements
        return 0;
    }
    let mut diff = vec![];
    for (a, b) in nums.iter().zip(nums.iter().skip(1)) {
        diff.push(b - a);
    }
    let predicted = predict_leftmost(&diff);
    return diff.first().unwrap() - predicted;
}

fn solve(content: &String) -> i32 {
    let nums_list = parse(content);
    nums_list
        .iter()
        .map(|nums| nums.first().unwrap() - predict_leftmost(nums))
        .sum()
}

fn main() {
    let content =
        fs::read_to_string(Path::new("inputs/day09.txt")).expect("input for day 9 is missing");
    let result = solve(&content);
    println!("day 9 part 2: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test() {
        let content = String::from(
            "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        );
        let result = solve(&content);
        assert_eq!(result, 2);
    }
}
