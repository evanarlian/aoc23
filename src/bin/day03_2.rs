use std::{collections::HashSet, fmt::Debug, fs, path::Path};

fn debug_print<T>(board: &Vec<Vec<T>>)
where
    T: Debug,
{
    for line in board {
        println!("{:?}", line);
    }
}

fn find_gear_locations(engine: &String) -> Vec<(i32, i32)> {
    let mut gears = vec![];
    for (i, line) in engine.lines().enumerate() {
        let mut temp = line
            .chars()
            .enumerate()
            .filter(|(_j, c)| *c == '*')
            .map(|(j, _c)| (i as i32, j as i32))
            .collect::<Vec<_>>();
        gears.append(&mut temp);
    }
    return gears;
}

fn parse_to_number(engine: &String) -> Vec<Vec<i32>> {
    let mut parsed = vec![];
    for line in engine.lines() {
        let temp = line
            .chars()
            .map(|x| match x.to_digit(10) {
                Some(num) => num as i32,
                None => -1,
            })
            .collect();
        parsed.push(temp);
    }
    return parsed;
}

fn groupify(parsed: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (h, w) = (parsed.len(), parsed[0].len());
    let mut groups = vec![vec![-1; w]; h];
    let mut curr_group = 0;
    let mut previously_valid = false;
    for i in 0..h {
        for j in 0..w {
            if parsed[i][j] != -1 {
                groups[i][j] = curr_group;
                previously_valid = true;
            } else {
                if previously_valid {
                    curr_group += 1;
                    previously_valid = false;
                }
            }
        }
        if previously_valid {
            curr_group += 1;
            previously_valid = false;
        }
    }
    return groups;
}

fn create_group_mapping(parsed: &Vec<Vec<i32>>) -> Vec<i32> {
    let (h, w) = (parsed.len(), parsed[0].len());
    let mut mapping = vec![];
    let mut curr_num = 0;
    let mut previously_valid = false;
    for i in 0..h {
        for j in 0..w {
            if parsed[i][j] != -1 {
                curr_num = 10 * curr_num + parsed[i][j];
                previously_valid = true;
            } else {
                if previously_valid {
                    mapping.push(curr_num);
                    curr_num = 0;
                    previously_valid = false;
                }
            }
        }
        if previously_valid {
            mapping.push(curr_num);
            curr_num = 0;
            previously_valid = false;
        }
    }
    return mapping;
}

fn solve(engine: String) -> i32 {
    let gear_locations = find_gear_locations(&engine);
    let parsed = parse_to_number(&engine);
    let groups = groupify(&parsed);
    let group_mapping = create_group_mapping(&parsed);
    
    // dbg
    // debug_print(&parsed);
    // println!();
    // debug_print(&groups);
    // println!();
    // println!("{:?}", gear_locations);
    // println!("{:?}", group_mapping);

    // with this information we can compute the solution
    let mut total = 0;
    let (h, w) = (parsed.len() as i32, parsed[0].len() as i32);
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
    for (i, j) in gear_locations {
        // count unique groups from the 8 directions, and only allow exactly 2
        let mut set = HashSet::new();
        for (dy, dx) in dirs {
            if i + dy < 0 || i + dy >= h || j + dx < 0 || j + dx >= w {
                continue;
            }
            set.insert(groups[(i + dy) as usize][(j + dx) as usize]); // add group id on 8 directions
        }
        set.remove(&-1); // remove non real group is (-1)
        if set.len() != 2 {
            continue;
        }
        total += set
            .iter()
            .map(|gid| group_mapping[*gid as usize])
            .product::<i32>();
    }
    return total;
}

fn main() {
    let content = fs::read_to_string(Path::new("inputs/day03.txt"))
        .expect("input for day 3 is missing");
    let result = solve(content);
    println!("day 3 part 2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let content = String::from(
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
        let result = solve(content);
        assert_eq!(result, 467835);
    }
}
