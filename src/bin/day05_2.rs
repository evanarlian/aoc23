use std::{fs, path::Path, vec};

struct Mapper {
    dest: i64,
    source: i64,
    range: i64,
}
struct MapperBlock {
    mappers: Vec<Mapper>,
}

impl MapperBlock {
    // notes:
    // `&self` is just simplification of
    // `self: &Self` which is also simplification of
    // `self: &MapperBlock`
    fn remap(&self, x: i64) -> i64 {
        for mapper in &self.mappers {
            // example source 98, dest 50, range 2 is (98, 99) -> (50, 51)
            if mapper.source <= x && x < mapper.source + mapper.range {
                let diff = x - mapper.source;
                return mapper.dest + diff;
            }
        }
        return x; // unmapped, just return original
    }
}

struct MapperPipeline {
    mapper_blocks: Vec<MapperBlock>,
}

impl MapperPipeline {
    fn get_last(&self, x: i64) -> i64 {
        let mut result = x;
        for block in &self.mapper_blocks {
            result = block.remap(result);
        }
        return result;
    }
}

fn parse_seed(seed_string: &str) -> Vec<i64> {
    let mut seeds = vec![];
    for (seed, range) in seed_string
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect::<Vec<i64>>()
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[1]))
    {
        for i in 0..range {
            seeds.push(seed + i);
        }
    }
    return seeds;
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
                Mapper {
                    dest: temp_iter.next().unwrap(),
                    source: temp_iter.next().unwrap(),
                    range: temp_iter.next().unwrap(),
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
    let seeds = parse_seed(seed_string);
    let pipeline = parse_mapper(mapper_string);
    return seeds
        .iter()
        .map(|seed| pipeline.get_last(*seed))
        .min()
        .unwrap();
}

fn main() {
    let content = fs::read_to_string(Path::new("inputs/day05_2.txt"))
        .expect("input for day 5 part 1 is missing");
    let result = solve(content);
    println!("day 5 part 1: {}", result);
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
