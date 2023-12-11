use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    path::Path,
};

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
struct Coord {
    y: usize,
    x: usize,
}

fn parse(content: &String) -> (Vec<Vec<char>>, Coord) {
    let mut pipes = content
        .lines()
        .map(|row| row.chars().collect())
        .collect::<Vec<Vec<char>>>();
    // find S
    let mut coord = Coord { y: 0, x: 0 };
    'outer: for (i, row) in pipes.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == 'S' {
                coord = Coord { y: i, x: j };
                break 'outer;
            }
        }
    }
    // replace S
    let ups = ['|', 'F', '7']; //
    let downs = ['|', 'L', 'J'];
    let lefts = ['-', 'F', 'L'];
    let rights = ['-', '7', 'J'];
    let Coord { y, x } = coord;
    if y as i32 - 1 >= 0
        && ups.contains(&pipes[y - 1][x])
        && y + 1 < pipes.len()
        && downs.contains(&pipes[y + 1][x])
    {
        pipes[y][x] = '|'
    } else if x as i32 - 1 > 0
        && lefts.contains(&pipes[y][x - 1])
        && x + 1 < pipes[0].len()
        && rights.contains(&pipes[y][x + 1])
    {
        pipes[y][x] = '-'
    } else if y + 1 < pipes.len()
        && downs.contains(&pipes[y + 1][x])
        && x + 1 < pipes[0].len()
        && rights.contains(&pipes[y][x + 1])
    {
        pipes[y][x] = 'F'
    } else if y + 1 < pipes.len()
        && downs.contains(&pipes[y + 1][x])
        && x as i32 - 1 > 0
        && lefts.contains(&pipes[y][x - 1])
    {
        pipes[y][x] = '7'
    } else if y as i32 - 1 >= 0
        && ups.contains(&pipes[y - 1][x])
        && x + 1 < pipes[0].len()
        && rights.contains(&pipes[y][x + 1])
    {
        pipes[y][x] = 'L'
    } else if y as i32 - 1 >= 0
        && ups.contains(&pipes[y - 1][x])
        && x as i32 - 1 > 0
        && lefts.contains(&pipes[y][x - 1])
    {
        pipes[y][x] = 'J'
    } else {
        unreachable!("sus coord during parsing: {:?}", coord);
    }
    return (pipes, coord);
}

fn make_debug_map_like(pipes: &Vec<Vec<char>>) -> Vec<Vec<i32>> {
    let (h, w) = (pipes.len(), pipes[0].len());
    return vec![vec![-1; w]; h];
}

fn debug_print(debug: &Vec<Vec<i32>>) {
    let biggest_ndigits = debug
        .iter()
        .flat_map(|row| {
            row.iter().map(|num| match num {
                -1 => 1,
                _ => num.to_string().len(),
            })
        })
        .max()
        .unwrap();
    for row in debug {
        for &num in row {
            let what = if num == -1 {
                "-".to_string()
            } else {
                num.to_string()
            };
            print!("{:width$}", what, width = biggest_ndigits + 1);
        }
        println!();
    }
    println!();
}

fn bfs(pipes: &Vec<Vec<char>>, starting_coord: Coord) -> i32 {
    let mut debug = make_debug_map_like(pipes);
    let mut visited = HashSet::<Coord>::new();
    let mut queue = VecDeque::<(Coord, i32)>::new();
    const UP: (i32, i32) = (-1, 0);
    const DOWN: (i32, i32) = (1, 0);
    const LEFT: (i32, i32) = (0, -1);
    const RIGHT: (i32, i32) = (0, 1);
    let allowed_dirs = HashMap::<char, HashSet<(i32, i32)>>::from([
        ('-', HashSet::from([LEFT, RIGHT])),
        ('|', HashSet::from([UP, DOWN])),
        ('F', HashSet::from([DOWN, RIGHT])),
        ('7', HashSet::from([DOWN, LEFT])),
        ('J', HashSet::from([UP, LEFT])),
        ('L', HashSet::from([UP, RIGHT])),
    ]);

    queue.push_back((starting_coord, 0));
    let mut farthest = -1;
    while !queue.is_empty() {
        let (coord, dist) = queue.pop_front().unwrap();
        visited.insert(coord);
        debug[coord.y][coord.x] = dist;
        farthest = dist; // in bfs it is guaranteed that the last in queue is the farthest

        // add the neighbour according to the pipe types
        for direction in [UP, DOWN, LEFT, RIGHT] {
            // check allowed direction
            let Coord { y, x } = coord; // destructure
            if !allowed_dirs[&pipes[y][x]].contains(&direction) {
                continue;
            }
            // bound checks
            let (dy, dx) = direction;
            let new_y = y as i32 + dy;
            let new_x = x as i32 + dx;
            if new_y < 0
                || new_y >= pipes.len() as i32
                || new_x < 0
                || new_x >= pipes[0].len() as i32
            {
                continue;
                // visit checks
            }
            let (new_y, new_x) = (new_y as usize, new_x as usize);
            let new_coord = Coord { y: new_y, x: new_x };
            if visited.contains(&new_coord) {
                continue;
            }
            // '.' checks
            if pipes[new_y][new_x] == '.' {
                continue;
            }
            // finally: can add neighbour
            queue.push_back((new_coord, dist + 1));
        }
    }
    // debug_print(&debug); // toggle for nice array output
    return farthest;
}

fn solve(content: &String) -> i32 {
    let (pipes, starting_coord) = parse(content);
    let farthest = bfs(&pipes, starting_coord);
    return farthest;
}

fn main() {
    let content =
        fs::read_to_string(Path::new("inputs/day10.txt")).expect("input for day 10 is missing");
    let result = solve(&content);
    println!("day 10 part 1: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::{parse, solve, Coord};

    #[test]
    fn test_location() {
        let content = String::from(
            "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
        );
        let (_, starting_coord) = parse(&content);
        assert_eq!(starting_coord, Coord { y: 2, x: 0 });
    }

    #[test]
    fn test1() {
        let content = String::from(
            "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
        );
        let result = solve(&content);
        assert_eq!(result, 8);
    }

    #[test]
    fn test2() {
        let content = String::from(
            ".....
.S-7.
.|.|.
.L-J.
.....",
        );
        let result = solve(&content);
        assert_eq!(result, 4);
    }

    #[test]
    fn test3() {
        let content = String::from(
            ".....
.S-7.
.|.|.
.L-J.
.....",
        );
        let (pipes, _) = parse(&content);
        assert_eq!(pipes[1][1], 'F');
    }
}
