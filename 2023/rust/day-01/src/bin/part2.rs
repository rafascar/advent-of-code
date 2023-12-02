fn main() {
    let input = include_str!("input.txt");
    let answer = process(input);
    dbg!(answer);
}

fn parse_numeric(numeric: &str) -> usize {
    match numeric {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        n => n.parse().unwrap(),
    }
}

fn process(input: &str) -> String {
    let mut answer = 0;
    for line in input.lines() {
        let mut matches = vec![];

        for pat in [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ] {
            matches.extend(line.match_indices(pat).collect::<Vec<_>>());
        }
        matches.extend(line.match_indices(char::is_numeric).collect::<Vec<_>>());
        matches.sort_by(|a, b| a.0.cmp(&b.0));

        let mut matches = matches.into_iter();
        let (first, last) = match (matches.next(), matches.last()) {
            (Some(first), Some(last)) => (first.1, last.1),
            (Some(first), None) => (first.1, first.1),
            _ => panic!("no numbers on line"),
        };

        answer += format!("{}{}", parse_numeric(first), parse_numeric(last))
            .parse::<usize>()
            .unwrap();
    }
    answer.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

        let result = process(input);
        assert_eq!(result, "281".to_string());
    }
}
