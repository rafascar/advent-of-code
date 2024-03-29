use std::collections::HashMap;

fn main() {
    let mut input = include_str!("input.txt");
    let answer = process(&mut input);
    dbg!(answer);
}

#[derive(Debug, PartialEq, Clone)]
struct Part(HashMap<char, (usize, usize)>);

#[derive(Debug, PartialEq)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl From<&str> for Workflow {
    fn from(value: &str) -> Self {
        let (name, rules) = value.split_once('{').unwrap();
        let rules = rules.trim_end_matches('}').split(',').map(Rule::from);

        Workflow {
            name: name.to_string(),
            rules: rules.collect(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Rule {
    cond: String,
    output: Output,
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        if let Some((cond, output)) = value.split_once(':') {
            Rule {
                cond: cond.to_string(),
                output: output.into(),
            }
        } else {
            Rule {
                cond: "*".to_string(),
                output: value.into(),
            }
        }
    }
}

impl Rule {
    fn evaluate(&self, part: &Part) -> (Part, &Output, Part) {
        if self.cond == "*" {
            return (part.clone(), &self.output, part.clone());
        }

        let mut cond = self.cond.chars();

        let name = cond.next().expect("should not be empty");
        let op = cond.next().expect("should not be empty");
        let value = cond
            .collect::<String>()
            .parse::<usize>()
            .expect("should be a number");

        let mut new_ratings = part.0.clone();
        let mut next_ratings = part.0.clone();

        let old_rating = part.0[&name];
        let new_rating;
        let next_rating;

        if op == '<' {
            new_rating = (old_rating.0, value - 1);
            next_rating = (value, old_rating.1);
        } else {
            new_rating = (value + 1, old_rating.1);
            next_rating = (old_rating.0, value);
        }

        new_ratings.insert(name, new_rating);
        next_ratings.insert(name, next_rating);

        (Part(new_ratings), &self.output, Part(next_ratings))
    }
}

#[derive(Debug, PartialEq)]
enum Output {
    Accept,
    Reject,
    Workflow(String),
}

impl From<&str> for Output {
    fn from(value: &str) -> Self {
        match value {
            "A" => Output::Accept,
            "R" => Output::Reject,
            _ => Output::Workflow(value.to_string()),
        }
    }
}

fn parse(input: &mut &str) -> HashMap<String, Workflow> {
    let (workflows, _) = input.split_once("\n\n").unwrap();

    workflows
        .lines()
        .map(|line| {
            let w = Workflow::from(line);
            (w.name.clone(), w)
        })
        .collect()
}

fn process(input: &mut &str) -> String {
    let workflows = parse(input);

    let workflow = &workflows["in"];
    let part = Part(HashMap::from([
        ('x', (1, 4000)),
        ('m', (1, 4000)),
        ('a', (1, 4000)),
        ('s', (1, 4000)),
    ]));
    let mut accepted_parts = vec![];

    pipe(&workflows, workflow, part, &mut accepted_parts);

    accepted_parts
        .iter()
        .map(|part| part.0.values().map(|(s, e)| e - s + 1).product::<usize>())
        .sum::<usize>()
        .to_string()
}

fn pipe(
    workflows: &HashMap<String, Workflow>,
    workflow: &Workflow,
    part: Part,
    accepted_parts: &mut Vec<Part>,
) {
    let mut current_part = part.clone();

    for rule in &workflow.rules {
        let (new_part, output, next_part) = rule.evaluate(&current_part);

        if *output == Output::Accept {
            accepted_parts.push(new_part.clone());
        } else if let Output::Workflow(w) = output {
            pipe(workflows, &workflows[w], new_part.clone(), accepted_parts);
        }

        current_part = next_part;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        let mut input = "px{a<2006:qkq,m>2090:A,rfg}
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
{x=2127,m=1623,a=2188,s=1013}";

        let result = process(&mut input);
        assert_eq!(result, "167409079868000".to_string());
    }

    #[test]
    fn test_parse_rule() {
        assert_eq!(
            Rule::from("a>1716:R"),
            Rule {
                cond: String::from("a>1716"),
                output: Output::Reject
            }
        );

        assert_eq!(
            Rule::from("s<537:gd"),
            Rule {
                cond: String::from("s<537"),
                output: Output::Workflow("gd".to_string())
            }
        );

        assert_eq!(
            Rule::from("A"),
            Rule {
                cond: String::from("*"),
                output: Output::Accept,
            }
        );
    }

    #[test]
    fn test_parse_workflow() {
        assert_eq!(
            Workflow::from("pv{a>1716:R,A}"),
            Workflow {
                name: "pv".to_string(),
                rules: vec![
                    Rule {
                        cond: "a>1716".to_string(),
                        output: Output::Reject
                    },
                    Rule {
                        cond: "*".to_string(),
                        output: Output::Accept
                    }
                ]
            }
        )
    }

    #[test]
    fn test_rule_evaluate_gt() {
        let part = Part(HashMap::from([
            ('x', (1, 4000)),
            ('m', (1, 4000)),
            ('a', (1, 4000)),
            ('s', (1, 4000)),
        ]));

        let rule = Rule {
            cond: "a>1716".to_string(),
            output: Output::Reject,
        };

        let (new_part, output, next_part) = rule.evaluate(&part);
        assert_eq!(new_part.0[&'a'], (1717, 4000));
        assert_eq!(next_part.0[&'a'], (1, 1716));
        assert_eq!(*output, Output::Reject);
    }

    #[test]
    fn test_rule_evaluate_lt() {
        let part = Part(HashMap::from([
            ('x', (1, 4000)),
            ('m', (1, 4000)),
            ('a', (1, 4000)),
            ('s', (1, 4000)),
        ]));

        let rule = Rule {
            cond: "a<1716".to_string(),
            output: Output::Reject,
        };

        let (new_part, output, next_part) = rule.evaluate(&part);
        assert_eq!(new_part.0[&'a'], (1, 1715));
        assert_eq!(next_part.0[&'a'], (1716, 4000));
        assert_eq!(*output, Output::Reject);
    }
}
