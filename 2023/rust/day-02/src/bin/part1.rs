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
    id: usize,
    cube_sets: Vec<CubeSet>,
}
#[derive(Debug)]
struct ParseGameError;

impl Game {
    fn is_allowed(&self) -> bool {
        for cube_set in &self.cube_sets {
            if cube_set.red > 12 || cube_set.green > 13 || cube_set.blue > 14 {
                return false;
            }
        }
        true
    }
}

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (header, body) = s.split_once(':').expect("invalid game round");

        let id = header.split(' ').last().expect("should be 'Game ID'");
        let cube_sets = body
            .split(';')
            .map(|set| set.parse().unwrap())
            .collect::<Vec<CubeSet>>();

        Ok(Game {
            id: id.parse().expect("should be a number"),
            cube_sets,
        })
    }
}

fn process(input: &str) -> String {
    let answer: usize = input
        .lines()
        .map(|line| line.parse::<Game>().unwrap())
        .filter(Game::is_allowed)
        .fold(0, |acc, game| acc + game.id);

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
        assert_eq!(result, "8".to_string());
    }
}
