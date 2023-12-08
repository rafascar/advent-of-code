use std::{ops::Range, str::FromStr, u64};

use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, newline},
    combinator::opt,
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

fn main() {
    let input = include_str!("input.txt");
    let answer = process(input);
    dbg!(answer);
}

#[derive(Debug, PartialEq)]
enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl FromStr for Category {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "seed" => Ok(Category::Seed),
            "soil" => Ok(Category::Soil),
            "fertilizer" => Ok(Category::Fertilizer),
            "water" => Ok(Category::Water),
            "light" => Ok(Category::Light),
            "temperature" => Ok(Category::Temperature),
            "humidity" => Ok(Category::Humidity),
            "location" => Ok(Category::Location),
            _ => Err("invalid category"),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Map {
    src: Category,
    dst: Category,
    ranges: Vec<(u64, u64, u64)>,
}

impl Map {
    /// Convert 'value' from 'src' category to 'dst' category.
    fn convert(&self, value: u64) -> u64 {
        for range in &self.ranges {
            if (range.1..range.1 + range.2).contains(&value) {
                return (value - range.1) + range.0;
            }
        }

        value
    }

    /// Restores 'value' from 'dst' category to 'src' category.
    fn restore(&self, value: u64) -> u64 {
        for range in &self.ranges {
            if (range.0..range.0 + range.2).contains(&value) {
                return (value - range.0) + range.1;
            }
        }

        value
    }
}

#[derive(Debug)]
struct Number {
    value: u64,
    category: Category,
}

impl Number {
    fn to(&self, dst: Category, maps: &[Map]) -> Number {
        let map = maps
            .iter()
            .find(|map| map.src == self.category && map.dst == dst)
            .expect("should find a map from src->dst");

        Number {
            value: map.convert(self.value),
            category: dst,
        }
    }

    fn from(&self, src: Category, maps: &[Map]) -> Number {
        let map = maps
            .iter()
            .find(|map| map.dst == self.category && map.src == src)
            .expect("should find a map from src->dst");

        Number {
            value: map.restore(self.value),
            category: src,
        }
    }
}

fn seeds(input: &str) -> IResult<&str, SeedBag> {
    let (input, seeds) = separated_list1(
        tag(" "),
        separated_pair(complete::u64, tag(" "), complete::u64),
    )(input)?;

    let seeds = seeds
        .iter()
        .map(|(start, length)| *start..(*start + *length))
        .collect();

    Ok((input, SeedBag(seeds)))
}

fn range(input: &str) -> IResult<&str, (u64, u64, u64)> {
    let (input, ranges) = separated_list1(tag(" "), complete::u64)(input)?;

    let mut ranges = ranges.iter();
    if let (Some(dst), Some(src), Some(n)) = (ranges.next(), ranges.next(), ranges.next()) {
        Ok((input, (*dst, *src, *n)))
    } else {
        panic!("invalid range");
    }
}

fn map(input: &str) -> IResult<&str, Map> {
    let (input, (src, dst)) =
        terminated(separated_pair(alpha1, tag("-to-"), alpha1), tag(" map:\n"))(input)?;
    let (input, ranges) = terminated(separated_list1(newline, range), opt(newline))(input)?;

    Ok((
        input,
        Map {
            src: src.parse().unwrap(),
            dst: dst.parse().unwrap(),
            ranges,
        },
    ))
}

struct SeedBag(Vec<Range<u64>>);

impl SeedBag {
    fn contains(&self, value: u64) -> bool {
        for s in &self.0 {
            if s.contains(&value) {
                return true;
            }
        }
        false
    }
}

fn process(input: &str) -> IResult<&str, String> {
    let (input, bag) = terminated(preceded(tag("seeds: "), seeds), newline)(input)?;
    let (input, _) = newline(input)?;
    let (_, maps) = separated_list1(newline, map)(input)?;

    let mut i = 0;
    let answer = loop {
        let seed = Number {
            value: i,
            category: Category::Location,
        }
        .from(Category::Humidity, &maps)
        .from(Category::Temperature, &maps)
        .from(Category::Light, &maps)
        .from(Category::Water, &maps)
        .from(Category::Fertilizer, &maps)
        .from(Category::Soil, &maps)
        .from(Category::Seed, &maps);

        if bag.contains(seed.value) {
            break i;
        }
        i += 1;
    };

    Ok(("", answer.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        let result = process(input);
        match result {
            Ok((_, answer)) => assert_eq!(answer, "46".to_string()),
            err => println!("{err:?}"),
        }
    }

    #[test]
    fn test_convert() {
        let map = Map {
            src: Category::Seed,
            dst: Category::Soil,
            ranges: vec![(50, 98, 2), (52, 50, 48)],
        };

        assert_eq!(map.convert(0), 0);
        assert_eq!(map.convert(99), 51);
    }

    #[test]
    fn test_restore() {
        let map = Map {
            src: Category::Seed,
            dst: Category::Soil,
            ranges: vec![(50, 98, 2), (52, 50, 48)],
        };

        assert_eq!(map.restore(0), 0);
        assert_eq!(map.restore(51), 99);
    }

    #[test]
    fn test_range() {
        let (_, range) = range("1 2 3").expect("should be able to parse");
        assert_eq!(range, (1, 2, 3));
    }

    #[test]
    fn test_map() {
        let (_, map) = map("seed-to-soil map:
50 98 2
52 50 48")
        .expect("should be able to parse");

        assert_eq!(
            map,
            Map {
                src: Category::Seed,
                dst: Category::Soil,
                ranges: vec![(50, 98, 2), (52, 50, 48)]
            }
        )
    }
}
