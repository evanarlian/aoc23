use std::{collections::HashMap, fs};

#[derive(Debug, Clone, Copy)]
enum Pulse {
    Lo,
    Hi,
}

#[derive(Debug)]
enum Module {
    Flip {
        name: String,
        inputs: Vec<String>,
        outputs: Vec<String>,
        state: Pulse,
    },
    Conj {
        TODO: HashMap<>
        name: String,
        inputs: Vec<String>,
        outputs: Vec<String>,
        states: Vec<Pulse>,
    },
    Broad {
        name: String,
        outputs: Vec<String>,
    },
}

fn parse(content: &String) -> HashMap<String, Module> {
    // broadcaster -> a, b, c
    // %a -> b
    // %b -> c
    // %c -> inv
    // &inv -> a

    // AH FUCK turns out this is quite complex, we need hashmap to track input from previous

    // the parsing is a bit unique because we must know both right and left side first
    let mut in2outs: HashMap<String, Vec<String>> = HashMap::new();
    let mut out2ins: HashMap<String, Vec<String>> = HashMap::new();
    for row in content.lines() {
        let (left, right) = row.split_once(" -> ").unwrap();
        let prefix = left.chars().nth(0).unwrap();
        let in_name = match prefix {
            'b' => "broadcaster",
            _ => &left[1..],
        }
        .to_owned();
        let out_names = right.split(", ").map(|s| s.to_owned()).collect::<Vec<_>>();
        in2outs
            .entry(in_name.clone())
            .or_default()
            .extend(out_names.iter().cloned());
        for out_name in out_names {
            out2ins.entry(out_name).or_default().push(in_name.clone());
        }
    }
    // modules
    let mut modules = HashMap::new();
    for row in content.lines() {
        let (left, _) = row.split_once(" -> ").unwrap();
        let prefix = left.chars().nth(0).unwrap();
        let name = match prefix {
            'b' => "broadcaster",
            _ => &left[1..],
        }
        .to_owned();
        let module = match prefix {
            'b' => Module::Broad {
                name: name.clone(),
                outputs: in2outs[&name].clone(),
            },
            '%' => Module::Flip {
                name: name.clone(),
                inputs: out2ins[&name].clone(),
                outputs: in2outs[&name].clone(),
                state: Pulse::Lo,
            },
            '&' => Module::Conj {
                name: name.clone(),
                inputs: out2ins[&name].clone(),
                outputs: in2outs[&name].clone(),
                states: vec![Pulse::Lo; out2ins[&name].len()],
            },
            other => unreachable!("bad prefix: {other}"),
        };
        modules.insert(name, module);
    }
    modules
}

fn solve(content: &String) -> i32 {
    let modules = parse(content);
    for (k, v) in modules {
        println!("{k:?} {v:?}");
    }
    0
}

fn main() {
    let content = fs::read_to_string("inputs/day20.txt").expect("input for day 20 is missing");
    let result = solve(&content);
    println!("day 20 part 1: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let content = String::from(
            "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a",
        );
        let result = solve(&content);
        assert_eq!(result, 32000000);
    }

    #[test]
    fn test2() {
        let content = String::from(
            "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output",
        );
        let result = solve(&content);
        assert_eq!(result, 11687500);
    }
}
