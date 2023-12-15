use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::fs;
use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq, Eq, Hash)]
enum Rock {
    Round,
    Square,
    Air,
}

#[derive(Hash)]
struct Engine {
    map: Vec<Vec<Rock>>,
}
impl Engine {
    fn north(&mut self) {
        let (h, w) = (self.map.len(), self.map[0].len());
        for j in 0..w {
            let mut start_pos = 0;
            let mut is_after_square_rock = true;
            let mut round_rock_count = 0;
            for i in 0..h {
                if is_after_square_rock {
                    start_pos = i;
                    is_after_square_rock = false;
                }
                match self.map[i][j] {
                    Rock::Round => {
                        self.map[i][j] = Rock::Air;
                        round_rock_count += 1;
                    }
                    Rock::Square => {
                        for k in start_pos..start_pos + round_rock_count {
                            self.map[k][j] = Rock::Round;
                        }
                        is_after_square_rock = true;
                        round_rock_count = 0;
                    }
                    _ => {}
                }
            }
            for k in start_pos..start_pos + round_rock_count {
                self.map[k][j] = Rock::Round;
            }
        }
    }
    fn west(&mut self) {
        let (h, w) = (self.map.len(), self.map[0].len());
        for i in 0..h {
            let mut start_pos = 0;
            let mut is_after_square_rock = true;
            let mut round_rock_count = 0;
            for j in 0..w {
                if is_after_square_rock {
                    start_pos = j;
                    is_after_square_rock = false;
                }
                match self.map[i][j] {
                    Rock::Round => {
                        self.map[i][j] = Rock::Air;
                        round_rock_count += 1;
                    }
                    Rock::Square => {
                        for k in start_pos..start_pos + round_rock_count {
                            self.map[i][k] = Rock::Round;
                        }
                        is_after_square_rock = true;
                        round_rock_count = 0;
                    }
                    _ => {}
                }
            }
            for k in start_pos..start_pos + round_rock_count {
                self.map[i][k] = Rock::Round;
            }
        }
    }
    fn south(&mut self) {
        let (h, w) = (self.map.len(), self.map[0].len());
        for j in 0..w {
            let mut start_pos = 0;
            let mut is_after_square_rock = true;
            let mut round_rock_count = 0;
            for i in (0..h).rev() {
                if is_after_square_rock {
                    start_pos = i;
                    is_after_square_rock = false;
                }
                match self.map[i][j] {
                    Rock::Round => {
                        self.map[i][j] = Rock::Air;
                        round_rock_count += 1;
                    }
                    Rock::Square => {
                        for k in start_pos + 1 - round_rock_count..start_pos + 1 {
                            self.map[k][j] = Rock::Round;
                        }
                        is_after_square_rock = true;
                        round_rock_count = 0;
                    }
                    _ => {}
                }
            }
            for k in start_pos + 1 - round_rock_count..start_pos + 1 {
                self.map[k][j] = Rock::Round;
            }
        }
    }
    fn east(&mut self) {
        let (h, w) = (self.map.len(), self.map[0].len());
        for i in 0..h {
            let mut start_pos = 0;
            let mut is_after_square_rock = true;
            let mut round_rock_count = 0;
            for j in (0..w).rev() {
                if is_after_square_rock {
                    start_pos = j;
                    is_after_square_rock = false;
                }
                match self.map[i][j] {
                    Rock::Round => {
                        self.map[i][j] = Rock::Air;
                        round_rock_count += 1;
                    }
                    Rock::Square => {
                        for k in start_pos + 1 - round_rock_count..start_pos + 1 {
                            self.map[i][k] = Rock::Round;
                        }
                        is_after_square_rock = true;
                        round_rock_count = 0;
                    }
                    _ => {}
                }
            }
            for k in start_pos + 1 - round_rock_count..start_pos + 1 {
                self.map[i][k] = Rock::Round;
            }
        }
    }
    fn cycle(&mut self) {
        self.north();
        self.west();
        self.south();
        self.east();
    }
    fn get_score(&self) -> i32 {
        let h = self.map.len();
        self.map
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .map(|rock| match rock {
                        Rock::Round => h - i,
                        _ => 0,
                    })
                    .sum::<usize>()
            })
            .sum::<usize>() as i32
    }
    fn get_hash(&self) -> u64 {
        let mut state = DefaultHasher::new();
        self.hash(&mut state);
        state.finish()
    }
    fn print(&self) {
        let mut temp = String::new();
        for row in self.map.iter() {
            for rock in row.iter() {
                temp.push(match rock {
                    Rock::Air => '.',
                    Rock::Round => 'O',
                    Rock::Square => '#',
                });
            }
            temp.push('\n');
        }
        println!("{temp}");
    }
}

fn parse(content: &String) -> Engine {
    let map = content
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
        .collect();
    Engine { map }
}

fn solve(content: &String) -> i32 {
    let mut engine = parse(content);
    let mut hash_collection = HashSet::new();
    hash_collection.insert(engine.get_hash());
    // idea:
    // 1. find duplicates, that means the sequence will be like xxxxxabcdea <- last a triggers duplicate
    // 2. we want vector starting with hash a, that is abcde, but instead of hash, we store the score
    // 3. predict the future using how many iterations left and using modulo to get the score

    // step 1
    let mut first_duplicate = 0;
    let mut scores = vec![];
    let mut iterations_done = 0;
    loop {
        engine.cycle();
        iterations_done += 1;
        let curr_hash = engine.get_hash();
        // println!("[{i}] {curr_hash}");
        if hash_collection.contains(&curr_hash) {
            first_duplicate = curr_hash;
            scores.push(engine.get_score());
            break;
        }
        hash_collection.insert(curr_hash);
    }
    // step 2
    loop {
        engine.cycle();
        iterations_done += 1;
        let curr_hash = engine.get_hash();
        if curr_hash == first_duplicate {
            break;
        }
        scores.push(engine.get_score());
    }
    // step 3
    let iterations_todo = 1000000000 - iterations_done;
    // for (i, score) in scores.iter().enumerate() {
    //     println!("[{}] {}", i, score);
    // }
    // println!("iter done: {iterations_done}");
    // println!("iter todo: {iterations_todo}");
    // println!("mod {}", iterations_todo % scores.len());
    scores[iterations_todo % scores.len()] // 88371
}

fn main() {
    let content = fs::read_to_string("inputs/day14.txt").expect("input for day 14 is missing");
    let result = solve(&content);
    println!("day 14 part 2: {}", result);
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
        assert_eq!(result, 64);
    }
}
