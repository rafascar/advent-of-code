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

fn process(input: &str) -> usize {
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
    let mut prev = s;
    let (mut s1, _) = find_connected(&pipes, s);
    while s1.label != 'S' {
        let (s11, s12) = find_connected(&pipes, s1);

        (prev, s1) = if s11.coords != prev.coords {
            (s1, s11)
        } else {
            (s1, s12)
        };

        points.push(s1);
    }

    points.len() / 2
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
        assert_eq!(result, 4);
    }

    #[test]
    fn example2() {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

        let result = process(input);
        assert_eq!(result, 8);
    }
}
