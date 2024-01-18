use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let answer = process(input);
    dbg!(answer);
}

fn process(input: &str) -> String {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let start = grid
        .iter()
        .enumerate()
        .find_map(|(i, line)| line.iter().position(|&c| c == 'S').map(|j| (i, j)))
        .expect("should find the S");

    let mut positions = HashSet::from([start]);

    for _ in 0..64 {
        let mut new_positions: HashSet<(usize, usize)> = HashSet::new();

        for (i, j) in positions.iter() {
            for (di, dj) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let (ni, nj) = (i.wrapping_add_signed(di), j.wrapping_add_signed(dj));
                if ni < grid.len() && nj < grid[0].len() && ['.', 'S'].contains(&grid[ni][nj]) {
                    new_positions.insert((ni, nj));
                }
            }
        }

        positions = new_positions
    }

    positions.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

        let result = process(input);
        assert_eq!(result, "42".to_string());
    }
}
