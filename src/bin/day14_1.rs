use std::fs;

#[derive(Debug, PartialEq, Eq)]
enum Rock {
    Round,
    Square,
    Air,
}

fn parse(content: &String) -> Vec<Vec<Rock>> {
    content
        .lines()
        .map(|line| {
            line.chars()
                .map(|rock| match rock {
                    'O' => Rock::Round,
                    '#' => Rock::Square,
                    '.' => Rock::Air,
                    invalid => unreachable!("bad input {invalid}"),
                })
                .collect()
        })
        .collect()
}

fn tilt_north(map: &Vec<Vec<Rock>>) -> i32 {
    let (h, w) = (map.len(), map[0].len());
    let mut total = 0;
    for j in 0..w {
        // count round rocks in the segment and use some cool shit math
        let mut start_pos = 0;
        let mut is_after_square_rock = true;
        let mut round_rock_count = 0;
        for i in 0..h {
            if is_after_square_rock {
                start_pos = i;
                is_after_square_rock = false;
            }
            match map[i][j] {
                Rock::Air => {
                    // find air, do nothing
                }
                Rock::Round => {
                    round_rock_count += 1;
                }
                Rock::Square => {
                    if round_rock_count > 0 {
                        // only handle meaningful addition, that means skipping adjacent square rocks
                        // below math is for example: 9 + 8 + 7 + 6
                        let biggest_score = h - start_pos;
                        total += (biggest_score + (biggest_score - round_rock_count + 1))
                            * round_rock_count
                            / 2;
                    }
                    // reset
                    is_after_square_rock = true;
                    round_rock_count = 0;
                }
            }
        }
        // add the last group
        if round_rock_count > 0 {
            // only handle meaningful addition, that means skipping adjacent square rocks
            // below math is just for example: 9 + 8 + 7 + 6
            let biggest_score = h - start_pos;
            total +=
                (biggest_score + (biggest_score - round_rock_count + 1)) * round_rock_count / 2;
        }
    }
    total as i32
}

fn solve(content: &String) -> i32 {
    let map = parse(content);
    tilt_north(&map)
}

fn main() {
    let content = fs::read_to_string("inputs/day14.txt").expect("input for day 14 is missing");
    let result = solve(&content);
    println!("day 14 part 1: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let content = String::from(
            "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        );
        let result = solve(&content);
        assert_eq!(result, 136);
    }
}
