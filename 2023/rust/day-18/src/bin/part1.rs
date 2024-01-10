use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let answer = process(input);
    dbg!(answer);
}

#[derive(Debug)]
struct Instruction {
    direction: (i32, i32),
    amount: u64,
}

fn flood_fill(grid: &mut Vec<Vec<char>>) {
    let width = grid[0].len();
    let height = grid.len();

    let mut frontier = VecDeque::from([(0, 0)]);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    while let Some((i, j)) = frontier.pop_back() {
        visited.insert((i, j));

        let tile = &mut grid[i][j];
        if *tile == '#' {
            continue;
        } else if *tile == '.' {
            *tile = 'x';
        }

        for (di, dj) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let ni = i.wrapping_add_signed(di);
            let nj = j.wrapping_add_signed(dj);

            if ni < height && nj < width && !visited.contains(&(ni, nj)) {
                frontier.push_back((ni, nj));
            }
        }
    }
}

fn process(input: &str) -> String {
    let instructions = input
        .lines()
        .map(|line| {
            if let Some((_, _, color)) = line.splitn(3, ' ').collect_tuple() {
                let color = color[2..color.len() - 1].to_string();
                let direction = match color.chars().last().expect("should not be empty") {
                    '0' => (0, 1),
                    '1' => (1, 0),
                    '2' => (0, -1),
                    '3' => (-1, 0),
                    _ => panic!("invalid direction: {color}"),
                };
                let amount = u64::from_str_radix(&color[0..color.len() - 1], 16)
                    .expect("should be a valid hex");

                Instruction { direction, amount }
            } else {
                panic!("Expected three elements")
            }
        })
        .collect::<Vec<Instruction>>();

    let coords = instructions
        .iter()
        .flat_map(|i| std::iter::repeat(i.direction).take(i.amount as usize))
        .scan((0, 0), |state, dir| {
            *state = (state.0 + dir.0, state.1 + dir.1);
            Some(*state)
        })
        .collect::<Vec<_>>();

    // Shoelace Formula
    // https://en.wikipedia.org/wiki/Shoelace_formula
    let area = (coords.windows(2).fold(0isize, |acc, p| {
        acc + p[0].0 as isize * p[1].1 as isize - p[1].0 as isize * p[0].1 as isize
    }) / 2)
        .abs();

    // Pick's Theorem
    // https://en.wikipedia.org/wiki/Pick%27s_theorem
    (area as f64 - 0.5 * coords.len() as f64 + 1.0 + coords.len() as f64).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

        let result = process(input);
        assert_eq!(result, "952408144115".to_string());
    }
}
