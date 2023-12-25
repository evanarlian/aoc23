use std::{
    collections::{HashMap, VecDeque},
    fs,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Lo,
    Hi,
}

#[derive(Debug)]
enum Module {
    Broad {
        name: String,
    },
    Flip {
        name: String,
        state: Pulse,
    },
    Conj {
        name: String,
        input_states: HashMap<String, Pulse>,
    },
}
impl Module {
    fn send_pulse(&mut self, sender: &String, received_pulse: Pulse) -> Option<Pulse> {
        match self {
            Module::Broad { name: _ } => None,
            Module::Flip { name: _, state } => {
                if received_pulse == Pulse::Hi {
                    return None;
                }
                // save current state and toggle it
                let saved_state = state.clone();
                *state = match state {
                    Pulse::Hi => Pulse::Lo,
                    Pulse::Lo => Pulse::Hi,
                };
                // use saved state to perform duty
                match (received_pulse, saved_state) {
                    (Pulse::Lo, Pulse::Lo) => Some(Pulse::Hi),
                    (Pulse::Lo, Pulse::Hi) => Some(Pulse::Lo),
                    _ => None,
                }
            }
            Module::Conj {
                name: _,
                input_states,
            } => {
                assert!(input_states.contains_key(sender)); // this is guaranteed from parsing
                input_states.insert(sender.clone(), received_pulse);
                match input_states.iter().all(|(_, &pulse)| pulse == Pulse::Hi) {
                    true => Some(Pulse::Lo),
                    false => Some(Pulse::Hi),
                }
            }
        }
    }
}

fn parse(content: &String) -> (HashMap<String, Module>, HashMap<String, Vec<String>>) {
    // broadcaster -> a, b, c
    // %a -> b
    // %b -> c
    // %c -> inv
    // &inv -> a

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
            'b' => Module::Broad { name: name.clone() },
            '%' => Module::Flip {
                name: name.clone(),
                state: Pulse::Lo,
            },
            '&' => Module::Conj {
                name: name.clone(),
                input_states: HashMap::new(),
            },
            other => unreachable!("bad prefix: {other}"),
        };
        modules.insert(name, module);
    }
    // just for conj module, we need to keep track the input names
    for (_, module) in modules.iter_mut() {
        if let Module::Conj {
            name, input_states, ..
        } = module
        {
            for out_name in out2ins.get(name).unwrap() {
                input_states.insert(out_name.clone(), Pulse::Lo);
            }
        }
    }
    (modules, in2outs)
}

fn simulate(
    modules: &mut HashMap<String, Module>,
    transition: &HashMap<String, Vec<String>>,
) -> (i32, i32) {
    let mut lo_send = 1; // initially 1 from button module
    let mut hi_send = 0;
    let mut queue = VecDeque::<(&String, Pulse, &String)>::new();
    // first broadcast node
    let b = &String::from("broadcaster");
    for next in transition.get("broadcaster").unwrap() {
        queue.push_back((b, Pulse::Lo, next));
    }
    while !queue.is_empty() {
        let (prev, pulse, curr) = queue.pop_front().unwrap();
        // prev sends pulse to curr
        match pulse {
            Pulse::Hi => hi_send += 1,
            Pulse::Lo => lo_send += 1,
        }
        // special case for untyped module, thus cannot produce pulse
        if !modules.contains_key(curr) {
            continue;
        }
        // curr produce pulse (or not!)
        let curr_module = modules.get_mut(curr).unwrap();
        if let Some(curr_pulse) = curr_module.send_pulse(prev, pulse) {
            for next in transition.get(curr).unwrap() {
                queue.push_back((curr, curr_pulse, next));
            }
        }
    }
    (lo_send, hi_send)
}

fn solve(content: &String) -> i32 {
    let (mut modules, transition) = parse(content);
    let mut lo_send_total = 0;
    let mut hi_send_total = 0;
    // println!("MODULES");
    // for (k, v) in &modules {
    //     println!("{k:?} {v:?}");
    // }
    // println!("\nTRANSITION");
    // for (k, v) in &transition {
    //     println!("{k} {v:?}");
    // }
    for _ in 0..1000 {
        let (lo_send, hi_send) = simulate(&mut modules, &transition);
        lo_send_total += lo_send;
        hi_send_total += hi_send;
    }
    lo_send_total * hi_send_total
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
