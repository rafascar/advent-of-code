fn main() {
    let input = include_str!("input.txt");
    let answer = part1(input);
    dbg!(answer);
}

fn part1(input: &str) -> String {
    let mut numbers = vec![];
    for line in input.lines() {
        let first = line.find(|c: char| c.is_numeric()).unwrap();
        let last = line.rfind(|c: char| c.is_numeric()).unwrap();

        let number: usize = format!(
            "{}{}",
            line.chars().nth(first).unwrap(),
            line.chars().nth(last).unwrap()
        )
        .parse()
        .unwrap();
        numbers.push(number);
    }

    numbers.into_iter().sum::<usize>().to_string()
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

        let result = part1(input);
        assert_eq!(result, "142".to_string());
    }
}
