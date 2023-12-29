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

    for x in 0..1000000000 {
        if x % 1000000 == 0 {
            println!("{}", x);
        }
        // Roll north.
        for i in 1..dish.len() {
            for j in 0..dish[0].len() {
                if dish[i][j] != 'O' {
                    continue;
                }
                dish[i][j] = '.';

                let row = dish
                    .iter()
                    .take(i)
                    .rev()
                    .map(|line| line.get(j).unwrap())
                    .position(|&c| c == 'O' || c == '#')
                    .unwrap_or(i);

                dish[i - row][j] = 'O';
            }
        }

        // Roll west.
        for i in 0..dish.len() {
            for j in 1..dish[0].len() {
                if dish[i][j] != 'O' {
                    continue;
                }
                dish[i][j] = '.';

                let col = dish[i]
                    .iter()
                    .take(j)
                    .rev()
                    .position(|&c| c == 'O' || c == '#')
                    .unwrap_or(j);

                dish[i][j - col] = 'O';
            }
        }

        // Roll south.
        for i in (0..dish.len() - 1).rev() {
            for j in 0..dish[0].len() {
                if dish[i][j] != 'O' {
                    continue;
                }
                dish[i][j] = '.';

                let row = dish
                    .iter()
                    .skip(i)
                    .map(|line| line.get(j).unwrap())
                    .position(|&c| c == 'O' || c == '#')
                    .unwrap_or(dish.len() - i);

                dish[i + row - 1][j] = 'O';
            }
        }

        // Roll east.
        for i in 0..dish.len() {
            for j in (0..dish[0].len() - 1).rev() {
                if dish[i][j] != 'O' {
                    continue;
                }
                dish[i][j] = '.';

                let col = dish[i]
                    .iter()
                    .skip(j)
                    .position(|&c| c == 'O' || c == '#')
                    .unwrap_or(dish[0].len() - j);

                dish[i][j + col - 1] = 'O';
            }
        }
    }

    for line in dish.iter() {
        println!("{}", line.iter().collect::<String>());
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
        assert_eq!(result, "135".to_string());
    }
}
