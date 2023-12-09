use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let answer = process(input);
    dbg!(answer);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Card(char);

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let labels = "23456789TJQKA";
        let my_label = labels.find(self.0).expect("should be a valid label");
        let other_label = labels.find(other.0).expect("should be a valid label");
        my_label.cmp(&other_label)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[derive(Debug, Eq)]
struct Hand {
    cards: [Card; 5],
    bet: usize,
}

impl Hand {
    fn r#type(&self) -> Type {
        let counts = self.cards.iter().fold(HashMap::new(), |mut count, card| {
            count.entry(card.0).and_modify(|c| *c += 1).or_insert(1);
            count
        });
        let mut counts = counts.values().cloned().collect::<Vec<i32>>();
        counts.sort();
        counts.reverse();

        match counts[..] {
            [5] => Type::FiveKind,
            [4, 1] => Type::FourKind,
            [3, 2] => Type::FullHouse,
            [3, 1, 1] => Type::ThreeKind,
            [2, 2, 1] => Type::TwoPair,
            [2, 1, 1, 1] => Type::OnePair,
            [1, 1, 1, 1, 1] => Type::HighCard,
            _ => panic!("invalid hand type"),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.r#type() == other.r#type() {
            return self.cards.cmp(&other.cards);
        }

        self.r#type().cmp(&other.r#type())
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

fn parse(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| {
            let (cards, bet) = line.split_once(' ').expect("should be able to parse line");
            let cards = cards.chars().map(Card).collect::<Vec<_>>();

            Hand {
                cards: cards.try_into().expect("should have only 5 cards"),
                bet: bet.parse().expect("should be a number"),
            }
        })
        .collect()
}

fn process(input: &str) -> String {
    let mut hands = parse(input);
    hands.sort();

    hands
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (i + 1) * hand.bet)
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let result = process(input);
        assert_eq!(result, "6440".to_string());
    }

    #[test]
    fn test_hand_type() {
        let hand = Hand {
            cards: [Card('A'), Card('A'), Card('A'), Card('B'), Card('C')],
            bet: 0,
        };
        assert_eq!(hand.r#type(), Type::ThreeKind);
    }

    #[test]
    fn test_hand_cmp() {
        let my_hand = Hand {
            cards: [Card('A'), Card('A'), Card('A'), Card('3'), Card('3')],
            bet: 0,
        };
        let other_hand = Hand {
            cards: [Card('A'), Card('A'), Card('A'), Card('2'), Card('2')],
            bet: 0,
        };
        assert!(my_hand > other_hand);
    }
}
