use std::collections::{BinaryHeap, HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");
    let answer = process(input);
    dbg!(answer);
}

fn process(input: &str) -> String {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("should be a number"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // (hl, r, c, dr, dc, n)
    let mut heap = BinaryHeap::from([(0isize, 0usize, 0usize, 0isize, 0isize, 0)]);
    // (r, c, dr, dc, n)
    let mut seen = HashSet::new();

    while let Some((hl, r, c, dr, dc, n)) = heap.pop() {
        if seen.contains(&(r, c, dr, dc, n)) {
            continue;
        }
        seen.insert((r, c, dr, dc, n));

        if r == grid.len() - 1 && c == grid[0].len() - 1 {
            return hl.abs().to_string();
        }

        for (ndr, ndc) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            if (ndr, ndc) == (-dr, -dc) {
                continue;
            }

            if (ndr, ndc) != (dr, dc) && (dr, dc) != (0, 0) && n < 4 {
                continue;
            }

            let nn = if (ndr, ndc) == (dr, dc) { n + 1 } else { 1 };

            let nr = r.wrapping_add_signed(ndr);
            let nc = c.wrapping_add_signed(ndc);
            if nr < grid.len() && nc < grid[0].len() && nn <= 10 {
                heap.push((hl - grid[nr][nc] as isize, nr, nc, ndr, ndc, nn));
            }
        }
    }

    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve2() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

        let result = process(input);
        assert_eq!(result, "102".to_string());
    }
}
