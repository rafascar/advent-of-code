use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let answer = process(input);
    dbg!(answer);
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Beam {
    pos: (usize, usize),
    dir: Dir,
}

#[derive(Debug)]
enum Tile {
    Empty,
    Mirror(char),
    Splitter(char),
}

impl TryFrom<char> for Tile {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Tile::Empty),
            '/' | '\\' => Ok(Tile::Mirror(value)),
            '-' | '|' => Ok(Tile::Splitter(value)),
            _ => Err(format!("invalid char {value}")),
        }
    }
}

struct Grid(Vec<Vec<Tile>>);

impl Grid {
    fn update(&self, beam: Beam) -> Vec<Beam> {
        let (i, j) = beam.pos;
        let tile = &self.0[i][j];

        match tile {
            Tile::Empty => vec![beam.dir],
            Tile::Mirror(c) => match c {
                '/' if beam.dir == Dir::Left => vec![Dir::Down],
                '/' if beam.dir == Dir::Right => vec![Dir::Up],
                '/' if beam.dir == Dir::Up => vec![Dir::Right],
                '/' if beam.dir == Dir::Down => vec![Dir::Left],
                '\\' if beam.dir == Dir::Left => vec![Dir::Up],
                '\\' if beam.dir == Dir::Right => vec![Dir::Down],
                '\\' if beam.dir == Dir::Up => vec![Dir::Left],
                '\\' if beam.dir == Dir::Down => vec![Dir::Right],
                _ => panic!("should not happen"),
            },
            Tile::Splitter(c) => match c {
                '-' if (beam.dir == Dir::Up || beam.dir == Dir::Down) => {
                    vec![Dir::Right, Dir::Left]
                }
                '|' if (beam.dir == Dir::Left || beam.dir == Dir::Right) => {
                    vec![Dir::Up, Dir::Down]
                }
                _ => vec![beam.dir],
            },
        }
        .iter()
        .filter_map(|&dir| match dir {
            Dir::Up if i > 0 => Some(Beam {
                pos: (i - 1, j),
                dir,
            }),
            Dir::Down if i < self.height() - 1 => Some(Beam {
                pos: (i + 1, j),
                dir,
            }),
            Dir::Left if j > 0 => Some(Beam {
                pos: (i, j - 1),
                dir,
            }),
            Dir::Right if j < self.width() - 1 => Some(Beam {
                pos: (i, j + 1),
                dir,
            }),
            _ => None,
        })
        .collect()
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn height(&self) -> usize {
        self.0.len()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Dir {
    Up,
    Down,
    Right,
    Left,
}

fn process(input: &str) -> String {
    let grid = Grid(
        input
            .lines()
            .map(|line| line.chars().map(|c| Tile::try_from(c).unwrap()).collect())
            .collect(),
    );

    let origin = Beam {
        pos: (0, 0),
        dir: Dir::Right,
    };
    let mut beams = VecDeque::from([origin]);
    let mut visited: HashSet<Beam> = HashSet::from([origin]);

    while !beams.is_empty() {
        let beam = beams.pop_front().expect("should not be empty");
        let new_beams = grid.update(beam);
        for b in new_beams {
            if !visited.contains(&b) {
                visited.insert(b);
                beams.push_back(b);
            }
        }
    }

    visited
        .iter()
        .unique_by(|beam| beam.pos)
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

        let result = process(input);
        assert_eq!(result, "46".to_string());
    }
}
