use std::fs;

enum Command {
    Add { label: String, focal_length: usize },
    Delete { label: String },
}

#[derive(Clone, Debug)]
struct Lens {
    label: String,
    focal_length: usize,
}

const N: usize = 256;

fn parse(content: &String) -> Vec<Command> {
    content
        .split(",")
        .map(|p| match p.contains("=") {
            true => {
                let (label, focal_length) = p.split_once("=").unwrap();
                Command::Add {
                    label: label.to_string(),
                    focal_length: focal_length.parse().unwrap(),
                }
            }
            false => {
                let label = p.strip_suffix("-").unwrap();
                Command::Delete {
                    label: label.to_string(),
                }
            }
        })
        .collect()
}

fn label_hash(s: &str) -> usize {
    let mut curr_hash = 0;
    for c in s.chars() {
        curr_hash = (curr_hash + c as usize) * 17 % N;
    }
    curr_hash
}

fn solve(content: &String) -> i32 {
    let commands = parse(content);
    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; N];
    for command in commands {
        match command {
            Command::Add {
                label,
                focal_length,
            } => {
                let index = label_hash(&label);
                let new_lens = Lens {
                    label: label.clone(),
                    focal_length,
                };
                if let Some(lens_idx) = boxes[index].iter().position(|lens| lens.label == label) {
                    boxes[index][lens_idx] = new_lens;
                } else {
                    boxes[index].push(new_lens);
                }
            }
            Command::Delete { label } => {
                let index = label_hash(&label);
                if let Some(lens_idx) = boxes[index].iter().position(|lens| lens.label == label) {
                    boxes[index].remove(lens_idx);
                }
            }
        }
    }
    // debug
    // for (box_id, lenses) in boxes.iter().enumerate() {
    //     if lenses.is_empty() {
    //         continue;
    //     }
    //     println!("[{box_id}] {lenses:?}");
    // }
    let mut total = 0;
    for (box_id, lenses) in boxes.iter().enumerate() {
        total += lenses
            .iter()
            .enumerate()
            .map(|(lens_id, lens)| (box_id + 1) * (lens_id + 1) * lens.focal_length)
            .sum::<usize>();
    }
    total as i32
}

fn main() {
    let content = fs::read_to_string("inputs/day15.txt").expect("input for day 15 is missing");
    let result = solve(&content);
    println!("day 15 part 2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let content = String::from("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        let result = solve(&content);
        assert_eq!(result, 145);
    }
}
