use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let answer = process(input);
    dbg!(answer);
}

#[derive(Debug, Clone, Copy)]
struct Galaxy(isize, isize);

impl Galaxy {
    fn distance(&self, other: &Galaxy) -> isize {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

fn expand(v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut expanded: Vec<Vec<char>> = vec![];

    for row in &v {
        expanded.push(row.clone());
        if row.iter().all(|&c| c == '.') {
            expanded.push(row.clone());
        }
    }

    expanded
}

fn transpose(v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = v.len();
    let cols = v[0].len();

    (0..cols)
        .map(|col| (0..rows).map(|row| v[row][col]).collect())
        .collect()
}

fn process(input: &str) -> String {
    let universe: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let expanded = transpose(expand(transpose(expand(universe))));

    let galaxies = expanded.iter().enumerate().flat_map(|(r, row)| {
        row.iter().enumerate().filter_map(move |(c, &ch)| {
            if ch == '#' {
                Some(Galaxy(r as isize, c as isize))
            } else {
                None
            }
        })
    });

    galaxies
        .combinations(2)
        .fold(0, |acc, gs| acc + gs[0].distance(&gs[1]))
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let result = process(input);
        assert_eq!(result, "374".to_string());
    }

    #[test]
    fn test_distance() {
        let g1 = Galaxy(4, 0);
        let g2 = Galaxy(9, 1);
        assert_eq!(g1.distance(&g2), 6);
    }
}
