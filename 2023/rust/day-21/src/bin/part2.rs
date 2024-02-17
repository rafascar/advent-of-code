use std::collections::{HashMap, HashSet, VecDeque};

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

    println!("{start:?}");

    let mut visited: HashMap<(usize, usize), usize> = HashMap::from([(start, 0)]);
    let mut queue = VecDeque::from([(start, 0)]);

    while let Some(((i, j), n)) = queue.pop_front() {
        for (di, dj) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let (ni, nj) = (i.wrapping_add_signed(di), j.wrapping_add_signed(dj));
            if ni < grid.len()
                && nj < grid[0].len()
                && ['.', 'S'].contains(&grid[ni][nj])
                && !visited.contains_key(&(ni, nj))
            {
                visited.insert((ni, nj), n + 1);
                queue.push_back(((ni, nj), n + 1));
            }
        }
    }

    // Expl.: https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
    let even_corners = visited
        .values()
        .filter(|v| **v % 2 == 0 && **v > 65)
        .count();
    let odd_corners = visited
        .values()
        .filter(|v| **v % 2 == 1 && **v > 65)
        .count();

    let even_full = visited.values().filter(|v| **v % 2 == 0).count();
    let odd_full = visited.values().filter(|v| **v % 2 == 1).count();

    let n = (26501365 - (grid.len() / 2)) / grid.len();
    assert_eq!(n, 202300);

    let p2 = ((n + 1) * (n + 1)) * odd_full + (n * n) * even_full - (n + 1) * odd_corners
        + n * even_corners;

    p2.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve2() {
        let input = include_str!("input.txt");
        let result = process(input);
        assert_eq!(result, "609298746763952".to_string());
    }
}
