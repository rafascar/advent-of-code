fn main() {
    let input = include_str!("input.txt");
    let answer = process(input);
    dbg!(answer);
}

fn process(input: &str) -> String {
    input
        .trim_end()
        .split(',')
        .map(|s| s.chars().fold(0, |acc, c| ((acc + (c as u64)) * 17) % 256))
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        let result = process(input);
        assert_eq!(result, "1320".to_string());
    }
}
