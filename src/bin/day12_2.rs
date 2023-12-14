// complete rewrite of part 1, now using proper top down DP
// huge thanks to: https://www.reddit.com/r/adventofcode/comments/18hbbxe/2023_day_12python_stepbystep_tutorial_with_bonus/

use std::{collections::HashMap, fs, hash::BuildHasherDefault};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Block {
    Opr,
    Dmg,
    Unk,
}

struct Question {
    data: Vec<Block>,
    groups: Vec<usize>,
}

fn parse(content: &str, copies: usize) -> Vec<Question> {
    let mut questions = vec![];
    for line in content.lines() {
        let (raw_data, raw_groups) = line.split_once(" ").unwrap();
        let raw_data = raw_data
            .chars()
            .map(|c| match c {
                '.' => Block::Opr,
                '#' => Block::Dmg,
                '?' => Block::Unk,
                _ => unreachable!("impossible during parsing"),
            })
            .collect::<Vec<_>>();
        let raw_groups = raw_groups
            .split(",")
            .map(|num| num.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        // make copies
        let mut data = raw_data.clone();
        for _ in 0..copies - 1 {
            data.push(Block::Unk);
            data.extend(raw_data.clone());
        }
        let groups = vec![raw_groups; copies].concat();
        questions.push(Question { data, groups });
    }
    return questions;
}

fn skip<'a>(
    data: &'a [Block],
    groups: &'a [usize],
    memo: &mut HashMap<(&'a [Block], &'a [usize]), i64>,
) -> i64 {
    return calculate_combinations(&data[1..], groups, memo);
}

fn lay<'a>(
    data: &'a [Block],
    groups: &'a [usize],
    memo: &mut HashMap<(&'a [Block], &'a [usize]), i64>,
) -> i64 {
    // lay means we start planting the groups
    if groups.is_empty() {
        // we cannot lay anything if there is no more group
        return 0;
    }
    let curr_group = groups[0];
    if data.len() < curr_group {
        // we cannot lay if not enough space
        return 0;
    } else {
        if data[..curr_group].contains(&Block::Opr) {
            // can't lay because '.' underneath
            return 0;
        }
        if data.len() == curr_group {
            // special case at the very end
            return calculate_combinations(&data[curr_group..], &groups[1..], memo);
        } else {
            // because this is not the very end, the +1 after that must NOT be #
            // because if . we can fullfill current group, if ? we assume .
            if data[curr_group] == Block::Dmg {
                return 0;
            } else {
                return calculate_combinations(&data[curr_group + 1..], &groups[1..], memo);
            }
        }
    }
}

// idk man what is this lifetime thing??
fn calculate_combinations<'a>(
    data: &'a [Block],
    groups: &'a [usize],
    memo: &mut HashMap<(&'a [Block], &'a [usize]), i64>,
) -> i64 {
    // check memo first
    if let Some(&cached) = memo.get(&(data, groups)) {
        return cached;
    }

    if data.is_empty() && groups.is_empty() {
        // we reached valid finish state
        return 1;
    } else if data.is_empty() && !groups.is_empty() {
        // invalid state, prune early
        return 0;
    }

    // at this point, your data is not empty, but groups can be either
    // for example "......" () is valid
    // "...##.." (2,) is valid too
    // so group emptiness cannot be used for detecting validness
    let curr = data[0];
    let mut total_so_far = 0;
    match curr {
        Block::Opr => {
            // if we find '.', the only logical way is to skip
            total_so_far += skip(data, groups, memo)
        }
        Block::Dmg => {
            // if we find '#', we must lay, because if we skip '#' then it will be a stray '#'
            total_so_far += lay(data, groups, memo)
        }
        Block::Unk => {
            // we can choose to lay and skip
            total_so_far += skip(data, groups, memo);
            total_so_far += lay(data, groups, memo);
        }
    }
    memo.insert((data, groups), total_so_far);
    total_so_far
}

fn calculate_combinations_helper(question: &Question) -> i64 {
    let mut memo = HashMap::new();
    return calculate_combinations(&question.data, &question.groups, &mut memo);
}

fn solve(content: &String, copies: usize) -> i64 {
    let questions = parse(content, copies);
    questions
        .iter()
        .map(|q| calculate_combinations_helper(q))
        .sum()
}

fn main() {
    let content = fs::read_to_string("inputs/day12.txt").expect("input for day 12 is missing");
    let result = solve(&content, 5);
    println!("day 12 part 2: {}", result);
}
#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_easy() {
        let content = String::from(
            "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
        );
        let result = solve(&content, 1);
        assert_eq!(result, 21);
    }

    #[test]
    fn test_hard() {
        let content = String::from(
            "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
        );
        let result = solve(&content, 5);
        assert_eq!(result, 525152);
    }
}
