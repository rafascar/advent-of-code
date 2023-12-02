fn main() {
    let input = include_str!("input.txt");
    let answer = process(input);
    dbg!(answer);
}

const DIGIT_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn parse_digits(digit: &str) -> usize {
    if let Some(i) = DIGIT_WORDS.iter().position(|&d| d == digit) {
        i + 1
    } else {
        digit.parse().expect("not a digit")
    }
}

fn process(input: &str) -> String {
    let mut answer = 0;
    for line in input.lines() {
        let mut matches = vec![];

        for pat in DIGIT_WORDS {
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

        answer += format!("{}{}", parse_digits(first), parse_digits(last))
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
