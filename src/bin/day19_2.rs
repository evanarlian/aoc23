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

#[derive(Debug, Clone, Copy)]
struct PartRange {
    x_from: i64,
    x_to: i64,
    m_from: i64,
    m_to: i64,
    a_from: i64,
    a_to: i64,
    s_from: i64,
    s_to: i64,
}
impl Default for PartRange {
    fn default() -> Self {
        PartRange {
            x_from: 1,
            x_to: 4000,
            m_from: 1,
            m_to: 4000,
            a_from: 1,
            a_to: 4000,
            s_from: 1,
            s_to: 4000,
        }
    }
}
impl PartRange {
    fn count_distinct(&self) -> i64 {
        (self.x_to - self.x_from + 1)
            * (self.m_to - self.m_from + 1)
            * (self.a_to - self.a_from + 1)
            * (self.s_to - self.s_from + 1)
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

fn count_all_accepted(
    workflows: &HashMap<String, Workflow>,
    wf_name: &String,
    wf_idx: usize,
    part_range: PartRange,
) -> i64 {
    let rule = &workflows[wf_name].rules[wf_idx];
    match rule {
        Rule::Immediate(wf_type) => match wf_type {
            WorkflowType::Accepted => part_range.count_distinct(),
            WorkflowType::Rejected => 0,
            WorkflowType::Continue(next_wf) => {
                count_all_accepted(workflows, next_wf, 0, part_range)
            }
        },
        Rule::Evaluate(attr, comp, value, wf_type) => {
            // after evaluation, there are 3 possible ways:
            // (1) both from and to is contained in the rule, eg: 3-1200 with rule x<1500, resulting in going to new workflow
            // (2) both from and to is NOT contained in the rule, eg: 500-3000 with rule x<100, resulting in continuing workflow
            // (3) rule cuts in the middle of the ranges, eg: 300-1000 with rule x<500, resulting in (300-499) and (500-1000)
            match (attr, comp) {
                (Attr::X, Comp::Gt) => {
                    if part_range.x_from > *value {
                        // case (1)
                        match wf_type {
                            WorkflowType::Accepted => part_range.count_distinct(),
                            WorkflowType::Rejected => 0,
                            WorkflowType::Continue(next_wf) => {
                                count_all_accepted(workflows, next_wf, 0, part_range)
                            }
                        }
                    } else if part_range.x_to <= *value {
                        // case (2)
                        count_all_accepted(workflows, wf_name, wf_idx + 1, part_range)
                    } else {
                        // case (3a) (contained part range)
                        let contained_part_range = PartRange {
                            x_from: *value + 1,
                            x_to: part_range.x_to,
                            ..part_range
                        };
                        let a = match wf_type {
                            WorkflowType::Accepted => contained_part_range.count_distinct(),
                            WorkflowType::Rejected => 0,
                            WorkflowType::Continue(next_wf) => {
                                count_all_accepted(workflows, next_wf, 0, contained_part_range)
                            }
                        };
                        // case (3b) (not contained part range)
                        let not_contained_part_range = PartRange {
                            x_from: part_range.x_from,
                            x_to: *value,
                            ..part_range
                        };
                        let b = count_all_accepted(
                            workflows,
                            wf_name,
                            wf_idx + 1,
                            not_contained_part_range,
                        );
                        a + b
                    }
                }
                (Attr::M, Comp::Gt) => {
                    if part_range.m_from > *value {
                        // case (1)
                        match wf_type {
                            WorkflowType::Accepted => part_range.count_distinct(),
                            WorkflowType::Rejected => 0,
                            WorkflowType::Continue(next_wf) => {
                                count_all_accepted(workflows, next_wf, 0, part_range)
                            }
                        }
                    } else if part_range.m_to <= *value {
                        // case (2)
                        count_all_accepted(workflows, wf_name, wf_idx + 1, part_range)
                    } else {
                        // case (3a) (contained part range)
                        let contained_part_range = PartRange {
                            m_from: *value + 1,
                            m_to: part_range.m_to,
                            ..part_range
                        };
                        let a = match wf_type {
                            WorkflowType::Accepted => contained_part_range.count_distinct(),
                            WorkflowType::Rejected => 0,
                            WorkflowType::Continue(next_wf) => {
                                count_all_accepted(workflows, next_wf, 0, contained_part_range)
                            }
                        };
                        // case (3b) (not contained part range)
                        let not_contained_part_range = PartRange {
                            m_from: part_range.m_from,
                            m_to: *value,
                            ..part_range
                        };
                        let b = count_all_accepted(
                            workflows,
                            wf_name,
                            wf_idx + 1,
                            not_contained_part_range,
                        );
                        a + b
                    }
                }
                (Attr::A, Comp::Gt) => {
                    if part_range.a_from > *value {
                        // case (1)
                        match wf_type {
                            WorkflowType::Accepted => part_range.count_distinct(),
                            WorkflowType::Rejected => 0,
                            WorkflowType::Continue(next_wf) => {
                                count_all_accepted(workflows, next_wf, 0, part_range)
                            }
                        }
                    } else if part_range.a_to <= *value {
                        // case (2)
                        count_all_accepted(workflows, wf_name, wf_idx + 1, part_range)
                    } else {
                        // case (3a) (contained part range)
                        let contained_part_range = PartRange {
                            a_from: *value + 1,
                            a_to: part_range.a_to,
                            ..part_range
                        };
                        let a = match wf_type {
                            WorkflowType::Accepted => contained_part_range.count_distinct(),
                            WorkflowType::Rejected => 0,
                            WorkflowType::Continue(next_wf) => {
                                count_all_accepted(workflows, next_wf, 0, contained_part_range)
                            }
                        };
                        // case (3b) (not contained part range)
                        let not_contained_part_range = PartRange {
                            a_from: part_range.a_from,
                            a_to: *value,
                            ..part_range
                        };
                        let b = count_all_accepted(
                            workflows,
                            wf_name,
                            wf_idx + 1,
                            not_contained_part_range,
                        );
                        a + b
                    }
                }
                (Attr::S, Comp::Gt) => {
                    if part_range.s_from > *value {
                        // case (1)
                        match wf_type {
                            WorkflowType::Accepted => part_range.count_distinct(),
                            WorkflowType::Rejected => 0,
                            WorkflowType::Continue(next_wf) => {
                                count_all_accepted(workflows, next_wf, 0, part_range)
                            }
                        }
                    } else if part_range.s_to <= *value {
                        // case (2)
                        count_all_accepted(workflows, wf_name, wf_idx + 1, part_range)
                    } else {
                        // case (3a) (contained part range)
                        let contained_part_range = PartRange {
                            s_from: *value + 1,
                            s_to: part_range.s_to,
                            ..part_range
                        };
                        let a = match wf_type {
                            WorkflowType::Accepted => contained_part_range.count_distinct(),
                            WorkflowType::Rejected => 0,
                            WorkflowType::Continue(next_wf) => {
                                count_all_accepted(workflows, next_wf, 0, contained_part_range)
                            }
                        };
                        // case (3b) (not contained part range)
                        let not_contained_part_range = PartRange {
                            s_from: part_range.s_from,
                            s_to: *value,
                            ..part_range
                        };
                        let b = count_all_accepted(
                            workflows,
                            wf_name,
                            wf_idx + 1,
                            not_contained_part_range,
                        );
                        a + b
                    }
                }
                (Attr::X, Comp::Lt) => {
                    if part_range.x_to < *value {
                        // case (1)
                        match wf_type {
                            WorkflowType::Accepted => part_range.count_distinct(),
                            WorkflowType::Rejected => 0,
                            WorkflowType::Continue(next_wf) => {
                                count_all_accepted(workflows, next_wf, 0, part_range)
                            }
                        }
                    } else if part_range.x_from >= *value {
                        // case (2)
                        count_all_accepted(workflows, wf_name, wf_idx + 1, part_range)
                    } else {
                        // case (3a) (contained part range)
                        let contained_part_range = PartRange {
                            x_from: part_range.x_from,
                            x_to: *value - 1,
                            ..part_range
                        };
                        let a = match wf_type {
                            WorkflowType::Accepted => contained_part_range.count_distinct(),
                            WorkflowType::Rejected => 0,
                            WorkflowType::Continue(next_wf) => {
                                count_all_accepted(workflows, next_wf, 0, contained_part_range)
                            }
                        };
                        // case (3b) (not contained part range)
                        let not_contained_part_range = PartRange {
                            x_from: *value,
                            x_to: part_range.x_to,
                            ..part_range
                        };
                        let b = count_all_accepted(
                            workflows,
                            wf_name,
                            wf_idx + 1,
                            not_contained_part_range,
                        );
                        a + b
                    }
                }
                (Attr::M, Comp::Lt) => {
                    if part_range.m_to < *value {
                        // case (1)
                        match wf_type {
                            WorkflowType::Accepted => part_range.count_distinct(),
                            WorkflowType::Rejected => 0,
                            WorkflowType::Continue(next_wf) => {
                                count_all_accepted(workflows, next_wf, 0, part_range)
                            }
                        }
                    } else if part_range.m_from >= *value {
                        // case (2)
                        count_all_accepted(workflows, wf_name, wf_idx + 1, part_range)
                    } else {
                        // case (3a) (contained part range)
                        let contained_part_range = PartRange {
                            m_from: part_range.m_from,
                            m_to: *value - 1,
                            ..part_range
                        };
                        let a = match wf_type {
                            WorkflowType::Accepted => contained_part_range.count_distinct(),
                            WorkflowType::Rejected => 0,
                            WorkflowType::Continue(next_wf) => {
                                count_all_accepted(workflows, next_wf, 0, contained_part_range)
                            }
                        };
                        // case (3b) (not contained part range)
                        let not_contained_part_range = PartRange {
                            m_from: *value,
                            m_to: part_range.m_to,
                            ..part_range
                        };
                        let b = count_all_accepted(
                            workflows,
                            wf_name,
                            wf_idx + 1,
                            not_contained_part_range,
                        );
                        a + b
                    }
                }
                (Attr::A, Comp::Lt) => {
                    if part_range.a_to < *value {
                        // case (1)
                        match wf_type {
                            WorkflowType::Accepted => part_range.count_distinct(),
                            WorkflowType::Rejected => 0,
                            WorkflowType::Continue(next_wf) => {
                                count_all_accepted(workflows, next_wf, 0, part_range)
                            }
                        }
                    } else if part_range.a_from >= *value {
                        // case (2)
                        count_all_accepted(workflows, wf_name, wf_idx + 1, part_range)
                    } else {
                        // case (3a) (contained part range)
                        let contained_part_range = PartRange {
                            a_from: part_range.a_from,
                            a_to: *value - 1,
                            ..part_range
                        };
                        let a = match wf_type {
                            WorkflowType::Accepted => contained_part_range.count_distinct(),
                            WorkflowType::Rejected => 0,
                            WorkflowType::Continue(next_wf) => {
                                count_all_accepted(workflows, next_wf, 0, contained_part_range)
                            }
                        };
                        // case (3b) (not contained part range)
                        let not_contained_part_range = PartRange {
                            a_from: *value,
                            a_to: part_range.a_to,
                            ..part_range
                        };
                        let b = count_all_accepted(
                            workflows,
                            wf_name,
                            wf_idx + 1,
                            not_contained_part_range,
                        );
                        a + b
                    }
                }
                (Attr::S, Comp::Lt) => {
                    if part_range.s_to < *value {
                        // case (1)
                        match wf_type {
                            WorkflowType::Accepted => part_range.count_distinct(),
                            WorkflowType::Rejected => 0,
                            WorkflowType::Continue(next_wf) => {
                                count_all_accepted(workflows, next_wf, 0, part_range)
                            }
                        }
                    } else if part_range.s_from >= *value {
                        // case (2)
                        count_all_accepted(workflows, wf_name, wf_idx + 1, part_range)
                    } else {
                        // case (3a) (contained part range)
                        let contained_part_range = PartRange {
                            s_from: part_range.s_from,
                            s_to: *value - 1,
                            ..part_range
                        };
                        let a = match wf_type {
                            WorkflowType::Accepted => contained_part_range.count_distinct(),
                            WorkflowType::Rejected => 0,
                            WorkflowType::Continue(next_wf) => {
                                count_all_accepted(workflows, next_wf, 0, contained_part_range)
                            }
                        };
                        // case (3b) (not contained part range)
                        let not_contained_part_range = PartRange {
                            s_from: *value,
                            s_to: part_range.s_to,
                            ..part_range
                        };
                        let b = count_all_accepted(
                            workflows,
                            wf_name,
                            wf_idx + 1,
                            not_contained_part_range,
                        );
                        a + b
                    }
                }
            }
        }
    }
}

fn solve(content: &String) -> i64 {
    let workflows = parse(content);
    // for (name, wf) in &workflows {
    //     println!("{name} => {wf:?}");
    // }
    count_all_accepted(&workflows, &String::from("in"), 0, PartRange::default())
}

fn main() {
    let content = fs::read_to_string("inputs/day19.txt").expect("input for day 19 is missing");
    let result = solve(&content);
    println!("day 19 part 2: {}", result);
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
