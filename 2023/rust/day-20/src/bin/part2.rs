use std::{
    collections::{HashMap, VecDeque},
    fmt::{Debug, Display},
};

use num::integer::lcm;

fn main() {
    let input = include_str!("input.txt");
    let answer = process(input);
    dbg!(answer);
}

#[derive(Debug, PartialEq)]
struct Pulse {
    from: String,
    to: String,
    high: bool,
}

impl Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let level = if self.high { "high" } else { "low" };
        write!(f, "{} -{}-> {}", self.from, level, self.to)
    }
}

#[derive(Debug)]
enum Module {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Broadcaster(Broadcaster),
}

#[derive(Debug)]
struct FlipFlop {
    name: String,
    on: bool,
    outs: Vec<String>,
}

impl FlipFlop {
    fn process(&mut self, pulse: &Pulse) -> Vec<Pulse> {
        if pulse.high {
            return vec![];
        }

        self.on = !self.on;
        let from = &self.name;
        self.outs
            .iter()
            .map(|out| Pulse {
                from: from.to_string(),
                to: out.to_string(),
                high: self.on,
            })
            .collect()
    }
}

#[derive(Debug)]
struct Conjunction {
    name: String,
    state: HashMap<String, bool>,
    outs: Vec<String>,
}

impl Conjunction {
    fn process(&mut self, pulse: &Pulse) -> Vec<Pulse> {
        self.state.insert(pulse.from.to_string(), pulse.high);

        let high = !self.state.values().all(|v| *v);
        let from = &self.name;
        self.outs
            .iter()
            .map(|out| Pulse {
                from: from.to_string(),
                to: out.to_string(),
                high,
            })
            .collect()
    }
}

#[derive(Debug)]
struct Broadcaster {
    outs: Vec<String>,
}

impl Broadcaster {
    fn process(&mut self, pulse: &Pulse) -> Vec<Pulse> {
        self.outs
            .iter()
            .map(|out| Pulse {
                from: String::from("broadcaster"),
                to: out.to_string(),
                high: pulse.high,
            })
            .collect()
    }
}

fn parse(input: &str) -> HashMap<String, Module> {
    let mut modules = input
        .lines()
        .filter_map(|line| {
            let (name, rest) = line.split_once(' ').expect("should find a whitespace");
            let (_, outs) = rest.split_once("-> ").expect("should find an arrow");

            let outs = outs
                .split(", ")
                .map(|out| out.to_string())
                .collect::<Vec<_>>();

            if name == "broadcaster" {
                return Some((name.to_string(), Module::Broadcaster(Broadcaster { outs })));
            }

            match name.chars().next().expect("should not be empty") {
                '%' => Some((
                    name[1..].to_string(),
                    Module::FlipFlop(FlipFlop {
                        name: name[1..].to_string(),
                        on: false,
                        outs,
                    }),
                )),
                '&' => Some((
                    name[1..].to_string(),
                    Module::Conjunction(Conjunction {
                        name: name[1..].to_string(),
                        state: HashMap::new(),
                        outs,
                    }),
                )),
                _ => None,
            }
        })
        .collect::<HashMap<String, Module>>();

    let mut conj_ins: HashMap<String, Vec<String>> = HashMap::new();
    for (name, module) in modules.iter() {
        let outs = match module {
            Module::FlipFlop(ff) => &ff.outs,
            Module::Conjunction(c) => &c.outs,
            Module::Broadcaster(b) => &b.outs,
        };

        for out in outs {
            if let Some(Module::Conjunction(_)) = modules.get(out) {
                conj_ins
                    .entry(out.to_string())
                    .or_default()
                    .push(name.to_string());
            }
        }
    }

    for (name, ins) in conj_ins.iter() {
        if let Some(Module::Conjunction(c)) = modules.get_mut(name) {
            for input in ins {
                c.state.insert(input.to_string(), false);
            }
        }
    }

    modules
}

fn process(input: &str) -> String {
    let mut modules = parse(input);

    let mut feed = String::new();
    for line in input.lines() {
        let (left, right) = line.split_once(" -> ").expect("should find an arrow");
        let right = right.split(", ").collect::<Vec<_>>();

        if right.contains(&"rx") {
            feed = left[1..].to_string();
        }
    }

    let mut seen: HashMap<String, usize> = HashMap::new();
    for line in input.lines() {
        let (left, right) = line.split_once(" -> ").expect("should find an arrow");
        let right = right.split(", ").collect::<Vec<_>>();

        if right.contains(&feed.as_str()) {
            seen.insert(left[1..].to_string(), 0);
        }
    }

    'outer: for presses in 1.. {
        let mut pulses: VecDeque<Pulse> = VecDeque::from([Pulse {
            from: "button".to_string(),
            to: "broadcaster".to_string(),
            high: false,
        }]);

        while let Some(pulse) = pulses.pop_front() {
            if pulse.high && seen.contains_key(&pulse.from) && seen[&pulse.from] == 0 {
                seen.insert(pulse.from.to_string(), presses);
            }

            if seen.values().all(|&v| v != 0) {
                break 'outer;
            }

            if let Some(module) = modules.get_mut(&pulse.to) {
                let new_pulses = match module {
                    Module::FlipFlop(ff) => ff.process(&pulse),
                    Module::Conjunction(c) => c.process(&pulse),
                    Module::Broadcaster(b) => b.process(&pulse),
                };
                pulses.extend(new_pulses);
            }
        }
    }

    seen.into_values().fold(1, lcm).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

        let result = process(input);
        assert_eq!(result, "32000000".to_string());
    }

    #[test]
    fn solve2() {
        let input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

        let result = process(input);
        assert_eq!(result, "11687500".to_string());
    }

    #[test]
    fn test_flipflop() {
        let mut ff = FlipFlop {
            name: "a".to_string(),
            on: false,
            outs: vec!["b".to_string(), "c".to_string()],
        };

        assert_eq!(
            ff.process(&Pulse {
                from: "x".to_string(),
                to: "a".to_string(),
                high: true
            }),
            vec![]
        );
        assert!(!ff.on);

        assert_eq!(
            ff.process(&Pulse {
                from: "x".to_string(),
                to: "a".to_string(),
                high: false
            }),
            vec![
                Pulse {
                    from: "a".to_string(),
                    to: "b".to_string(),
                    high: true,
                },
                Pulse {
                    from: "a".to_string(),
                    to: "c".to_string(),
                    high: true
                }
            ]
        );

        assert!(ff.on);
    }

    #[test]
    fn test_conjunction() {
        let mut c = Conjunction {
            name: "a".to_string(),
            state: HashMap::from([("b".to_string(), false), ("c".to_string(), false)]),
            outs: vec!["d".to_string()],
        };

        assert_eq!(
            c.process(&Pulse {
                from: "b".to_string(),
                to: "a".to_string(),
                high: true
            }),
            vec![Pulse {
                from: "a".to_string(),
                to: "d".to_string(),
                high: true,
            }]
        );
        assert_eq!(
            c.state,
            HashMap::from([("b".to_string(), true), ("c".to_string(), false)]),
        );

        assert_eq!(
            c.process(&Pulse {
                from: "c".to_string(),
                to: "a".to_string(),
                high: true
            }),
            vec![Pulse {
                from: "a".to_string(),
                to: "d".to_string(),
                high: false,
            }]
        );
        assert_eq!(
            c.state,
            HashMap::from([("b".to_string(), true), ("c".to_string(), true)]),
        );
    }
}
