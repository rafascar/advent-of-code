use std::iter::zip;

fn main() {
    let input = include_str!("input.txt");
    let answer = process(input);
    dbg!(answer);
}

fn process(input: &str) -> String {
    let histories = input.lines().map(|line| {
        line.split(' ')
            .map(|n| n.parse::<isize>().expect("should be a number"))
    });

    let mut answer = 0;
    for history in histories {
        let mut curr = history.collect::<Vec<isize>>();
        let mut diffs: Vec<isize>;

        loop {
            answer += curr.last().expect("should not be empty");

            diffs = zip(curr.iter(), curr.iter().skip(1))
                .map(|(a, b)| b - a)
                .collect();

            if diffs.iter().all(|&n| n == 0) {
                break;
            } else {
                curr = diffs;
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
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        let result = process(input);
        assert_eq!(result, "114".to_string());
    }
}
