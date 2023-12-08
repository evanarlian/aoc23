use std::{collections::HashSet, fmt::Debug, fs, path::Path};

fn parse(engine: String) -> Vec<Vec<char>> {
    let mut board: Vec<Vec<char>> = vec![];
    for line in engine.lines() {
        let linevec: Vec<char> = line.chars().collect();
        board.push(linevec);
    }
    return board;
}

fn debug_print<T>(board: &Vec<Vec<T>>)
where
    T: Debug,
{
    for line in board {
        println!("{:?}", line);
    }
}

fn floodfill(i: i32, j: i32, board: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>) {
    if i < 0 || i >= board.len() as i32 || j < 0 || j >= board[0].len() as i32 {
        return;
    }
    if visited[i as usize][j as usize] {
        return;
    }
    if board[i as usize][j as usize] == '.' {
        return;
    }
    visited[i as usize][j as usize] = true;
    let dirs = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    for (dy, dx) in dirs {
        floodfill(i + dy, j + dx, board, visited);
    }
}

fn sum_connected(board: &Vec<Vec<char>>, visited: &Vec<Vec<bool>>) -> i32 {
    let numbers: HashSet<char> = HashSet::from_iter("0123456789".chars());
    let mut total = 0;
    for i in 0..board.len() {
        let mut curr_group = 0;
        for j in 0..board[0].len() {
            if !(numbers.contains(&board[i][j]) && visited[i][j]) {
                total += curr_group;
                if curr_group != 0 {
                    // println!("{}", curr_group);
                }
                curr_group = 0;
                continue;
            }
            // at this point, the char is connected to the symbol and aslo parsable to int
            curr_group = 10 * curr_group + board[i][j].to_digit(10).unwrap() as i32;
        }
        if curr_group != 0 {
            // println!("{}", curr_group);
        }
        total += curr_group;
    }
    return total;
}

fn solve(engine: String) -> i32 {
    // the idea is to do floodfill and only keeping the numbers that is floodfilled
    let board = parse(engine);
    let (h, w) = (board.len(), board[0].len());
    let mut visited = vec![vec![false; w]; h];
    let not_symbols: HashSet<char> = HashSet::from_iter("0123456789.".chars());
    for i in 0..h {
        for j in 0..w {
            if !not_symbols.contains(&board[i][j]) {
                floodfill(i as i32, j as i32, &board, &mut visited)
            }
        }
    }
    // now collect all true
    let total = sum_connected(&board, &visited);
    // debug_print(&board);
    // debug_print(&visited);
    return total;
}

fn main() {
    let content = fs::read_to_string(Path::new("inputs/day03.txt"))
        .expect("input for day 3 is missing");
    let result = solve(content);
    println!("day 3 part 1: {}", result);
}

// cfg test is useful to silence the not used warning
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let engine = String::from(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        let result = solve(engine);
        assert_eq!(result, 4361);
    }
}
