fn main() {
    let input = include_str!("input.txt");
    let answer = process(input);
    dbg!(answer);
}

#[derive(Debug)]
struct Pipe {
    label: char,
    coords: (usize, usize),
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Pipe {
    fn connects_to(&self, other: &Pipe, dir: Dir) -> bool {
        match self.label {
            '|' => match dir {
                Dir::N => "S|7F".contains(other.label),
                Dir::S => "S|LJ".contains(other.label),
                _ => false,
            },
            '-' => match dir {
                Dir::E => "S-J7".contains(other.label),
                Dir::W => "S-LF".contains(other.label),
                _ => false,
            },
            'L' => match dir {
                Dir::N => "S|7F".contains(other.label),
                Dir::E => "S-J7".contains(other.label),
                _ => false,
            },
            'J' => match dir {
                Dir::N => "S|7F".contains(other.label),
                Dir::W => "S-LF".contains(other.label),
                _ => false,
            },
            '7' => match dir {
                Dir::S => "S|LJ".contains(other.label),
                Dir::W => "S-LF".contains(other.label),
                _ => false,
            },
            'F' => match dir {
                Dir::E => "S-J7".contains(other.label),
                Dir::S => "S|LJ".contains(other.label),
                _ => false,
            },
            '.' => false,
            'S' => match dir {
                Dir::N => "S|7F".contains(other.label),
                Dir::S => "S|LJ".contains(other.label),
                Dir::E => "S-J7".contains(other.label),
                Dir::W => "S-LF".contains(other.label),
            },
            _ => panic!("invalid pipe"),
        }
    }
}

trait CoordsExt {
    fn apply_dir(&self, dir: &Dir) -> (Option<usize>, Option<usize>);
}

impl CoordsExt for (usize, usize) {
    fn apply_dir(&self, dir: &Dir) -> (Option<usize>, Option<usize>) {
        match dir {
            Dir::N => (Some(self.0), self.1.checked_sub(1)),
            Dir::E => (Some(self.0 + 1), Some(self.1)),
            Dir::S => (Some(self.0), Some(self.1 + 1)),
            Dir::W => (self.0.checked_sub(1), Some(self.1)),
        }
    }
}

fn find_connected<'a>(pipes: &'a Vec<Vec<Pipe>>, p: &Pipe) -> (&'a Pipe, &'a Pipe) {
    let mut connected = vec![];
    for d in [Dir::N, Dir::E, Dir::S, Dir::W] {
        if let (Some(x), Some(y)) = p.coords.apply_dir(&d) {
            if x < pipes[0].len() && y < pipes.len() && p.connects_to(&pipes[y][x], d) {
                connected.push(&pipes[y][x]);
            }
        }
    }

    (connected[0], connected[1])
}

fn process(input: &str) -> isize {
    let pipes: Vec<Vec<Pipe>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, label)| Pipe {
                    label,
                    coords: (x, y),
                })
                .collect()
        })
        .collect();

    let s = pipes
        .iter()
        .find_map(|row| row.iter().find(|pipe| pipe.label == 'S'))
        .expect("should find S");

    let mut points = vec![s];
    let mut p1 = s;
    let (mut s1, _) = find_connected(&pipes, s);
    while s1.label != 'S' {
        let (s11, s12) = find_connected(&pipes, s1);

        (p1, s1) = if s11.coords != p1.coords {
            (s1, s11)
        } else {
            (s1, s12)
        };

        points.push(s1);
    }

    // Shoelace Formula
    // https://en.wikipedia.org/wiki/Shoelace_formula
    let area = (points.windows(2).fold(0isize, |acc, p| {
        acc + p[0].coords.0 as isize * p[1].coords.1 as isize
            - p[1].coords.0 as isize * p[0].coords.1 as isize
    }) / 2)
        .abs();

    // Pick's Theorem
    // https://en.wikipedia.org/wiki/Pick%27s_theorem
    (area as f64 - 0.5 * points.len() as f64 + 1.0) as isize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

        let result = process(input);
        assert_eq!(result, 1);
    }

    #[test]
    fn example2() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

        let result = process(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn example3() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

        let result = process(input);
        assert_eq!(result, 10);
    }
}
