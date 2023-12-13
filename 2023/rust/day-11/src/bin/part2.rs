use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let answer = process(input);
    dbg!(answer);
}

#[derive(Debug, Clone, Copy)]
struct Galaxy(isize, isize);

impl Galaxy {
    fn distance(&self, other: &Galaxy, universe: &[Vec<char>]) -> usize {
        let d = (self.0 - other.0).abs() + (self.1 - other.1).abs();

        let (rmin, rmax) = (self.0.min(other.0), self.0.max(other.0));
        let (cmin, cmax) = (self.1.min(other.1), self.1.max(other.1));

        let r = (rmin..rmax)
            .filter(|r| universe[*r as usize][0] == '*')
            .count();
        let c = (cmin..cmax)
            .filter(|c| universe[0][*c as usize] == '*')
            .count();

        d as usize + (r + c) * 1_000_000 - (r + c)
    }
}

fn expand(v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut expanded: Vec<Vec<char>> = vec![];

    for row in &v {
        if !row.iter().any(|&c| c == '#') {
            let mut new_row = row.to_vec();
            new_row.fill('*');
            expanded.push(new_row);
        } else {
            expanded.push(row.to_vec());
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

    for line in &expanded {
        println!("{}", line.iter().join(""));
    }

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
        .fold(0, |acc, gs| acc + gs[0].distance(&gs[1], &expanded))
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
        assert_eq!(result, "82000210".to_string());
    }
}
