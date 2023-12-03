use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");
    let answer = process(input);
    dbg!(answer);
}

#[derive(Debug)]
struct CubeSet {
    red: usize,
    green: usize,
    blue: usize,
}

impl CubeSet {
    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

impl FromStr for CubeSet {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blue = 0;
        let mut red = 0;
        let mut green = 0;

        for cubes in s.split(',') {
            let (n, cube) = cubes.trim().split_once(' ').unwrap();
            match cube {
                "red" => red = n.parse().unwrap(),
                "green" => green = n.parse().unwrap(),
                "blue" => blue = n.parse().unwrap(),
                _ => panic!("invalid color"),
            }
        }

        Ok(CubeSet { red, green, blue })
    }
}

#[derive(Debug)]
struct Game {
    cube_sets: Vec<CubeSet>,
}
#[derive(Debug)]
struct ParseGameError;

impl Game {
    fn min_cube_set(&self) -> CubeSet {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for cube_set in &self.cube_sets {
            if cube_set.red > red {
                red = cube_set.red;
            }
            if cube_set.green > green {
                green = cube_set.green;
            }
            if cube_set.blue > blue {
                blue = cube_set.blue;
            }
        }

        CubeSet { red, green, blue }
    }
}

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, body) = s.split_once(':').expect("invalid game round");

        let cube_sets = body
            .split(';')
            .map(|set| set.parse().unwrap())
            .collect::<Vec<CubeSet>>();

        Ok(Game { cube_sets })
    }
}

fn process(input: &str) -> String {
    let answer: usize = input
        .lines()
        .map(|line| line.parse::<Game>().unwrap())
        .map(|game| game.min_cube_set())
        .fold(0, |acc, cube_set| acc + cube_set.power());

    answer.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let result = process(input);
        assert_eq!(result, "2286".to_string());
    }
}
