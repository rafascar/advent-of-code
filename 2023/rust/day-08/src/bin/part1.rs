use core::fmt;
use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, newline},
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

fn main() {
    let input = include_str!("input.txt");
    let answer = process(input);
    dbg!(answer);
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Node<'a> {
    label: &'a str,
    left: &'a str,
    right: &'a str,
}

impl Node<'_> {
    fn get(&self, cmd: &Command) -> &'_ str {
        match cmd {
            Command::Left => self.left,
            Command::Right => self.right,
        }
    }
}

#[derive(Debug)]
enum Command {
    Left,
    Right,
}

impl fmt::Display for Node<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}  = ({}, {})", self.label, self.left, self.right)
    }
}

fn parse_commands(i: &str) -> IResult<&str, Vec<Command>> {
    let (i, commands) = terminated(alphanumeric1, newline)(i)?;

    let commands = commands
        .chars()
        .map(|c| match c {
            'L' => Command::Left,
            'R' => Command::Right,
            _ => panic!("invalid command: {c}"),
        })
        .collect();

    Ok((i, commands))
}

fn parse_node(i: &str) -> IResult<&str, Node<'_>> {
    let (i, label) = terminated(alphanumeric1, tag(" = "))(i)?;
    let (i, (left, right)) = delimited(
        tag("("),
        separated_pair(alphanumeric1, tag(", "), alphanumeric1),
        tag(")"),
    )(i)?;

    Ok((i, Node { label, left, right }))
}

fn parse(i: &str) -> IResult<&str, (Vec<Command>, HashMap<String, Node>)> {
    let (i, commands) = terminated(parse_commands, newline)(i)?;
    let (i, nodes) = separated_list1(newline, parse_node)(i)?;

    let nodes = nodes
        .iter()
        .map(|&node| (node.label.to_string(), node))
        .collect();
    Ok((i, (commands, nodes)))
}

fn process(input: &str) -> Option<String> {
    let (_, (commands, nodes)) = parse(input).expect("should be able to parse");

    let mut node = nodes["AAA"];
    for (i, cmd) in commands.iter().cycle().enumerate() {
        if node.label == "ZZZ" {
            return Some(i.to_string());
        }

        node = nodes[node.get(cmd)];
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        let result = process(input);
        assert_eq!(result, Some("2".to_string()));
    }

    #[test]
    fn example2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        let result = process(input);
        assert_eq!(result, Some("6".to_string()));
    }

    #[test]
    fn test_parse_node() {
        let i = "AAA = (BBB, CCC)";
        assert_eq!(
            parse_node(i),
            Ok((
                "",
                Node {
                    label: "AAA",
                    left: "BBB",
                    right: "CCC"
                }
            ))
        );
    }
}
