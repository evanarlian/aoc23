use std::{cmp, fs};

struct Cave {
    data: Vec<Vec<char>>,
}
impl Cave {
    fn print(&self) {
        for row in self.data.iter() {
            for c in row {
                print!("{c}");
            }
            println!();
        }
    }
    fn find_horizontal_mirror(&self) -> Option<usize> {
        let (nrows, ncols) = (self.data.len(), self.data[0].len());
        for mirror in 0..nrows - 1 {
            let mut curr_mirror_possible = true;
            let comparisons = cmp::min(mirror + 1, nrows - mirror - 1);
            for i in 0..comparisons {
                for j in 0..ncols {
                    if self.data[mirror - i][j] != self.data[mirror + i + 1][j] {
                        curr_mirror_possible = false;
                        break;
                    }
                }
                if !curr_mirror_possible {
                    break;
                }
            }
            if curr_mirror_possible {
                return Some(mirror + 1); // +1 to convert index to line count
            }
        }
        None
    }
    fn find_vertical_mirror(&self) -> Option<usize> {
        let (nrows, ncols) = (self.data.len(), self.data[0].len());
        for mirror in 0..ncols - 1 {
            let mut curr_mirror_possible = true;
            let comparisons = cmp::min(mirror + 1, ncols - mirror - 1);
            for j in 0..comparisons {
                for i in 0..nrows {
                    if self.data[i][mirror - j] != self.data[i][mirror + j + 1] {
                        curr_mirror_possible = false;
                        break;
                    }
                }
                if !curr_mirror_possible {
                    break;
                }
            }
            if curr_mirror_possible {
                return Some(mirror + 1); // +1 to convert index to line count
            }
        }
        None
    }
}

fn parse(content: &String) -> Vec<Cave> {
    let mut caves = vec![];
    for block in content.split("\n\n") {
        let data = block.lines().map(|line| line.chars().collect()).collect();
        caves.push(Cave { data });
    }
    return caves;
}

fn solve(content: &String) -> i32 {
    let caves = parse(content);
    let mut total = 0;
    for cave in caves {
        // cave.print();
        let horizontal = cave.find_horizontal_mirror();
        let vertical = cave.find_vertical_mirror();
        match (horizontal, vertical) {
            (Some(h), None) => total += h * 100,
            (None, Some(v)) => total += v,
            _ => panic!("mirror must either vertical or horizontal but not both"),
        }
    }
    return total as i32;
}

fn main() {
    let content = fs::read_to_string("inputs/day13.txt").expect("input for day 13 is missing");
    let result = solve(&content);
    println!("day 13 part 1: {}", result);
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test() {
        let content = String::from(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        );
        let result = solve(&content);
        assert_eq!(result, 405);
    }
}
