use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let answer = process(input);
    dbg!(answer);
}

fn count(cfg: &[char], nums: &[usize], cache: &mut HashMap<(String, Vec<usize>), usize>) -> usize {
    if nums.is_empty() {
        return if cfg.contains(&'#') { 0 } else { 1 };
    }

    if cfg.is_empty() {
        return if nums.is_empty() { 1 } else { 0 };
    }

    let key = (cfg.iter().collect::<String>(), nums.to_vec());
    if let Some(r) = cache.get(&key) {
        return *r;
    }

    let mut result = 0;

    if ".?".contains(cfg[0]) {
        result += count(&cfg[1..], nums, cache);
    }

    if "#?".contains(cfg[0])
        && nums[0] <= cfg.len()
        && !cfg[..nums[0]].contains(&'.')
        && (nums[0] == cfg.len() || cfg[nums[0]] != '#')
    {
        result += count(cfg.get(nums[0] + 1..).unwrap_or(&[]), &nums[1..], cache);
    }

    cache.insert(key, result);
    result
}

fn process(input: &str) -> String {
    let mut answer = 0;
    let mut cache = HashMap::new();
    for line in input.lines().collect_vec().iter() {
        let (cfg, nums) = line.split_once(' ').expect("should be 'cfg nums'");
        let cfg = std::iter::repeat(cfg)
            .take(5)
            .join("?")
            .chars()
            .collect::<Vec<char>>();
        let nums = std::iter::repeat(nums)
            .take(5)
            .join(",")
            .split(',')
            .map(|n| n.parse::<usize>().expect("should be a number"))
            .collect::<Vec<usize>>();

        answer += count(&cfg, &nums, &mut cache);
    }
    answer.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        let result = process(input);
        assert_eq!(result, "525152".to_string());
    }
}
