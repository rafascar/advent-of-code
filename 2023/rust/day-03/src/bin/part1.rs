use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    let answer = process(input);
    dbg!(answer);
}

fn process(input: &str) -> String {
    let re_number = Regex::new(r"\d+").unwrap();
    let re_symbol = Regex::new(r"[^.\d]").unwrap();
    let mut answer = 0;

    let lines: Vec<&str> = input.lines().collect();

    for (i, line) in lines.iter().enumerate() {
        let matches = re_number.find_iter(line);
        for m in matches {
            let x0 = m.start().saturating_sub(1);
            let x1 = m.end().min(line.len() - 1);

            let i0 = i.saturating_sub(1);
            let i1 = (i + 1).min(lines.len() - 1);

            for i in i0..=i1 {
                let window = &lines[i][x0..=x1];
                if re_symbol.is_match(window) {
                    answer += m.as_str().parse::<usize>().unwrap();
                    break;
                }
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
        assert_eq!(result, "4361".to_string());
    }
}
