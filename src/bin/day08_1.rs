use std::{collections::HashMap, fs, path::Path};

fn solve(content: &String) -> i32 {
    // parse
    let (steps, directions) = content.split_once("\n\n").unwrap();
    let steps = steps.chars().collect::<Vec<_>>();
    let directions: HashMap<String, (String, String)> = directions
        .lines()
        .map(|x| {
            (
                (&x[0..3]).to_string(),
                ((&x[7..10]).to_string(), (&x[12..15]).to_string()),
            )
        })
        .collect();
    // do walking simulation
    let mut curr = &String::from("AAA");
    let finish = &String::from("ZZZ");
    let mut counter = 0;
    for step in steps.into_iter().cycle() {
        if curr == finish {
            break;
        }
        counter += 1;
        curr = match step {
            'L' => &directions[curr].0,
            'R' => &directions[curr].1,
            _ => unreachable!(),
        }
    }
    return counter;
}

fn main() {
    let content =
        fs::read_to_string(Path::new("inputs/day08.txt")).expect("input for day 8 is missing.");
    let result = solve(&content);
    println!("day 8 part 1: {}", result);
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test1() {
        let content = String::from(
            "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
        );
        let result = solve(&content);
        assert_eq!(result, 2);
    }
    #[test]
    fn test2() {
        let content = String::from(
            "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
        );
        let result = solve(&content);
        assert_eq!(result, 6);
    }
}
