use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day11)]
fn parse(input: &str) -> Vec<usize> {
    let mut res: Vec<usize> = Vec::new();
    for x in input.split_whitespace() {
        res.push(x.parse::<usize>().unwrap());
    }
    res
}

fn update_number(n: usize) -> (usize, Option<usize>) {
    if n == 0 {
        return (1, None);
    }
    let n_digits = n.ilog10() as i32 + 1;
    if n_digits % 2 == 0 {
        let p = n_digits / 2;
        let x = n as f64 / (10_f64.powi(p));
        let left = x.floor();
        let right = (x - left) * 10_f64.powi(p);
        return (left as usize, Some(right.round() as usize));
    } else {
        return (n * 2024, None);
    }
}

fn solve(input: &Vec<usize>, n: usize) -> usize {
    let mut val_map: HashMap<usize, usize> = HashMap::new();
    for k in input.iter() {
        val_map.entry(*k).and_modify(|v| *v += 1).or_insert(1);
    }
    for _i in 0..n {
        let mut new_map: HashMap<usize, usize> = HashMap::new();
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
fn part1(input: &Vec<usize>) -> usize {
    solve(input, 25)
}

#[aoc(day11, part2)]
fn part2(input: &Vec<usize>) -> usize {
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
