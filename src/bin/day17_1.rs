use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    fs,
};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
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
    fn allowed_dirs(&self) -> [Dir; 3] {
        // you can only turn left, right and keep going. Cannot go directly back
        match self {
            Dir::Left => [Dir::Left, Dir::Up, Dir::Down],
            Dir::Right => [Dir::Right, Dir::Up, Dir::Down],
            Dir::Up => [Dir::Left, Dir::Right, Dir::Up],
            Dir::Down => [Dir::Left, Dir::Right, Dir::Down],
        }
    }
}

#[derive(PartialEq, Eq)]
struct State {
    dir: Dir,
    heatloss: i32,
    consecutive: i32,
    y: i32,
    x: i32,
    paths: Vec<(i32, i32)>,
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

fn dijkstra(heatmap: &Vec<Vec<i32>>) -> Option<i32> {
    // make visited mask
    let (h, w) = (heatmap.len() as i32, heatmap[0].len() as i32);
    let finished = (h - 1, w - 1);
    let mut visited = HashSet::<(Dir, i32, i32, i32)>::new();
    let mut pq = BinaryHeap::<State>::new();
    // we are on the top left and the first direction can be EITHER down or right, does not matter
    pq.push(State {
        dir: Dir::Right,
        heatloss: -heatmap[0][0], // pre-ignore the first block heat
        consecutive: 0,           // pre-ignore the first step
        y: 0,
        x: 0,
        paths: vec![], // save paths chosen by dijkstra
    });
    while !pq.is_empty() {
        let state = pq.pop().unwrap();
        // bound check
        let (y, x) = (state.y, state.x);
        if y < 0 || y >= h || x < 0 || x >= w {
            continue;
        }
        // cannot move more than 3 times in a row
        if state.consecutive > 3 {
            continue;
        }
        // check visited, turns out you need to store consecutive direction and the direction too
        // https://www.reddit.com/r/adventofcode/comments/18kr07r/comment/kdtho4d/?utm_source=share&utm_medium=web2x&context=3
        if visited.contains(&(state.dir, state.consecutive, state.y, state.x)) {
            continue;
        }
        visited.insert((state.dir, state.consecutive, state.y, state.x));
        // add current block
        let curr_heatloss = state.heatloss + heatmap[y as usize][x as usize];
        let mut new_paths = state.paths;
        new_paths.push((y, x));
        // check finish
        if (y, x) == finished {
            // debug the chosen paths
            // let mut temp = vec![vec![false; w as usize]; h as usize];
            // for (cy, cx) in new_paths {
            //     temp[cy as usize][cx as usize] = true;
            // }
            return Some(curr_heatloss);
        }
        // add neighbors
        for allowed_dir in state.dir.allowed_dirs() {
            let (dy, dx) = allowed_dir.dydx();
            let new_consecutive = if allowed_dir == state.dir {
                state.consecutive + 1
            } else {
                1
            };
            let new_state = State {
                dir: allowed_dir,
                heatloss: curr_heatloss,
                consecutive: new_consecutive,
                y: y + dy,
                x: x + dx,
                paths: new_paths.clone(),
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
    println!("day 17 part 1: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
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
        assert_eq!(result, 102);
    }

    #[test]
    fn test_easy1() {
        // 119
        // 911
        // 991
        let content = String::from(
            "119
911
991",
        );
        let result = solve(&content);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_easy2() {
        let content = String::from(
            "14999
23111
99991",
        );
        let result = solve(&content);
        assert_eq!(result, 11);
    }
}
