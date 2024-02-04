use core::fmt;
use std::fmt::Display;

use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let answer = process(input);
    dbg!(answer);
}

#[derive(Debug)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl From<&str> for Vec3 {
    fn from(s: &str) -> Self {
        let (x, y, z) = s.splitn(3, ',').collect_tuple().expect("should be x,y,z");
        Vec3 {
            x: x.trim().parse().expect("'x' should be a number"),
            y: y.trim().parse().expect("'y' should be a number"),
            z: z.trim().parse().expect("'z' should be a number"),
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.x, self.y, self.z)
    }
}

#[derive(Debug)]
struct Hailstone {
    position: Vec3,
    velocity: Vec3,
}

impl From<&str> for Hailstone {
    fn from(s: &str) -> Self {
        let (p, v) = s.split_once('@').expect("'@' should exist");
        Hailstone {
            position: p.into(),
            velocity: v.into(),
        }
    }
}

impl Hailstone {
    fn intersects_at_2d(&self, other: &Self) -> Option<Vec3> {
        // a*x + b*y + c = 0
        let (a1, b1, c1) = (
            self.velocity.y,
            -self.velocity.x,
            self.velocity.x * self.position.y - self.velocity.y * self.position.x,
        );
        let (a2, b2, c2) = (
            other.velocity.y,
            -other.velocity.x,
            other.velocity.x * other.position.y - other.velocity.y * other.position.x,
        );

        // Cross multiplication rule.
        // https://www.cuemath.com/geometry/intersection-of-two-lines/
        let x = (b1 * c2 - b2 * c1) / (a1 * b2 - a2 * b1);
        let y = (c1 * a2 - c2 * a1) / (a1 * b2 - a2 * b1);

        let i = Vec3 { x, y, z: 0.0 };
        // println!("Hailstone A: {self}, a: {a1} b: {b1} c: {c1}");
        // println!("Hailstone B: {other}, a: {a2} b: {b2} c: {c2}");
        // println!("Intersection: {i}");
        // println!();

        // Check if intersection happens in the future.
        if self.velocity.x * (i.x - self.position.x) > 0.0
            && self.velocity.y * (i.y - self.position.y) > 0.0
            && other.velocity.x * (i.x - other.position.x) > 0.0
            && other.velocity.y * (i.y - other.position.y) > 0.0
        {
            Some(i)
        } else {
            None
        }
    }
}

impl Display for Hailstone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} @ {}", self.position, self.velocity)
    }
}

fn process(input: &str) -> String {
    let hailstones: Vec<Hailstone> = input.lines().map(Hailstone::from).collect();
    hailstones
        .iter()
        .combinations(2)
        .filter(|c| {
            if let Some(v) = c[0].intersects_at_2d(c[1]) {
                v.x >= 7.0 && v.x <= 27.0 && v.y >= 7.0 && v.y <= 27.0
            } else {
                false
            }
        })
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        let input = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

        let result = process(input);
        assert_eq!(result, "2".to_string());
    }
}
