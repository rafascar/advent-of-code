use core::panic;

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

type Color = String;

#[derive(Debug)]
enum Tile {
    Dirt,
    Hole(Color),
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

    let width = instructions
        .iter()
        .filter(|i| i.direction.0 == 0)
        .scan(1, |state, i| {
            *state += i.direction.1 * i.amount as i32;
            Some(*state)
        })
        .max()
        .unwrap_or(0);

    let height = instructions
        .iter()
        .filter(|i| i.direction.1 == 0)
        .scan(1, |state, i| {
            *state += i.direction.0 * i.amount as i32;
            Some(*state)
        })
        .max()
        .unwrap_or(0);

    let mut grid: Vec<Vec<char>> = (0..height)
        .map(|_| (0..width).map(|_| '.').collect())
        .collect();

    let mut p = (0usize, 0usize);
    for instruction in instructions.iter() {
        for _ in 0..instruction.amount {
            p = (
                p.0.wrapping_add_signed(instruction.direction.0 as isize),
                p.1.wrapping_add_signed(instruction.direction.1 as isize),
            );
            grid[p.0][p.1] = '#';
        }
    }

    for line in grid.iter_mut() {
        let mut holes = 0;
        let mut seen = false;

        for tile in line.iter_mut() {
            if *tile == '#' && !seen {
                seen = true;
                holes += 1;
            } else if *tile == '.' {
                if holes % 2 != 0 {
                    *tile = '#';
                }
                seen = false;
            }
        }
    }

    grid.iter()
        .flatten()
        .filter(|&tile| *tile == '#')
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
