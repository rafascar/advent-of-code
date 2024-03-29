use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let answer = process(input);
    dbg!(answer);
}

#[derive(Debug, Clone, Copy)]
struct Brick {
    label: char,
    coords: (usize, usize, usize),
    dimensions: (usize, usize, usize),
}

impl Brick {
    fn intersects_with(&self, other: &Brick) -> bool {
        let (xs0, xs1) = (self.coords.0, self.coords.0 + self.dimensions.0);
        let (xe0, xe1) = (other.coords.0, other.coords.0 + other.dimensions.0);

        let (ys0, ys1) = (self.coords.1, self.coords.1 + self.dimensions.1);
        let (ye0, ye1) = (other.coords.1, other.coords.1 + other.dimensions.1);

        xs0 < xe1 && xe0 < xs1 && ys0 < ye1 && ye0 < ys1
    }

    fn supports<'a>(&self, pile: &'a [Brick]) -> Vec<&'a Brick> {
        pile.iter()
            .filter(|other| {
                other.coords.2 == (self.coords.2 + self.dimensions.2) && self.intersects_with(other)
            })
            .collect()
    }

    fn supported_by<'a>(&self, pile: &'a [Brick]) -> Vec<&'a Brick> {
        pile.iter()
            .filter(|other| {
                self.coords.2 == (other.coords.2 + other.dimensions.2)
                    && self.intersects_with(other)
            })
            .collect()
    }
}

fn process(input: &str) -> String {
    let mut bricks = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let (start, end) = line.split_once('~').expect("should be separated by '~'");

            let xyz0 = start
                .splitn(3, ',')
                .map(|n| n.parse::<usize>().expect("should be a number"))
                .collect_tuple::<(usize, usize, usize)>()
                .expect("should be x,y,z");

            let xyz1 = end
                .splitn(3, ',')
                .map(|n| n.parse::<usize>().expect("should be a number"))
                .collect_tuple::<(usize, usize, usize)>()
                .expect("should be x,y,z");

            Brick {
                label: char::from_u32(i as u32 + 65).expect("should be a char"),
                coords: xyz0,
                dimensions: (
                    xyz1.0 - xyz0.0 + 1,
                    xyz1.1 - xyz0.1 + 1,
                    xyz1.2 - xyz0.2 + 1,
                ),
            }
        })
        .collect::<Vec<Brick>>();

    bricks.sort_by_key(|b| b.coords.2);
    bricks.reverse();

    let mut pile: Vec<Brick> = vec![];

    'outer: while let Some(mut brick) = bricks.pop() {
        let mut p = pile.clone();
        p.sort_by_key(|b| b.coords.2 + b.dimensions.2);

        for other in p.iter().rev() {
            if brick.intersects_with(other) {
                brick.coords.2 = other.coords.2 + other.dimensions.2;
                pile.push(brick);
                continue 'outer;
            }
        }

        brick.coords.2 = 1;
        pile.push(brick);
    }

    let mut count = 0;

    for brick in pile.iter() {
        let mut p = pile.clone();
        remove_from_pile(brick, &mut p);

        let pp = p.clone();
        let mut would_fall = pp
            .iter()
            .filter(|b| b.coords.2 > 1 && b.supported_by(&pp).is_empty())
            .cloned()
            .collect::<Vec<Brick>>();

        while !would_fall.is_empty() {
            for b in would_fall.iter() {
                remove_from_pile(b, &mut p);
            }

            let pp = p.clone();
            would_fall = pp
                .iter()
                .filter(|b| b.coords.2 > 1 && b.supported_by(&pp).is_empty())
                .cloned()
                .collect();
        }

        count += pile.len() - p.len() - 1;
    }

    count.to_string()
}

fn remove_from_pile(brick: &Brick, pile: &mut Vec<Brick>) {
    if let Some(i) = pile.iter().position(|b| b.label == brick.label) {
        pile.remove(i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

        let result = process(input);
        assert_eq!(result, "7".to_string());
    }
}
