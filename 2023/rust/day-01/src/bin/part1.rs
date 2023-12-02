fn main() {
    let input = include_str!("input.txt");
    let answer = process(input);
    dbg!(answer);
}

fn process(input: &str) -> String {
    let mut answer = 0;
    for line in input.lines() {
        let mut numerics = line.matches(char::is_numeric);

        let (first, last) = match (numerics.next(), numerics.last()) {
            (Some(first), Some(last)) => (first, last),
            (Some(first), None) => (first, first),
            _ => panic!("no numbers on line"),
        };

        answer += format!("{}{}", first, last).parse::<usize>().unwrap();
    }
    answer.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        let result = process(input);
        assert_eq!(result, "142".to_string());
    }
}
