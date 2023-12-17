use std::collections::{BTreeMap, HashSet};

fn main() {
    let input = include_str!("input.txt");
    let answer = process(input);
    dbg!(answer);
}

fn transpose(block: &str) -> String {
    let v: Vec<Vec<char>> = block.lines().map(|line| line.chars().collect()).collect();
    let rows = v.len();
    let cols = v[0].len();

    (0..cols)
        .map(|col| (0..rows).map(|row| v[row][col]).collect())
        .collect::<Vec<String>>()
        .join("\n")
}

fn find_mirror(block: &str) -> Option<usize> {
    let mut matches: BTreeMap<usize, HashSet<(usize, usize)>> = BTreeMap::new();

    for (l, line) in block.lines().enumerate() {
        matches.insert(l, HashSet::new());

        for i in 1..line.len() {
            let (first, last) = line.split_at(i);
            let first_rev = first.chars().rev().collect::<String>();

            let n = first_rev
                .chars()
                .zip(last.chars())
                .take_while(|(c1, c2)| c1 == c2)
                .count();

            if n > 0 {
                matches.get_mut(&l).unwrap().insert((i, n));
            }
        }
    }

    Some(
        matches
            .values()
            .cloned()
            .reduce(|acc, m| acc.intersection(&m).cloned().collect::<HashSet<_>>())?
            .iter()
            .next()?
            .0,
    )
}

fn process(input: &str) -> String {
    let blocks = input.split("\n\n").collect::<Vec<_>>();

    let mut answer = 0;
    for block in blocks {
        if let Some(n) = find_mirror(block) {
            answer += n;
        } else if let Some(n) = find_mirror(&transpose(block)) {
            answer += 100 * n;
        } else {
            panic!("did not find any mirror");
        }
    }

    answer.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        let result = process(input);
        assert_eq!(result, "405".to_string());
    }
}
