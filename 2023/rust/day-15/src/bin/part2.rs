fn main() {
    let input = include_str!("input.txt");
    let answer = process(input);
    dbg!(answer);
}

#[derive(Debug)]
struct Lens {
    label: String,
    focal_length: usize,
}

fn hash(label: &str) -> usize {
    label
        .chars()
        .fold(0, |acc, c| ((acc + (c as u64)) * 17) % 256) as usize
}

#[derive(Debug)]
struct Box(Vec<Lens>);

impl Box {
    fn remove(&mut self, label: &str) {
        if let Some(i) = self.0.iter().position(|lens| lens.label == label) {
            self.0.remove(i);
        }
    }

    fn insert(&mut self, lens: Lens) {
        if let Some(i) = self.0.iter().position(|l| l.label == lens.label) {
            self.0[i] = lens;
        } else {
            self.0.push(lens);
        }
    }
}

fn process(input: &str) -> String {
    const EMPTY_BOX: Box = Box(Vec::new());
    let mut boxes: [Box; 256] = [EMPTY_BOX; 256];

    for s in input.trim_end().split(',') {
        if let Some((label, focal_length)) = s.split_once('=') {
            let lens = Lens {
                label: label.into(),
                focal_length: focal_length.parse().expect("should be a number"),
            };
            boxes[hash(&lens.label)].insert(lens);
        } else if let Some((label, _)) = s.split_once('-') {
            boxes[hash(label)].remove(label);
        } else {
            panic!("failed parsing input");
        }
    }

    boxes
        .iter()
        .enumerate()
        .flat_map(|(i, b)| {
            b.0.iter()
                .enumerate()
                .map(move |(j, lens)| (i + 1) * (j + 1) * lens.focal_length)
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        let result = process(input);
        assert_eq!(result, "145".to_string());
    }
}
