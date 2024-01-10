use core::panic;
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
    amount: u32,
    color: String,
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
            if let Some((direction, amount, color)) = line.splitn(3, ' ').collect_tuple() {
                let direction = match direction {
                    "R" => (0, 1),
                    "L" => (0, -1),
                    "D" => (1, 0),
                    "U" => (-1, 0),
                    _ => panic!("invalid direction: {direction}"),
                };
                let amount = amount.parse::<u32>().expect("should be a number");
                let color = color[2..color.len() - 1].to_string();

                Instruction {
                    direction,
                    amount,
                    color,
                }
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

    let padding = coords.iter().fold((0, 0), |acc, coord| {
        (acc.0.min(coord.0), acc.1.min(coord.1))
    });

    let coords = coords
        .iter()
        .map(|coord| (coord.0 + padding.0.abs(), coord.1 + padding.1.abs()))
        .collect::<Vec<_>>();

    println!("{coords:?}");

    let (height, width) = coords
        .iter()
        .fold((0, 0), |acc, c| (acc.0.max(c.0 + 3), acc.1.max(c.1 + 3)));
    println!("{height:?},{width:?}");

    let mut grid: Vec<Vec<char>> = (0..height)
        .map(|_| (0..width).map(|_| '.').collect())
        .collect();

    let (i, j) = coords[coords.len() - 1];
    let mut p = (i + 1, j + 1);
    for instruction in instructions.iter() {
        for _ in 0..instruction.amount {
            p = (p.0 + instruction.direction.0, p.1 + instruction.direction.1);
            grid[p.0 as usize][p.1 as usize] = '#';
        }
    }

    for line in grid.iter() {
        println!("{}", line.iter().collect::<String>());
    }

    flood_fill(&mut grid);
    for line in grid.iter() {
        println!("{}", line.iter().collect::<String>());
    }

    grid.iter()
        .flatten()
        .filter(|&tile| *tile == '#' || *tile == '.')
        .count()
        .to_string()
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
        assert_eq!(result, "62".to_string());
    }
}
