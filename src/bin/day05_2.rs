use std::{cmp, collections::HashSet, fs, path::Path, vec};

/// Seed-related ///
struct Seed {
    start: i64,
    end: i64,
}
struct SeedBlock {
    seeds: Vec<Seed>,
}
impl SeedBlock {
    fn get_interesting_points(&self) -> HashSet<i64> {
        self.seeds.iter().flat_map(|s| [s.start, s.end]).collect()
    }
    fn is_valid_seed(&self, seed: i64) -> bool {
        for s in self.seeds.iter() {
            if s.start <= seed && seed <= s.end {
                return true;
            }
        }
        return false;
    }
}

/// Mapper-related ///
struct Mapper {
    source_start: i64,
    source_end: i64,
    dest_start: i64,
    dest_end: i64,
}
struct MapperBlock {
    mappers: Vec<Mapper>,
}
impl MapperBlock {
    fn remap(&self, x: i64) -> i64 {
        for mapper in self.mappers.iter() {
            if mapper.source_start <= x && x <= mapper.source_end {
                let diff = x - mapper.source_start;
                return mapper.dest_start + diff;
            }
        }
        return x;
    }
    fn inverse_remap(&self, x: i64) -> i64 {
        for mapper in self.mappers.iter().rev() {
            if mapper.dest_start <= x && x <= mapper.dest_end {
                let diff = x - mapper.dest_start;
                return mapper.source_start + diff;
            }
        }
        return x;
    }
    fn get_interesting_points(&self) -> HashSet<i64> {
        self.mappers
            .iter()
            .flat_map(|m| [m.source_start, m.source_end])
            .collect()
    }
}
struct MapperPipeline {
    mapper_blocks: Vec<MapperBlock>,
}
impl MapperPipeline {
    fn get_location(&self, x: i64) -> i64 {
        let mut result = x;
        for block in self.mapper_blocks.iter() {
            result = block.remap(result);
        }
        return result;
    }
    fn get_seed(&self, x: i64) -> i64 {
        let mut result = x;
        for block in self.mapper_blocks.iter().rev() {
            result = block.inverse_remap(result);
        }
        return result;
    }
    fn run(&self, initial_seed: &HashSet<i64>) -> HashSet<i64> {
        let mut unique_over_time = initial_seed.clone();
        for block in self.mapper_blocks.iter() {
            unique_over_time.extend(block.get_interesting_points());
            unique_over_time = unique_over_time.iter().map(|x| block.remap(*x)).collect();
        }
        return unique_over_time;
    }
}

fn parse_seed(seed_string: &str) -> SeedBlock {
    SeedBlock {
        seeds: seed_string
            .split_once(":")
            .unwrap()
            .1
            .trim()
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect::<Vec<i64>>()
            .chunks(2)
            .map(|chunk| Seed {
                start: chunk[0],
                end: chunk[0] + chunk[1] - 1,
            })
            .collect(),
    }
}

fn parse_mapper(mapper_string: &str) -> MapperPipeline {
    let mut mapper_blocks = vec![];
    for block in mapper_string.trim().split("\n\n") {
        let mapper_block = block
            .trim()
            .lines()
            .skip(1)
            .map(|mapper_string| {
                let mut temp_iter = mapper_string
                    .split_whitespace()
                    .map(|num| num.parse::<i64>().unwrap());
                let dest = temp_iter.next().unwrap();
                let source = temp_iter.next().unwrap();
                let range = temp_iter.next().unwrap();
                Mapper {
                    source_start: source,
                    source_end: source + range - 1,
                    dest_start: dest,
                    dest_end: dest + range - 1,
                }
            })
            .collect();
        mapper_blocks.push(MapperBlock {
            mappers: mapper_block,
        });
    }
    return MapperPipeline { mapper_blocks };
}

fn solve(content: String) -> i64 {
    let (seed_string, mapper_string) = content.split_once("\n\n").unwrap();
    let seed_block = parse_seed(seed_string);
    let pipeline = parse_mapper(mapper_string);
    let initial_seed = seed_block.get_interesting_points();
    let interesting_pts = pipeline.run(&initial_seed);
    // how we have all interesting points, just keep track of the min value
    let mut minimum_location = 999999999999999i64;
    for pts in interesting_pts {
        let maybe_seed = pipeline.get_seed(pts);
        if seed_block.is_valid_seed(maybe_seed) {
            minimum_location = cmp::min(minimum_location, pts);
        }
    }
    return minimum_location; //37384986
}

fn main() {
    // the main idea is to only keep track of the edge points because the extreme values will be one of the edges
    // get all edges point from the seed, then adding the start ranges for every blocks, put all of them to the
    // transfomation rules, and repeat. At the end you will have those interesting points. Reverse and check if
    // that location points back to valid seed while keeping track of the min location.
    let content = fs::read_to_string(Path::new("inputs/day05_2.txt"))
        .expect("input for day 5 part 2 is missing");
    let result = solve(content);
    println!("day 5 part 2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let content = String::from(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        );
        let result = solve(content);
        assert_eq!(result, 46);
    }
}
