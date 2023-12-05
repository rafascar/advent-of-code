use std::{collections::HashSet, str::FromStr};

fn main() {
    let input = include_str!("input.txt");
    let answer = process(input);
    dbg!(answer);
}

#[derive(Debug)]
struct Card {
    winning: HashSet<usize>,
    ours: HashSet<usize>,
}

#[derive(Debug)]
struct CardParseError;

impl FromStr for Card {
    type Err = CardParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, body) = s.split_once(':').unwrap();
        let (winning, ours) = body.split_once('|').unwrap();

        Ok(Card {
            winning: winning
                .split(' ')
                .filter_map(|n| n.trim().parse::<usize>().ok())
                .collect(),
            ours: ours
                .split(' ')
                .filter_map(|n| n.trim().parse::<usize>().ok())
                .collect(),
        })
    }
}

fn process(input: &str) -> String {
    let mut cards: Vec<(Card, usize)> = input
        .lines()
        .map(|line| (line.parse::<Card>().unwrap(), 1))
        .collect();

    for i in 0..cards.len() {
        let card = &cards[i];
        let matching = card.0.winning.intersection(&card.0.ours).count();
        for n in i + 1..=i + matching {
            cards[n].1 += cards[i].1;
        }
    }

    cards.iter().fold(0, |acc, card| acc + card.1).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let result = process(input);
        assert_eq!(result, "30".to_string());
    }
}
