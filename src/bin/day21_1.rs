use std::{collections::HashSet, fs};

#[derive(Debug, PartialEq, Eq)]
enum Field {
    Garden,
    Rock,
}

fn parse(content: &String) -> (Vec<Vec<Field>>, (i32, i32)) {
    // find starting position and replace
    let mut area = content
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<_>>>();
    let (mut start_y, mut start_x) = (0, 0);
    for (i, line) in area.iter_mut().enumerate() {
        for (j, ch) in line.iter_mut().enumerate() {
            if *ch == 'S' {
                (start_y, start_x) = (i as i32, j as i32);
                *ch = '.'; // replace S with regular garden '.'
                break;
            }
        }
    }
    // convert to enums
    let area = area
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|ch| match ch {
                    '#' => Field::Rock,
                    '.' => Field::Garden,
                    other => unreachable!("bad input: {other}"),
                })
                .collect()
        })
        .collect();
    (area, (start_y, start_x))
}

fn mark(area: &Vec<Vec<Field>>, steps: i32, start_y: i32, start_x: i32) -> usize {
    // some vars
    const UP: (i32, i32) = (-1, 0);
    const DOWN: (i32, i32) = (1, 0);
    const LEFT: (i32, i32) = (0, -1);
    const RIGHT: (i32, i32) = (0, 1);
    let (h, w) = (area.len() as i32, area[0].len() as i32);
    // the idea: instead of using dfs, better to keep expanding out from the marked area
    let mut positions = HashSet::new();
    positions.insert((start_y, start_x));
    for _ in 0..steps {
        let mut new_pos = HashSet::new();
        for (y, x) in positions {
            for (dy, dx) in [UP, DOWN, LEFT, RIGHT] {
                let (new_y, new_x) = (y + dy, x + dx);
                if new_y < 0 || new_y >= h || new_x < 0 || new_x >= w {
                    continue;
                }
                if area[new_y as usize][new_x as usize] == Field::Rock {
                    continue;
                }
                new_pos.insert((new_y, new_x));
            }
        }
        positions = new_pos;
    }
    positions.len()
}

fn solve(content: &String, steps: i32) -> i32 {
    let (area, (start_y, start_x)) = parse(content);
    mark(&area, steps, start_y, start_x) as i32
}

fn main() {
    let content = fs::read_to_string("inputs/day21.txt").expect("input for day 21 is missing");
    let result = solve(&content, 64);
    println!("day 21 part 1: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let content = String::from(
            "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........",
        );
        let result = solve(&content, 6);
        assert_eq!(result, 16);
    }
}
