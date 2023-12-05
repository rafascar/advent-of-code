use std::ops::Range;

use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    let answer = process(input);
    dbg!(answer);
}

#[derive(Debug)]
struct Part {
    number: usize,
    range: Range<usize>,
    line: usize,
}

impl PartialEq for Part {
    fn eq(&self, other: &Self) -> bool {
        self.range == other.range && self.line == other.line
    }
}

fn process(input: &str) -> String {
    let re_number = Regex::new(r"\d+").unwrap();
    let re_symbol = Regex::new(r"\*").unwrap();
    let mut answer = 0;

    let lines: Vec<&str> = input.lines().collect();

    let parts: Vec<Part> = lines
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            re_number.find_iter(line).map(move |m| Part {
                number: m.as_str().parse::<usize>().unwrap(),
                range: m.range(),
                line: i,
            })
        })
        .collect();

    for (i, line) in lines.iter().enumerate() {
        let matches = re_symbol.find_iter(line);
        for m in matches {
            let x0 = m.start().saturating_sub(1);
            let x1 = m.end().min(line.len() - 1);

            let i0 = i.saturating_sub(1);
            let i1 = (i + 1).min(lines.len() - 1);

            let mut answers: Vec<&Part> = Vec::new();
            for i in i0..=i1 {
                for x in x0..=x1 {
                    let parts = parts
                        .iter()
                        .filter(|p| p.line == i && p.range.contains(&x))
                        .collect::<Vec<&Part>>();

                    for part in parts {
                        if !answers.contains(&part) {
                            answers.push(part);
                        }
                    }
                }
            }

            if answers.len() == 2 {
                answer += answers.iter().fold(1, |acc, p| acc * p.number);
            }
        }
    }

    answer.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let result = process(input);
        assert_eq!(result, "467835".to_string());
    }
}
