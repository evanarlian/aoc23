use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    fs,
};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Dir {
    Start,
    Left,
    Right,
    Up,
    Down,
}
impl Dir {
    fn dydx(&self) -> (i32, i32) {
        match self {
            Dir::Start => (0, 0),
            Dir::Left => (0, -1),
            Dir::Right => (0, 1),
            Dir::Up => (-1, 0),
            Dir::Down => (1, 0),
        }
    }
    fn allowed_dirs(&self) -> Vec<Dir> {
        // you can only turn left, right and keep going. Cannot go directly back
        match self {
            Dir::Start => vec![Dir::Left, Dir::Right, Dir::Up, Dir::Down],
            Dir::Left => vec![Dir::Left, Dir::Up, Dir::Down],
            Dir::Right => vec![Dir::Right, Dir::Up, Dir::Down],
            Dir::Up => vec![Dir::Left, Dir::Right, Dir::Up],
            Dir::Down => vec![Dir::Left, Dir::Right, Dir::Down],
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct State {
    dir: Dir,
    heatloss: i32,
    consecutive: i32,
    y: i32,
    x: i32,
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // order importance by heatloss only, reversed for min heap
        other.heatloss.partial_cmp(&self.heatloss)
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // order importance by heatloss only, reversed for min heap
        other.heatloss.cmp(&self.heatloss)
    }
}

fn parse(content: &String) -> Vec<Vec<i32>> {
    content
        .lines()
        .map(|line| {
            line.chars()
                .map(|num| num.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect()
}

fn is_safe_bound(h: i32, w: i32, y: i32, x: i32) -> bool {
    0 <= y && y < h && 0 <= x && x < w
}

fn dijkstra(heatmap: &Vec<Vec<i32>>) -> Option<i32> {
    // make visited mask
    let (h, w) = (heatmap.len() as i32, heatmap[0].len() as i32);
    let finished = (h - 1, w - 1);
    let mut visited = HashSet::<(Dir, i32, i32, i32)>::new();
    let mut pq = BinaryHeap::<State>::new();
    pq.push(State {
        dir: Dir::Start,
        heatloss: 0,
        consecutive: 0,
        y: 0,
        x: 0,
    });
    while !pq.is_empty() {
        let state = pq.pop().unwrap();
        // println!("{state:?}");
        if state.consecutive > 10 {
            continue;
        }
        // https://www.reddit.com/r/adventofcode/comments/18kr07r/comment/kdtho4d/?utm_source=share&utm_medium=web2x&context=3
        if visited.contains(&(state.dir, state.consecutive, state.y, state.x)) {
            continue;
        }
        visited.insert((state.dir, state.consecutive, state.y, state.x));
        let (y, x) = (state.y, state.x);
        if (y, x) == finished {
            return Some(state.heatloss);
        }
        // add neighbours
        for allowed_dir in state.dir.allowed_dirs() {
            let (dy, dx) = allowed_dir.dydx();
            let n_moves = if allowed_dir == state.dir && state.consecutive >= 4 {
                1
            } else {
                4
            };
            let new_consecutive = if allowed_dir == state.dir {
                state.consecutive + n_moves
            } else {
                n_moves
            };
            let (new_y, new_x) = (y + dy * n_moves, x + dx * n_moves);
            if !is_safe_bound(h, w, new_y, new_x) {
                continue;
            }
            let mut extra_heatloss = 0;
            for i in 1..=n_moves {
                extra_heatloss += heatmap[(y + i * dy) as usize][(x + i * dx) as usize];
            }
            let new_state = State {
                dir: allowed_dir,
                heatloss: state.heatloss + extra_heatloss,
                consecutive: new_consecutive,
                y: new_y,
                x: new_x,
            };
            pq.push(new_state);
        }
    }
    None
}

fn solve(content: &String) -> i32 {
    let heatmap = parse(content);
    dijkstra(&heatmap).expect("Dijkstra is returning None")
}

fn main() {
    let content = fs::read_to_string("inputs/day17.txt").expect("input for day 17 is missing");
    let result = solve(&content);
    println!("day 17 part 2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let content = String::from(
            "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533",
        );
        let result = solve(&content);
        assert_eq!(result, 94);
    }

    #[test]
    fn test2() {
        let content = String::from(
            "111111111111
999999999991
999999999991
999999999991
999999999991",
        );
        let result = solve(&content);
        assert_eq!(result, 71);
    }

    #[test]
    fn test3() {
        let content = String::from(
            "11111
99991
99991
99991
99991",
        );
        let result = solve(&content);
        assert_eq!(result, 8);
    }
}
