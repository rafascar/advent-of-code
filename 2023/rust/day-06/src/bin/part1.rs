use nom::{
    bytes::complete::take_till,
    character::complete::{self, multispace1, newline},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

fn main() {
    let input = include_str!("input.txt");
    let answer = process(input);
    dbg!(answer);
}

#[derive(Debug)]
struct Round {
    time: u32,
    distance: u32,
}

impl Round {
    fn ways_to_win(&self) -> u32 {
        let t = self.time as f32;
        let r = self.distance as f32;

        let radical = (t.powi(2) - 4_f32 * r).sqrt();
        let min = 0.5 * (t - radical);
        let max = 0.5 * (radical + t);

        (max.ceil() as u32) - (min.floor() as u32) - 1
    }
}

fn parse_line(i: &str) -> IResult<&str, Vec<u32>> {
    preceded(
        take_till(|c: char| c.is_ascii_digit()),
        separated_list1(multispace1, complete::u32),
    )(i)
}

fn parse(i: &str) -> IResult<&str, Vec<Round>> {
    let (i, (times, distances)) = separated_pair(parse_line, newline, parse_line)(i)?;

    let rounds = times
        .iter()
        .zip(distances.iter())
        .map(|(&time, &distance)| Round { time, distance })
        .collect();

    Ok((i, rounds))
}

fn process(input: &str) -> String {
    let (_, rounds) = parse(input).expect("should be able to parse input");
    rounds
        .iter()
        .fold(1, |acc, round| acc * round.ways_to_win())
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        let result = process(input);
        assert_eq!(result, "288".to_string());
    }

    #[test]
    fn test_parse_line() {
        let input = "Time:      7  15   30";
        assert_eq!(parse_line(input), Ok(("", vec![7, 15, 30])));
    }

    #[test]
    fn test_ways_to_win() {
        let round = Round {
            time: 30,
            distance: 200,
        };
        assert_eq!(round.ways_to_win(), 9);
    }
}
