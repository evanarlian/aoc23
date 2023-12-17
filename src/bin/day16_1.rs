use std::{collections::HashSet, fs};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Light {
    Left,
    Right,
    Up,
    Down,
}

impl Light {
    fn dir(&self) -> (i32, i32) {
        match self {
            Light::Left => (0, -1),
            Light::Right => (0, 1),
            Light::Up => (-1, 0),
            Light::Down => (1, 0),
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Tile {
    LeftRight,
    UpDown,
    Positive,
    Negative,
    Empty,
}

fn parse(content: &String) -> Vec<Vec<Tile>> {
    content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '-' => Tile::LeftRight,
                    '|' => Tile::UpDown,
                    '/' => Tile::Positive,
                    '\\' => Tile::Negative,
                    '.' => Tile::Empty,
                    other => unreachable!("bad input: {other}"),
                })
                .collect()
        })
        .collect()
}

fn trace_light(
    tiles: &Vec<Vec<Tile>>,
    energy: &mut Vec<Vec<bool>>,
    visited: &mut HashSet<(Light, Tile, usize, usize)>,
    light: Light,
    y: i32,
    x: i32,
) {
    // bound check is always painful in rust
    let (h, w) = (tiles.len() as i32, tiles[0].len() as i32);
    if y < 0 || y >= h || x < 0 || x >= w {
        return;
    }
    // energize current tile if haven't visited
    let (y, x) = (y as usize, x as usize);
    energy[y][x] = true;
    let curr_tile = tiles[y][x];
    if visited.contains(&(light, curr_tile, y, x)) {
        return;
    }
    visited.insert((light, curr_tile, y, x));
    let (y, x) = (y as i32, x as i32);
    // encode just the directions to take
    let new_lights = match (light, curr_tile) {
        (Light::Left, Tile::LeftRight) => vec![Light::Left],
        (Light::Left, Tile::UpDown) => vec![Light::Up, Light::Down],
        (Light::Left, Tile::Positive) => vec![Light::Down],
        (Light::Left, Tile::Negative) => vec![Light::Up],
        (Light::Right, Tile::LeftRight) => vec![Light::Right],
        (Light::Right, Tile::UpDown) => vec![Light::Up, Light::Down],
        (Light::Right, Tile::Positive) => vec![Light::Up],
        (Light::Right, Tile::Negative) => vec![Light::Down],
        (Light::Up, Tile::LeftRight) => vec![Light::Left, Light::Right],
        (Light::Up, Tile::UpDown) => vec![Light::Up],
        (Light::Up, Tile::Positive) => vec![Light::Right],
        (Light::Up, Tile::Negative) => vec![Light::Left],
        (Light::Down, Tile::LeftRight) => vec![Light::Left, Light::Right],
        (Light::Down, Tile::UpDown) => vec![Light::Down],
        (Light::Down, Tile::Positive) => vec![Light::Left],
        (Light::Down, Tile::Negative) => vec![Light::Right],
        (Light::Left, Tile::Empty) => vec![Light::Left],
        (Light::Right, Tile::Empty) => vec![Light::Right],
        (Light::Up, Tile::Empty) => vec![Light::Up],
        (Light::Down, Tile::Empty) => vec![Light::Down],
    };
    for new_light in new_lights {
        let (dy, dx) = new_light.dir();
        trace_light(tiles, energy, visited, new_light, y + dy, x + dx);
    }
}

fn debugmap(energy: &Vec<Vec<bool>>) {
    let mut temp = String::new();
    for row in energy {
        for is_energized in row {
            temp.push(match is_energized {
                true => '#',
                false => '.',
            })
        }
        temp.push('\n');
    }
    println!("{temp}");
}

fn solve(content: &String) -> i32 {
    let tiles = parse(content);
    let (h, w) = (tiles.len(), tiles[0].len());
    let mut energy = vec![vec![false; w]; h];
    let mut visited = HashSet::new();
    trace_light(&tiles, &mut energy, &mut visited, Light::Right, 0, 0);
    energy
        .iter()
        .flat_map(|row| row.iter())
        .map(|&state| state as i32)
        .sum()
}
fn main() {
    let content = fs::read_to_string("inputs/day16.txt").expect("input for day 16 is missing");
    let result = solve(&content);
    println!("day 16 part 1: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let content = String::from(
            r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....",
        );
        let result = solve(&content);
        assert_eq!(result, 46);
    }
}
