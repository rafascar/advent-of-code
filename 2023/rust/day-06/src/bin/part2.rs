use nom::{
    bytes::complete::take_till,
    character::complete::{self, digit1, multispace1, newline},
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
    time: u64,
    distance: u64,
}

impl Round {
    fn ways_to_win(&self) -> u64 {
        let t = self.time as f64;
        let r = self.distance as f64;

        let radical = (t.powi(2) - 4_f64 * r).sqrt();
        let min = 0.5 * (t - radical);
        let max = 0.5 * (radical + t);

        (dbg!(max).ceil() as u64) - (dbg!(min).floor() as u64) - 1
    }
}

fn parse_line(i: &str) -> IResult<&str, u64> {
    let (i, numbers) = preceded(
        take_till(|c: char| c.is_ascii_digit()),
        separated_list1(multispace1, digit1),
    )(i)?;

    let number = numbers.join("").parse().expect("should be a number");
    Ok((i, number))
}

fn parse(i: &str) -> IResult<&str, Round> {
    let (i, (time, distance)) = separated_pair(parse_line, newline, parse_line)(i)?;

    Ok((i, Round { time, distance }))
}

fn process(input: &str) -> String {
    let (_, round) = parse(input).expect("should be able to parse input");
    dbg!(round).ways_to_win().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        let result = process(input);
        assert_eq!(result, "71503".to_string());
    }

    #[test]
    fn test_parse_line() {
        let input = "Time:      7  15   30";
        assert_eq!(parse_line(input), Ok(("", 71530)));
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
