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

fn determine_main_loop(pipes: &Vec<Vec<char>>, starting_coord: Coord) -> Vec<Vec<char>> {
    // this func is similar to the previous one, but this will transfer the main pipes to new vectors
    let mut main_loop = vec![vec!['.'; pipes[0].len()]; pipes.len()];
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
    while !queue.is_empty() {
        let (coord, dist) = queue.pop_front().unwrap();
        visited.insert(coord);
        let Coord { y, x } = coord;
        main_loop[y][x] = pipes[y][x];

        // add the neighbour according to the pipe types
        for direction in [UP, DOWN, LEFT, RIGHT] {
            // check allowed direction
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
            }
            let (new_y, new_x) = (new_y as usize, new_x as usize);
            let new_coord = Coord { y: new_y, x: new_x };
            // visit checks
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
    return main_loop;
}

fn poly_raycast_coloring(grid: &Vec<Vec<char>>) -> i32 {
    // super useful: https://en.wikipedia.org/wiki/Point_in_polygon
    let mut inside = 0;
    for line in grid {
        let mut prev = '.'; // assume we extrude the grid area 1 unit to the left
        let mut first_wall = '.'; // temp
        let mut is_inside = false;
        for &curr in line {
            if curr == '.' {
                // currently we are at the field
                if is_inside {
                    inside += 1;
                    // print!("T");
                } else {
                    // print!("-");
                }
            } else {
                // print!("-");
                // currently we are at the wall
                if prev == '.' || prev == '7' || prev == 'J' || prev == '|' {
                    // many ways to start a new wall, just by looking at the prev
                    first_wall = curr;
                }
                // now we are looking at how the wall ends
                if curr == '|' {
                    is_inside = !is_inside;
                } else if (first_wall, curr) == ('L', '7') || (first_wall, curr) == ('F', 'J') {
                    // these 2 configuration will toggle
                    is_inside = !is_inside;
                }
            }
            prev = curr;
        }
        // println!();
    }
    return inside;
}

fn solve(content: &String) -> i32 {
    let (pipes, starting_coord) = parse(content);
    let main_loop = determine_main_loop(&pipes, starting_coord);
    let area_inside = poly_raycast_coloring(&main_loop);
    return area_inside;
}

fn main() {
    let content =
        fs::read_to_string(Path::new("inputs/day10.txt")).expect("input for day 10 is missing");
    let result = solve(&content);
    println!("day 10 part 2: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::{determine_main_loop, parse, solve};

    #[test]
    fn test_main_loop() {
        let content = String::from(
            ".S7..
.LJ..
.F--7
.L--J",
        );
        let correct = String::from(
            ".F7..
.LJ..
.....
.....",
        )
        .lines()
        .map(|row| row.chars().collect())
        .collect::<Vec<Vec<_>>>();
        let (pipes, starting_coord) = parse(&content);
        let result = determine_main_loop(&pipes, starting_coord);
        assert_eq!(result, correct);
    }

    #[test]
    fn test1() {
        let content = String::from(
            "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
        );
        let result = solve(&content);
        assert_eq!(result, 4);
    }

    #[test]
    fn test2() {
        let content = String::from(
            ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",
        );
        let result = solve(&content);
        assert_eq!(result, 8);
    }

    #[test]
    fn test3() {
        let content = String::from(
            "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
        );
        let result = solve(&content);
        assert_eq!(result, 10);
    }
}
