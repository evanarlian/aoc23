use std::fs;

enum Dir {
    Left,
    Right,
    Up,
    Down,
}
impl Dir {
    fn dydx(&self) -> (i32, i32) {
        match self {
            Dir::Left => (0, -1),
            Dir::Right => (0, 1),
            Dir::Up => (-1, 0),
            Dir::Down => (1, 0),
        }
    }
}

struct Command {
    dir: Dir,
    length: i32,
}

fn parse(content: &String) -> Vec<Command> {
    let mut commands = vec![];
    for line in content.lines() {
        let mut line_iter = line.split_ascii_whitespace().into_iter();
        let dir = line_iter.next().unwrap();
        let length = line_iter.next().unwrap();
        commands.push(Command {
            dir: match dir {
                "L" => Dir::Left,
                "R" => Dir::Right,
                "U" => Dir::Up,
                "D" => Dir::Down,
                other => unreachable!("bad input: {other}"),
            },
            length: length.parse().unwrap(),
        })
    }
    commands
}

fn polygon_area_shoelace(commands: &Vec<Command>) -> i32 {
    // https://en.wikipedia.org/wiki/Shoelace_formula
    let mut area = 0;
    let (mut y, mut x) = (0, 0);
    for cmd in commands {
        let (dy, dx) = cmd.dir.dydx();
        let new_y = y + dy * cmd.length;
        let new_x = x + dx * cmd.length;
        area += (y * new_x) - (x * new_y);
        (y, x) = (new_y, new_x);
    }
    area.abs() / 2
}

fn count_boundary(commands: &Vec<Command>) -> i32 {
    // boundary points are just the sum of movements
    // (assuming the polygon connects)
    commands.iter().map(|cmd| cmd.length).sum()
}

fn solve(content: &String) -> i32 {
    let commands = parse(content);
    // i don't want to use floodfill
    // pick's theorem: area = interior + boundary/2 - 1
    // polygon area can be obtained using shoelace method (polygon area != area asked for day 18)
    // need to know boundary + interior to know the asked area
    let polygon_area = polygon_area_shoelace(&commands);
    let boundary = count_boundary(&commands);
    let interior = polygon_area - boundary / 2 + 1;
    let points_enclosed = boundary + interior;
    points_enclosed
}

fn main() {
    let content = fs::read_to_string("inputs/day18.txt").expect("input for day 18 is missing");
    let result = solve(&content);
    println!("day 18 part 1: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let content = String::from(
            "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)",
        );
        let result = solve(&content);
        assert_eq!(result, 62);
    }

    #[test]
    fn test_easy1() {
        let content = String::from(
            "R 2 (#70c710)
D 1 (#0dc571)
L 2 (#5713f0)
U 1 (#d2c081)",
        );
        let result = solve(&content);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_easy2() {
        let content = String::from(
            "R 2 (#70c710)
D 1 (#0dc571)
L 1 (#0dc571)
D 1 (#0dc571)
L 1 (#0dc571)
U 2 (#d2c081)",
        );
        let result = solve(&content);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_easy3() {
        let content = String::from(
            "R 3 (#70c710)
D 1 (#0dc571)
L 2 (#0dc571)
D 1 (#0dc571)
L 1 (#0dc571)
U 2 (#d2c081)",
        );
        let result = solve(&content);
        assert_eq!(result, 10);
    }
}
