use std::fs;

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
        let blocks = field
            .chars()
            .map(|c| match c {
                '.' => Block::Opr,
                '#' => Block::Dmg,
                '?' => Block::Unk,
                _ => unreachable!(),
            })
            .collect();
        let truths = truth.split(",").map(|num| num.parse().unwrap()).collect();
        parsed.push((blocks, truths));
    }
    return parsed;
}

fn find_combinations(
    blocks: &Vec<Block>,
    truths: &Vec<i32>,
    buffer: &mut Vec<Block>,
    level: usize,
    start_at: usize,
) -> i32 {
    if level == truths.len() {
        // this is the position when you can fit everything nicely
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
        if &damaged_groups == truths {
            return 1;
        } else {
            return 0;
        }
    }
    // the of this function
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
        // recursive backtracking, the new starting index is one +1 from the last painted block, +1 for gap
        total_so_far += find_combinations(blocks, truths, buffer, level + 1, i + curr_truth + 1);
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
        let temp = find_combinations(blocks, truths, &mut buffer, 0, 0);
        // println!("{}", temp);
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
    fn test() {
        let content = String::from(
            "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
        );
        let result = solve(&content);
        assert_eq!(result, 21);
    }
}
