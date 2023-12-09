use std::{collections::HashMap, fs, path::Path};

fn simulate_walk(
    starting_node: &String,
    steps: &Vec<char>,
    directions: &HashMap<String, (String, String)>,
) -> i64 {
    // do walking simulation
    let mut curr = starting_node;
    let mut counter = 0;
    for step in steps.into_iter().cycle() {
        if curr.ends_with("Z") {
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

fn gcd(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a, b);
    while b != 0 {
        (a, b) = (b, a % b);
    }
    return a;
}

fn lcm(a: i64, b: i64) -> i64 {
    return a * b / gcd(a, b);
}

fn solve(content: &String) -> i64 {
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
    // calculate num walks for every starting position that ends with A
    let num_walks = directions
        .keys()
        .filter(|x| x.ends_with("A"))
        .map(|x| simulate_walk(x, &steps, &directions))
        .collect::<Vec<_>>();
    // then the answer is just LCM of all those number
    let lcm_all = num_walks
        .iter()
        .fold(1, |lcm_so_far, &x| lcm(lcm_so_far, x));
    return lcm_all;
}

fn main() {
    let content =
        fs::read_to_string(Path::new("inputs/day08.txt")).expect("input for day 8 is missing.");
    let result = solve(&content);
    println!("day 8 part 2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::{gcd, lcm, solve};
    #[test]
    fn test() {
        let content = String::from(
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        );
        let result = solve(&content);
        assert_eq!(result, 6);
    }
    #[test]
    fn test_gcd() {
        assert_eq!(gcd(8, 12), 4);
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(1, 3), 1);
        assert_eq!(gcd(20, 4), 4);
        assert_eq!(gcd(11, 11), 11);
    }
    #[test]
    fn test_lcm() {
        assert_eq!(lcm(8, 12), 24);
        assert_eq!(lcm(12, 8), 24);
        assert_eq!(lcm(1, 3), 3);
        assert_eq!(lcm(20, 4), 20);
        assert_eq!(lcm(11, 11), 11);
        assert_eq!(lcm(5, 7), 35);
    }
}
