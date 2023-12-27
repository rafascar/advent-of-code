fn main() {
    let input = include_str!("input.txt");
    let answer = process(input);
    dbg!(answer);
}

fn process(input: &str) -> String {
    let mut dish: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    for i in 1..dish.len() {
        for j in 0..dish[0].len() {
            if dish[i][j] != 'O' {
                continue;
            }
            dish[i][j] = '.';

            let mut y = Some(i - 1);
            while y.is_some() && dish[y.unwrap()][j] == '.' {
                y = y.unwrap().checked_sub(1);
            }

            if let Some(y) = y {
                dish[y + 1][j] = 'O';
            } else {
                dish[0][j] = 'O';
            }
        }
    }

    dish.iter()
        .enumerate()
        .fold(0, |acc, (i, line)| {
            acc + line.iter().filter(|&&c| c == 'O').count() * (dish.len() - i)
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let result = process(input);
        assert_eq!(result, "136".to_string());
    }
}
