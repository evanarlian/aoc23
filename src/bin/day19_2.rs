use std::{collections::HashMap, fs};

#[derive(Debug)]
enum Attr {
    X,
    M,
    A,
    S,
}
#[derive(Debug)]
enum Comp {
    Lt,
    Gt,
}

#[derive(Debug)]
enum WorkflowType {
    Rejected,
    Accepted,
    Continue(String),
}

#[derive(Debug)]
enum Rule {
    Evaluate(Attr, Comp, i64, WorkflowType),
    Immediate(WorkflowType),
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

#[derive(Default, Debug)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}
impl Part {
    fn xmas_sum(&self) -> i64 {
        self.x + self.m + self.a + self.s
    }
}

fn parse(content: &String) -> HashMap<String, Workflow> {
    let raw_workflows = content.split_once("\n\n").unwrap().0;
    let mut workflows = HashMap::new();
    for line in raw_workflows.lines() {
        let (name, rest) = line.split_once("{").unwrap();
        let mut rules = vec![];
        let chunks = (&rest[..rest.len() - 1]).split(",").collect::<Vec<_>>();
        for chunk in chunks {
            if chunk.contains(":") {
                let (rest, next_workflow) = chunk.split_once(":").unwrap();
                let mut rest = rest.chars();
                let attr = rest.next().unwrap();
                let sign = rest.next().unwrap();
                let value = rest.collect::<String>().parse::<i64>().unwrap();
                rules.push(Rule::Evaluate(
                    match attr {
                        'x' => Attr::X,
                        'm' => Attr::M,
                        'a' => Attr::A,
                        's' => Attr::S,
                        other => unreachable!("bad attr: {other}"),
                    },
                    match sign {
                        '<' => Comp::Lt,
                        '>' => Comp::Gt,
                        other => unreachable!("bad comparison: {other}"),
                    },
                    value,
                    match next_workflow {
                        "R" => WorkflowType::Rejected,
                        "A" => WorkflowType::Accepted,
                        normal_wf => WorkflowType::Continue(normal_wf.to_owned()),
                    },
                ));
            } else {
                rules.push(Rule::Immediate(match chunk {
                    "R" => WorkflowType::Rejected,
                    "A" => WorkflowType::Accepted,
                    normal_wf => WorkflowType::Continue(normal_wf.to_owned()),
                }));
            }
        }
        workflows.insert(
            name.to_owned(),
            Workflow {
                name: name.to_owned(),
                rules,
            },
        );
    }
    workflows
}

fn check_accepted(workflows: &HashMap<String, Workflow>, part: &Part) -> bool {
    let mut curr_name = &String::from("in");
    loop {
        let curr_wf = &workflows[curr_name];
        for rule in &curr_wf.rules {
            match rule {
                Rule::Evaluate(attr, comp, value, next_wf) => {
                    let rule_accepted = match (attr, comp) {
                        (Attr::X, Comp::Gt) => part.x > *value,
                        (Attr::M, Comp::Gt) => part.m > *value,
                        (Attr::A, Comp::Gt) => part.a > *value,
                        (Attr::S, Comp::Gt) => part.s > *value,
                        (Attr::X, Comp::Lt) => part.x < *value,
                        (Attr::M, Comp::Lt) => part.m < *value,
                        (Attr::A, Comp::Lt) => part.a < *value,
                        (Attr::S, Comp::Lt) => part.s < *value,
                    };
                    if rule_accepted {
                        match next_wf {
                            WorkflowType::Accepted => return true,
                            WorkflowType::Rejected => return false,
                            WorkflowType::Continue(wf_name) => {
                                curr_name = wf_name;
                                break;
                            }
                        }
                    }
                }
                Rule::Immediate(next_wf) => match next_wf {
                    WorkflowType::Accepted => return true,
                    WorkflowType::Rejected => return false,
                    WorkflowType::Continue(wf_name) => {
                        curr_name = wf_name;
                        break;
                    }
                },
            }
        }
    }
}

fn solve(content: &String) -> i64 {
    let workflows = parse(content);
    for (name, wf) in &workflows {
        println!("{name} => {wf:?}");
    }
    let mut total = 0;
    total
}

fn main() {
    let content = fs::read_to_string("inputs/day19.txt").expect("input for day 19 is missing");
    let result = solve(&content);
    println!("day 19 part 1: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let content = String::from(
            "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}",
        );
        let result = solve(&content);
        assert_eq!(result, 167409079868000);
    }
}
