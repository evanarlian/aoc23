use std::{cmp, collections::HashSet, fs};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Block {
    Forest,
    Path,
    Up,
    Down,
    Left,
    Right,
}

impl Block {
    fn dir(&self) -> Vec<(i32, i32)> {
        const UP: (i32, i32) = (-1, 0);
        const DOWN: (i32, i32) = (1, 0);
        const LEFT: (i32, i32) = (0, -1);
        const RIGHT: (i32, i32) = (0, 1);
        match self {
            Block::Forest => vec![],
            Block::Path => vec![UP, DOWN, LEFT, RIGHT],
            Block::Up => vec![UP],
            Block::Down => vec![DOWN],
            Block::Left => vec![LEFT],
            Block::Right => vec![RIGHT],
        }
    }
}

fn parse(content: &String) -> Vec<Vec<Block>> {
    content
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '#' => Block::Forest,
                    '.' => Block::Path,
                    '^' => Block::Up,
                    'v' => Block::Down,
                    '<' => Block::Left,
                    '>' => Block::Right,
                    other => unreachable!("bad input: {other}"),
                })
                .collect()
        })
        .collect()
}

fn walk(
    map: &Vec<Vec<Block>>,
    tgt_coord: (i32, i32),
    coord: (i32, i32),
    depth: i32,
    visited: &mut HashSet<(i32, i32)>,
) -> Option<i32> {
    let (h, w) = (map.len() as i32, map[0].len() as i32);
    let (y, x) = coord;
    if y < 0 || y >= h || x < 0 || x >= w {
        return None;
    }
    let curr_block = map[y as usize][x as usize];
    if curr_block == Block::Forest {
        return None;
    }
    if tgt_coord == coord {
        return Some(depth);
    }
    if visited.contains(&coord) {
        return None;
    }
    visited.insert(coord);
    let dirs = curr_block.dir();
    let mut longest = None;
    for (dy, dx) in dirs {
        let (new_y, new_x) = (y + dy, x + dx);
        if let Some(from_below) = walk(map, tgt_coord, (new_y, new_x), depth + 1, visited) {
            longest = match longest {
                Some(longest) => Some(cmp::max(longest, from_below)),
                None => Some(from_below),
            };
        }
    }
    visited.remove(&coord);
    longest
}

fn solve(content: &String) -> i32 {
    let map = parse(content);
    let (h, w) = (map.len() as i32, map[0].len() as i32);
    let (tgt_y, tgt_x) = (h - 1, w - 2);
    let (start_y, start_x) = (0, 1);
    let mut visited = HashSet::new();
    let longest = walk(&map, (tgt_y, tgt_x), (start_y, start_x), 0, &mut visited);
    longest.unwrap()
}

fn main() {
    let content = fs::read_to_string("inputs/day23.txt").expect("input for day 21 is missing");
    let result = solve(&content);
    println!("day 23 part 1 {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let content = String::from(
            "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#",
        );
        let result = solve(&content);
        assert_eq!(result, 94);
    }
}
