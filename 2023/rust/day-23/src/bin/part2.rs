use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let answer = process(input);
    dbg!(answer);
}

fn get_neighbors(
    node: (usize, usize),
    path: &HashSet<(usize, usize)>,
    grid: &[Vec<char>],
) -> Vec<(usize, usize)> {
    let (i, j) = node;
    [(1, 0), (-1, 0), (0, 1), (0, -1)]
        .iter()
        .filter_map(|&(di, dj)| {
            let (ni, nj) = (i.wrapping_add_signed(di), j.wrapping_add_signed(dj));
            if ni < grid.len()
                && nj < grid[0].len()
                && grid[ni][nj] != '#'
                && !path.contains(&(ni, nj))
            {
                Some((ni, nj))
            } else {
                None
            }
        })
        .collect()
}

fn bfs(
    node: (usize, usize),
    path: &mut HashSet<(usize, usize)>,
    grid: &[Vec<char>],
    end: (usize, usize),
    longest: &mut usize,
) {
    let neighbors = get_neighbors(node, path, grid);
    if neighbors.is_empty() && node == end {
        let curr_length = path.len();
        if curr_length > *longest {
            *longest = curr_length;
        }
        return;
    }

    path.insert(node);

    for neighbor in neighbors {
        bfs(neighbor, path, grid, end, longest);
    }

    path.remove(&node);
}

fn process(input: &str) -> String {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let start = (
        0,
        grid[0]
            .iter()
            .position(|&c| c == '.')
            .expect("first line should have a '.'"),
    );

    let end = (
        grid.len() - 1,
        grid[grid.len() - 1]
            .iter()
            .position(|&c| c == '.')
            .expect("first last should have a '.'"),
    );

    let mut path = HashSet::new();
    let mut longest = 0;
    bfs(start, &mut path, &grid, end, &mut longest);

    longest.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve2() {
        let input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

        let result = process(input);
        assert_eq!(result, "154".to_string());
    }

    #[test]
    fn solve3() {
        let input = "#.###
#...#
#.#.#
#...#
###.#";

        let result = process(input);
        assert_eq!(result, "6".to_string());
    }
}
