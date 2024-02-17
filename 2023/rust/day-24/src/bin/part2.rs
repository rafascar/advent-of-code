use core::{fmt, panic};
use std::fmt::Display;

use itertools::Itertools;
use z3::{
    ast::{Ast, Int},
    Config, Context, Solver,
};

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

impl Hailstone {
    fn z3_hailstone<'a>(&self, ctx: &'a Context) -> Z3Hailstone<'a> {
        Z3Hailstone {
            x: Int::from_i64(ctx, self.position.x as i64),
            y: Int::from_i64(ctx, self.position.y as i64),
            z: Int::from_i64(ctx, self.position.z as i64),
            vx: Int::from_i64(ctx, self.velocity.x as i64),
            vy: Int::from_i64(ctx, self.velocity.y as i64),
            vz: Int::from_i64(ctx, self.velocity.z as i64),
        }
    }
}

struct Z3Hailstone<'a> {
    x: Int<'a>,
    y: Int<'a>,
    z: Int<'a>,
    vx: Int<'a>,
    vy: Int<'a>,
    vz: Int<'a>,
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

impl Display for Hailstone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} @ {}", self.position, self.velocity)
    }
}

fn process(input: &str) -> String {
    let hailstones: Vec<Hailstone> = input.lines().map(Hailstone::from).collect();

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let (rx, ry, rz, vrx, vry, vrz) = (
        Int::new_const(&ctx, "rx"),
        Int::new_const(&ctx, "ry"),
        Int::new_const(&ctx, "rz"),
        Int::new_const(&ctx, "vrx"),
        Int::new_const(&ctx, "vry"),
        Int::new_const(&ctx, "vrz"),
    );
    let zero = Int::from_i64(&ctx, 0);

    for (i, h) in (0..).zip(hailstones.iter().take(5).map(|h| h.z3_hailstone(&ctx))) {
        let t = Int::new_const(&ctx, format!("t{i}"));

        solver.assert(&t.gt(&zero));
        solver.assert(&(&rx + &vrx * &t)._eq(&(h.x + h.vx * &t)));
        solver.assert(&(&ry + &vry * &t)._eq(&(h.y + h.vy * &t)));
        solver.assert(&(&rz + &vrz * &t)._eq(&(h.z + h.vz * &t)));
    }

    solver.check();
    let model = solver.get_model().unwrap();
    let answer = model.eval(&(rx + ry + rz), true).unwrap();
    answer.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve2() {
        let input = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

        let result = process(input);
        assert_eq!(result, "47".to_string());
    }
}
