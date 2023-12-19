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
    let lines = block.lines().collect::<Vec<&str>>();

    for r in 1..lines.len() {
        let mut diff = 0;
        for (i, j) in (0..r).rev().zip(r..) {
            diff += lines[i]
                .chars()
                .zip(lines[j].chars())
                .filter(|(a, b)| a != b)
                .count();
        }

        if diff == 1 {
            return Some(r);
        }
    }

    None
}

fn process(input: &str) -> String {
    let blocks = input.split("\n\n").collect::<Vec<_>>();

    let mut answer = 0;
    for block in blocks {
        if let Some(n) = find_mirror(block) {
            answer += 100 * n;
        } else if let Some(n) = find_mirror(&transpose(block)) {
            answer += n;
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
        assert_eq!(result, "400".to_string());
    }
}
