use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let answer = process(input);
    dbg!(answer);
}

struct Record<'a>(&'a str, &'a str);

fn encode(input: &str) -> String {
    input
        .split('.')
        .filter(|&s| !s.is_empty())
        .map(|s| s.len().to_string())
        .join(",")
}

fn process(input: &str) -> String {
    let records = input.lines().map(|line| {
        let (value, encoded) = line.split_once(' ').unwrap();
        Record(value, encoded)
    });

    let mut answer = 0;
    for record in records {
        let indices = record
            .0
            .char_indices()
            .filter(|(_, c)| *c == '?')
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();

        for n in 0..2_usize.pow(indices.len() as u32) {
            let mut attempt = record.0.clone().chars().collect::<Vec<char>>();
            for (i, index) in indices.iter().enumerate() {
                attempt[*index] = if n >> i & 1 == 0 { '.' } else { '?' };
            }

            let attempt = attempt.into_iter().collect::<String>();
            if encode(&attempt) == record.1 {
                answer += 1;
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
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        let result = process(input);
        assert_eq!(result, "21".to_string());
    }

    #[test]
    fn test_encode() {
        assert_eq!(encode("#.#.###"), "1,1,3".to_string());
        assert_eq!(encode("####.#...#..."), "4,1,1".to_string());
    }
}
