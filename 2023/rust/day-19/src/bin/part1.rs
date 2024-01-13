use std::collections::HashMap;

fn main() {
    let mut input = include_str!("input.txt");
    let answer = process(&mut input);
    dbg!(answer);
}

#[derive(Debug, PartialEq)]
struct Part(HashMap<char, usize>);

impl From<&str> for Part {
    fn from(value: &str) -> Self {
        let ratings = value.trim_matches(['{', '}']).split(',').map(|r| {
            let (name, value) = r.split_once('=').unwrap();
            (
                name.chars().next().expect("should not be empty"),
                value.parse::<usize>().expect("should be a number"),
            )
        });

        Part(ratings.collect())
    }
}

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

impl Workflow {
    fn process(&self, part: &Part) -> &Output {
        self.rules
            .iter()
            .find_map(|rule| rule.evaluate(part))
            .expect("at least one rule should match")
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
    fn evaluate(&self, part: &Part) -> Option<&Output> {
        if self.cond == "*" {
            return Some(&self.output);
        }

        let mut cond = self.cond.chars();

        let name = cond.next().expect("should not be empty");
        let op = cond.next().expect("should not be empty");
        let value = cond
            .collect::<String>()
            .parse::<usize>()
            .expect("should be a number");

        match op {
            '<' if part.0[&name] < value => Some(&self.output),
            '>' if part.0[&name] > value => Some(&self.output),
            _ => None,
        }
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

fn parse(input: &mut &str) -> (HashMap<String, Workflow>, Vec<Part>) {
    let (workflows, parts) = input.split_once("\n\n").unwrap();

    let workflows = workflows.lines().map(|line| {
        let w = Workflow::from(line);
        (w.name.clone(), w)
    });
    let parts = parts.lines().map(Part::from);

    (workflows.collect(), parts.collect())
}

fn process(input: &mut &str) -> String {
    let (workflows, parts) = parse(input);

    parts
        .iter()
        .filter(|&part| {
            let mut workflow = &workflows["in"];
            loop {
                let output = workflow.process(part);
                if let Output::Workflow(w) = output {
                    workflow = &workflows[w];
                } else {
                    return output == &Output::Accept;
                }
            }
        })
        .map(|p| p.0.values().sum::<usize>())
        .sum::<usize>()
        .to_string()
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
        assert_eq!(result, "19114".to_string());
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
    fn test_parse_part() {
        assert_eq!(
            Part::from("{x=787,m=2655,a=1223,s=2876}"),
            Part(HashMap::from([
                ('x', 787),
                ('m', 2655),
                ('a', 1223),
                ('s', 2876)
            ]))
        )
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
    fn test_rule_evaluate() {
        let part = Part::from("{x=787,m=2655,a=1222,s=2876}");

        assert_eq!(Rule::from("a>1716:R").evaluate(&part), None);
        assert_eq!(
            Rule::from("s<5372:gd").evaluate(&part),
            Some(&Output::Workflow("gd".to_string()))
        );
        assert_eq!(Rule::from("A").evaluate(&part), Some(&Output::Accept));
    }
}
