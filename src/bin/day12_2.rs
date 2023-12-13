use std::{collections::HashMap, fs};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Block {
    Opr,
    Dmg,
    Unk,
}

fn parse(content: &String, copies: usize) -> Vec<(Vec<Block>, Vec<i64>)> {
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
        let mut blocks_copied = vec![];
        for i in 0..copies {
            blocks_copied.extend(blocks.clone());
            if i < copies - 1 {
                blocks_copied.push(Block::Unk);
            }
        }
        let truths_copied = vec![&truths[..]; copies].concat();
        parsed.push((blocks_copied, truths_copied));
    }
    return parsed;
}

fn early_group_check(buffer: &Vec<Block>, truths: &Vec<i64>, n_completed: usize) -> bool {
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
            Block::Dmg => Some(group.count() as i64),
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
    truths: &Vec<i64>,
    buffer: &mut Vec<Block>,
    level: usize,
    start_at: usize,
    memo: &mut HashMap<(usize, usize), i64>,
) -> i64 {
    // 012345678901234
    // ##??.#??##??.#? 2,1,2,1
    // ^^   ^  ^^   ^
    // when to backtrack
    let damaged_groups = buffer
        .iter()
        .copied()
        .group_by(|&k| k)
        .into_iter()
        .filter_map(|(item, group)| match item {
            Block::Dmg => Some(group.count() as i64),
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
    // check memo here?
    if memo.contains_key(&(level, start_at)) {
        return memo[&(level, start_at)];
    }
    while i <= max_possible_index {
        // println!("level:{level} ct:{curr_truth} i:{i} startat:{start_at}");
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
        total_so_far +=
            find_combinations(blocks, truths, buffer, level + 1, i + curr_truth + 1, memo);
        // restore to the original "paint"
        for j in i..i + curr_truth {
            buffer[j] = blocks[j];
        }
        // move to the next spot
        i += 1;
    }
    // memoize only meaningful score (nonzero)
    if total_so_far > 0 {
        memo.insert((level, start_at), total_so_far);
    }
    return total_so_far;
}

fn solve(content: &String, copies: usize) -> i64 {
    let parsed = parse(content, copies);
    let mut total = 0;
    for (i, (blocks, truths)) in parsed.iter().enumerate() {
        // println!("{:?}", blocks);
        // println!("{:?}", truths);
        let mut buffer = blocks.iter().copied().collect::<Vec<_>>();
        // we introduce memoization that maps (ith-truths/level, position) to how many iterations under that
        let mut memo = HashMap::new();
        let temp = find_combinations(blocks, truths, &mut buffer, 0, 0, &mut memo);
        println!("[{}] {}", i, temp);
        // for (k, v) in memo {
        //     println!("{k:?}\t{v}")
        // }
        total += temp;
    }
    return total;
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
    fn test1() {
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
    fn test2() {
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

    #[test]
    fn test_fast() {
        let content = String::from(
            "???.### 1,1,3
????.#...#... 4,1,1
????.######..#####. 1,6,5",
        );
        let result = solve(&content, 5);
        assert_eq!(result, 2517);
    }

    #[test]
    fn test_weird() {
        let content = String::from("?##?????? 2,1,1");
        let result = solve(&content, 2);
        assert_eq!(result, 60);
    }

    #[test]
    fn test_weird2() {
        // let content = String::from("??###??##?????.#? 10,1");
        // let result = solve(&content, 2);
        // ##??.#??##??.#? 2,1,2,1
        // ^^   ^  ^^   ^
        let content = String::from("##??.#? 2,1");
        let result = solve(&content, 2);
        assert_eq!(result, 1);
    }
}
