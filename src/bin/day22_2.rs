use std::{cmp::max, fs};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Brick {
    x0: i32,
    y0: i32,
    z0: i32,
    x1: i32,
    y1: i32,
    z1: i32,
}
impl Brick {
    fn is_xy_overlap(&self, other: &Self) -> bool {
        if self.x1 < other.x0 || other.x1 < self.x0 {
            return false;
        }
        if self.y1 < other.y0 || other.y1 < self.y0 {
            return false;
        }
        true
    }
    fn drop_z_to(&mut self, new_z0: i32) {
        let h = self.z1 - self.z0;
        self.z0 = new_z0;
        self.z1 = new_z0 + h;
    }
}

#[derive(Clone)]
struct Wall {
    bricks: Vec<Brick>,
}
impl Wall {
    fn print(&self) {
        for brick in &self.bricks {
            println!("{brick:?}");
        }
    }
    fn sort_z(&mut self) {
        self.bricks.sort_by_key(|brick| brick.z0);
    }
    fn find_lowest_possible(&self, i: usize, ignore_idx: Option<usize>) -> i32 {
        // drop i-th block, and see what's its new z0
        // iteratively checks blocks below curr selected overlaps or not, floor is guaranteed to overlap
        // note we only interact with roof LOWER than current
        let curr = &self.bricks[i];
        let mut lowest_possible = 1; // floor
        for j in (0..i).rev() {
            if let Some(ignore_idx) = ignore_idx {
                if j == ignore_idx {
                    continue;
                }
            }
            let lower = &self.bricks[j];
            if lower.z1 >= curr.z0 {
                continue;
            }
            if curr.is_xy_overlap(lower) {
                lowest_possible = max(lowest_possible, lower.z1 + 1);
            }
        }
        lowest_possible
    }
    fn drop_all(&mut self) {
        for i in 0..self.bricks.len() {
            let lowest_possible = self.find_lowest_possible(i, None);
            self.bricks[i].drop_z_to(lowest_possible);
        }
    }
    fn drop_all_ignore(&mut self, ignore_idx: usize) {
        for i in 0..self.bricks.len() {
            if i == ignore_idx {
                continue;
            }
            let lowest_possible = self.find_lowest_possible(i, Some(ignore_idx));
            self.bricks[i].drop_z_to(lowest_possible);
        }
    }
    fn count_differences(&self, other: &Self) -> i32 {
        self.bricks
            .iter()
            .zip(other.bricks.iter())
            .filter(|&(a, b)| a != b)
            .count() as i32
    }
}

fn parse(content: &String) -> Wall {
    let bricks = content
        .lines()
        .map(|line| {
            let (start, end) = line.split_once("~").unwrap();
            let s = start
                .split(",")
                .map(|num| num.parse().unwrap())
                .collect::<Vec<_>>();
            let (x0, y0, z0) = (s[0], s[1], s[2]);
            let e = end
                .split(",")
                .map(|num| num.parse().unwrap())
                .collect::<Vec<_>>();
            let (x1, y1, z1) = (e[0], e[1], e[2]);
            Brick {
                x0,
                y0,
                z0,
                x1,
                y1,
                z1,
            }
        })
        .collect();
    Wall { bricks }
}

fn solve(content: &String) -> i32 {
    let mut wall = parse(content);
    wall.sort_z();
    // wall.print();
    // println!();
    wall.drop_all();
    // wall.print();

    // VIRGIN SINGLE CORE
    // (0..wall.bricks.len())
    //     .into_iter()
    //     .map(|i| {
    //         let mut experiment = wall.clone();
    //         experiment.drop_all_ignore(i);
    //         experiment.count_differences(&wall)
    //     })
    //     .sum()

    // CHAD MULTI CORE RAYON
    (0..wall.bricks.len())
        .into_par_iter()
        .map(|i| {
            let mut experiment = wall.clone();
            experiment.drop_all_ignore(i);
            experiment.count_differences(&wall)
        })
        .sum()
}

fn main() {
    let content = fs::read_to_string("inputs/day22.txt").expect("input for day 22 is missing");
    let result = solve(&content);
    println!("day 22 part 2: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let content = String::from(
            "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9",
        );
        let result = solve(&content);
        assert_eq!(result, 7);
    }
}
