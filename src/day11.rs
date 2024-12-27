use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day11)]
fn parse(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
}

fn update_number(n: u64) -> (u64, Option<u64>) {
    if n == 0 {
        return (1, None);
    }
    let n_digits = n.ilog10() + 1;
    if n_digits % 2 == 0 {
        let base = 10_u64.pow(n_digits / 2);
        let left = n / base;
        let right = n % base;
        (left, Some(right))
    } else {
        (n * 2024, None)
    }
}

fn solve(input: &[u64], n: u64) -> u64 {
    let mut val_map: HashMap<u64, u64> = HashMap::new();
    for k in input.iter() {
        val_map.entry(*k).and_modify(|v| *v += 1).or_insert(1);
    }
    for _i in 0..n {
        let mut new_map: HashMap<u64, u64> = HashMap::new();
        for (k, v) in val_map.iter() {
            let res = update_number(*k);
            new_map.entry(res.0).and_modify(|e| *e += v).or_insert(*v);
            if let Some(new_val) = res.1 {
                new_map.entry(new_val).and_modify(|e| *e += v).or_insert(*v);
            }
        }
        val_map = new_map;
    }
    val_map.iter().fold(0, |acc, (_k, v)| acc + v)
}

#[aoc(day11, part1)]
fn part1(input: &[u64]) -> u64 {
    solve(input, 25)
}

#[aoc(day11, part2)]
fn part2(input: &[u64]) -> u64 {
    solve(input, 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "125 17";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 55312);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 65601038650482);
    }
}
