use std::{collections::HashMap, fs};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Block {
    Opr,
    Dmg,
    Unk,
}

fn parse(content: &String) -> Vec<(Vec<Block>, Vec<i32>)> {
    let mut parsed = vec![];
    for line in content.lines() {
        let (field, truth) = line.split_once(" ").unwrap();
        // create base block and truths
        let blocks = field
            .chars()
            .map(|c| match c {
                '.' => Block::Opr,
                '#' => Block::Dmg,
                '?' => Block::Unk,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();
        let truths = truth
            .split(",")
            .map(|num| num.parse().unwrap())
            .collect::<Vec<_>>();
        // add the twist (repeated)
        let mut blocks5 = vec![];
        for i in 0..5 {
            blocks5.extend(blocks.clone());
            if i < 4 {
                blocks5.push(Block::Unk);
            }
        }
        let truths5 = [&truths[..]; 5].concat();
        parsed.push((blocks5, truths5));
        // parsed.push((blocks, truths));
    }
    return parsed;
}

fn early_group_check(buffer: &Vec<Block>, truths: &Vec<i32>, n_completed: usize) -> bool {
    // the idea:
    // 1. you must have at least n_completed blocks
    // example (n_completed=3)
    // truth: ###.##.##
    // yours: #######..
    //        ^^^^^^^ giant clump of single group but less than 3 groups
    // 2. the first (n_completed-1) must be equal, because it has set in stone
    // example (n_completed=3)
    // truth: ###.##.##
    // yours: ###.##.#.
    //        ^^^ ^^ it must have set in stone, you cannot change it, but the last one (#) might still be added
    // 3. the n_completed'th group must be equal or smaller, because in the future you can only add
    // example (n_completed=3)
    // truth: ###.##.##.
    // yours: ###.##.###
    //               ^^^ impossible to fix to match the top one

    // rule 0: assume true if has not completed anything
    if n_completed == 0 {
        return true;
    }
    let damaged_groups = buffer
        .iter()
        .copied()
        .group_by(|&k| k)
        .into_iter()
        .filter_map(|(item, group)| match item {
            Block::Dmg => Some(group.count() as i32),
            _ => None,
        })
        .collect::<Vec<_>>();
    // rule 1
    if damaged_groups.len() < n_completed {
        return false;
    }
    // rule 2
    for i in 0..n_completed - 1 {
        if damaged_groups[i] != truths[i] {
            return false;
        }
    }
    // rule 3
    if damaged_groups[n_completed - 1] > truths[n_completed - 1] {
        return false;
    }
    return true;
}

fn find_combinations(
    blocks: &Vec<Block>,
    truths: &Vec<i32>,
    buffer: &mut Vec<Block>,
    level: usize,
    start_at: usize,
    memo: &mut HashMap<(usize, usize), i32>,
) -> i32 {
    // when to backtrack
    let damaged_groups = buffer
        .iter()
        .copied()
        .group_by(|&k| k)
        .into_iter()
        .filter_map(|(item, group)| match item {
            Block::Dmg => Some(group.count() as i32),
            _ => None,
        })
        .collect::<Vec<_>>();
    let is_valid_early = early_group_check(buffer, truths, level);
    if level == truths.len() {
        // this is the final phase, so this is the most strict, groups must be all the same
        if &damaged_groups == truths {
            return 1;
        } else {
            return 0;
        }
    } else if !is_valid_early {
        return 0;
    }
    // at this point, so far is valid
    let mut total_so_far = 0;
    // for current truths, select according to level
    let curr_truth = truths[level] as usize;
    // find the next available position
    let mut i = start_at;
    let max_possible_index = blocks.len() - curr_truth;
    while i <= max_possible_index {
        // find the first operational block that is on your way (if any)
        if let Some(index) = blocks
            .iter()
            .skip(i)
            .take(curr_truth)
            .position(|&block| block == Block::Opr)
        {
            // instead of moving just once, move forward so that you move PAST the first operational block you find earlier
            i += index + 1;
            continue;
        }
        // at this point, you can start "painting"
        for j in i..i + curr_truth {
            buffer[j] = Block::Dmg;
        }
        // RECUR
        let from_below =
            find_combinations(blocks, truths, buffer, level + 1, i + curr_truth + 1, memo);
        memo.insert((level + 1, i + curr_truth + 1), from_below);
        total_so_far += from_below;
        // restore to the original "paint"
        for j in i..i + curr_truth {
            buffer[j] = blocks[j];
        }
        // move to the next spot
        i += 1;
    }
    return total_so_far;
}

fn solve(content: &String) -> i32 {
    let parsed = parse(content);
    let mut total = 0;
    for (blocks, truths) in parsed.iter() {
        // println!("{:?}", blocks);
        // println!("{:?}", truths);
        let mut buffer = blocks.iter().copied().collect::<Vec<_>>();
        // we introduce memoization that maps (ith-truths/level, position) to how many iterations under that
        let mut memo = HashMap::new();
        let temp = find_combinations(blocks, truths, &mut buffer, 0, 0, &mut memo);
        println!("{}", temp);
        total += temp;
    }
    return total;
}

fn main() {
    let content = fs::read_to_string("inputs/day12.txt").expect("input for day 12 is missing");
    let result = solve(&content);
    println!("day 12 part 1: {}", result);
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test1() {
        let content = String::from(
            "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
        );
        let result = solve(&content);
        assert_eq!(result, 525152);
    }

    // #[test]
    // fn test2() {
    //     let content = String::from("?#?#?#?#?#?#?#? 1,3,1,6");
    //     let result = solve(&content);
    //     assert_eq!(result, 1);
    // }

    // #[test]
    // fn test3() {
    //     let content = String::from("?###???????? 3,2,1");
    //     let result = solve(&content);
    //     assert_eq!(result, 506250);
    // }
}
