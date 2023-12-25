# [Advent of Code 2023](https://adventofcode.com/2023)
Each challenge is separated into different files. To run, put input files to e.g. `inputs/day01.txt` and run with:
```bash
# example
cargo test --bin day01_1
cargo run --bin day05_2

# print to stdout and turn off multithreading to prevent racing
cargo test --bin day10_1 -- --nocapture --test-threads=1
```

## Notes
```bash
# Why is this (debug build) slower than python? Release is a bit faster tho
time cargo run --bin day12_2
time cargo run --release --bin day12_2
time python other/day12_2.py
```

## Progress
* [ ] day 20 part 2
* [ ] day 21 part 2
* [ ] day 22 part 1
* [ ] day 22 part 2
* [ ] day 24 part 1
* [ ] day 24 part 2
* [ ] day 25 part 1
* [ ] day 25 part 2
* [x] Everything else