use std::fs;

#[derive(Default, Debug)]
struct Coord {
    y: i64,
    x: i64,
}
impl Coord {
    fn manhattan(&self, other: &Coord) -> i64 {
        (self.y - other.y).abs() + (self.x - other.x).abs()
    }
}

fn parse(content: &String, expansion_multiplier: i64) -> Vec<Coord> {
    // parse to char first
    let space = content
        .lines()
        .map(|row| row.chars().collect())
        .collect::<Vec<Vec<char>>>();
    // find where the hole is in both x and y
    let (h, w) = (space.len(), space[0].len());
    let mut starless_y = vec![0; h];
    let mut starless_x = vec![0; w];
    for (i, row) in space.iter().enumerate() {
        if row.iter().all(|&x| x == '.') {
            starless_y[i] = 1;
        }
    }
    for j in 0..w {
        let mut galaxy_found = false;
        for i in 0..h {
            if space[i][j] == '#' {
                galaxy_found = true;
                break;
            }
        }
        if !galaxy_found {
            starless_x[j] = 1;
        }
    }
    // calculate the coords but added with cumulative sum of holes
    let starless_y_cumsum = starless_y
        .iter()
        .scan(0, |cumsum, x| {
            *cumsum += x;
            Some(*cumsum)
        })
        .collect::<Vec<_>>();
    let starless_x_cumsum = starless_x
        .iter()
        .scan(0, |cumsum, x| {
            *cumsum += x;
            Some(*cumsum)
        })
        .collect::<Vec<_>>();
    // return the galaxies but with the cumulative sum added
    let mut galaxies = vec![];
    for (i, row) in space.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == '#' {
                // -1 is for taking into account that we are still counting the original empty space
                galaxies.push(Coord {
                    y: i as i64 + ((expansion_multiplier - 1) * starless_y_cumsum[i]),
                    x: j as i64 + ((expansion_multiplier - 1) * starless_x_cumsum[j]),
                })
            }
        }
    }
    // println!("hole_y   {:?}", starless_y);
    // println!("cumsum_y {:?}", starless_y_cumsum);
    // println!("hole_x   {:?}", starless_x);
    // println!("cumsum_x {:?}", starless_x_cumsum);
    // println!();
    return galaxies;
}

fn shortest_paths(galaxies: &Vec<Coord>) -> i64 {
    let mut dist = 0;
    let n = galaxies.len();
    for i in 0..n {
        for j in i + 1..n {
            dist += galaxies[i].manhattan(&galaxies[j]);
        }
    }
    return dist;
}

fn solve(content: &String, expansion_multiplier: i64) -> i64 {
    let galaxies = parse(content, expansion_multiplier);
    // for g in galaxies.iter() {
    //     println!("{:?}", g);
    // }
    let dist = shortest_paths(&galaxies);
    return dist;
}

fn main() {
    // WOW i did not know this, turns out you can pass &str to a function that accepts Path because of AsRef<Path>
    let content = fs::read_to_string("inputs/day11.txt").expect("input for day 11 is missing");
    let result = solve(&content, 1000000);
    println!("day 11 part 2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test() {
        // hmm i want this loop to be similar to pytest's parametrize, but require installing rstest, so i use loops instead
        let content = String::from(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        );
        // the old multiplier is 2 because we are replacing 1 empty space to 2
        // multiplier = 1 means there is no change
        let multipliers = vec![2, 10, 100];
        let answers = vec![374, 1030, 8410];
        for (mult, ans) in multipliers.into_iter().zip(answers.into_iter()) {
            let result = solve(&content, mult);
            assert_eq!(result, ans);
        }
    }
}
