use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = include_str!("input.txt");
    let answer = process(input);
    dbg!(answer);
}

fn build_graph(
    node: (usize, usize),
    prev: (usize, usize),
    grid: &[Vec<char>],
    graph: &mut HashMap<(usize, usize), HashSet<(usize, usize)>>,
) {
    let (i, j) = node;
    let c = grid[i][j];

    let dirs = match c {
        '>' => vec![(0, 1)],
        '<' => vec![(0, -1)],
        'v' => vec![(1, 0)],
        '^' => vec![(-1, 0)],
        _ => vec![(1, 0), (-1, 0), (0, 1), (0, -1)],
    };

    for (di, dj) in dirs {
        let (ni, nj) = (i.wrapping_add_signed(di), j.wrapping_add_signed(dj));
        if ni < grid.len() && nj < grid[0].len() && (ni, nj) != prev {
            let c = grid[ni][nj];

            if c == '.'
                || ((di, dj) == (0, 1) && c == '>')
                || ((di, dj) == (0, -1) && c == '<')
                || ((di, dj) == (1, 0) && c == 'v')
                || ((di, dj) == (-1, 0) && c == '^')
            {
                build_graph((ni, nj), node, grid, graph);
                graph.entry(node).or_default().insert((ni, nj));
            }
        }
    }
}

fn dfs(
    node: (usize, usize),
    graph: &HashMap<(usize, usize), HashSet<(usize, usize)>>,
    topsort: &mut VecDeque<(usize, usize)>,
) {
    for n in graph[&node].iter() {
        dfs(*n, graph, topsort);
    }

    if !topsort.contains(&node) {
        topsort.push_front(node)
    }
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

    let mut graph = HashMap::from([
        (start, HashSet::from([(start.0 + 1, start.1)])),
        (end, HashSet::new()),
    ]);
    build_graph((start.0 + 1, start.1), start, &grid, &mut graph);

    let mut topsort = VecDeque::new();
    dfs(start, &graph, &mut topsort);

    let mut dist: Vec<_> = (0..topsort.len()).map(|_| isize::MAX).collect();
    dist[0] = 0;
    for (i, pos) in topsort.iter().enumerate() {
        for n in graph[pos].iter() {
            let j = topsort
                .iter()
                .position(|p| p == n)
                .expect("node should exist on topsort");

            if dist[i] - 1 < dist[j] {
                dist[j] = dist[i] - 1;
            }
        }
    }

    let e = topsort
        .iter()
        .position(|&p| p == end)
        .expect("end node should exist on topsort");

    dist[e].abs().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
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
        assert_eq!(result, "94".to_string());
    }
}
