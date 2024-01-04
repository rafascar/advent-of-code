use std::collections::{BinaryHeap, HashMap};

fn main() {
    let input = include_str!("input.txt");
    let answer = process(input);
    dbg!(answer);
}

struct Grid(Vec<Vec<u32>>);

impl Grid {
    fn neighbors(&self, pos: Position) -> Vec<Position> {
        [
            (pos.0.wrapping_add(1), pos.1),
            (pos.0.wrapping_sub(1), pos.1),
            (pos.0, pos.1.wrapping_add(1)),
            (pos.0, pos.1.wrapping_sub(1)),
        ]
        .into_iter()
        .filter(|&(i, j)| i < self.height() && j < self.width())
        .collect()
    }

    fn cost(&self, pos: Position) -> u32 {
        self.0[pos.0][pos.1]
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn start(&self) -> Position {
        (0, 0)
    }

    fn end(&self) -> Position {
        (self.height() - 1, self.width() - 1)
    }
}

type Position = (usize, usize);

#[derive(Debug, Eq, PartialEq, Clone)]
struct State {
    cost: u32,
    position: Position,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn process(input: &str) -> String {
    let grid = Grid(
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).expect("should be a digit"))
                    .collect()
            })
            .collect(),
    );

    let mut frontier: BinaryHeap<State> = BinaryHeap::new();
    frontier.push(State {
        cost: 0,
        position: grid.start(),
    });

    let mut cost_so_far: HashMap<Position, u32> = HashMap::new();
    let mut came_from: HashMap<Position, Position> = HashMap::new();

    while let Some(current) = frontier.pop() {
        if current.position == grid.end() {
            break;
        }

        for next in grid.neighbors(current.position) {
            let new_cost = current.cost + grid.cost(next);
            if new_cost < *cost_so_far.get(&next).unwrap_or(&u32::MAX) {
                cost_so_far.insert(next, new_cost);
                frontier.push(State {
                    cost: new_cost,
                    position: next,
                });
                came_from.insert(next, current.position);
            }
        }
    }

    let mut path = vec![];
    let mut current = grid.end();
    while current != grid.start() {
        path.push(current);
        current = *came_from.get(&current).expect("should exist");
    }
    println!("{:?}", path.iter().rev().collect::<Vec<_>>());

    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
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
