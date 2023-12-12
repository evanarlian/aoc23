use std::fs;

#[derive(Default, Debug)]
struct Coord {
    y: i32,
    x: i32,
}
impl Coord {
    fn manhattan(&self, other: &Coord) -> i32 {
        (self.y - other.y).abs() + (self.x - other.x).abs()
    }
}

fn parse(content: &String) -> Vec<Coord> {
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
                galaxies.push(Coord {
                    y: i as i32 + starless_y_cumsum[i],
                    x: j as i32 + starless_x_cumsum[j],
                })
            }
        }
    }
    // println!("hole_y   {:?}", starless_y);
    // println!("cumsum_y {:?}", starless_y_cumsum);
    // println!("hole_x   {:?}", starless_x);
    // println!("cumsum_x {:?}", starless_x_cumsum);
    return galaxies;
}

fn shortest_paths(galaxies: &Vec<Coord>) -> i32 {
    let mut dist = 0;
    let n = galaxies.len();
    for i in 0..n {
        for j in i + 1..n {
            dist += galaxies[i].manhattan(&galaxies[j]);
        }
    }
    return dist;
}

fn solve(content: &String) -> i32 {
    let galaxies = parse(content);
    let dist = shortest_paths(&galaxies);
    return dist;
}

fn main() {
    // WOW i did not know this, turns out you can pass &str to a function that accepts Path because of AsRef<Path>
    let content = fs::read_to_string("inputs/day11.txt").expect("input for day 11 is missing");
    let result = solve(&content);
    println!("day 11 part 1: {}", result);
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test() {
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
        let result = solve(&content);
        assert_eq!(result, 374);
    }
}
