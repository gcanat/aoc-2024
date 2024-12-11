use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::iter::zip;

#[aoc_generator(day1)]
pub fn parse(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut left_list: Vec<usize> = Vec::new();
    let mut right_list: Vec<usize> = Vec::new();
    let _ = input
        .lines()
        .map(|l| {
            let mut iter = l.split_whitespace();
            left_list.push(iter.next().unwrap().parse::<usize>().unwrap());
            right_list.push(iter.next().unwrap().parse::<usize>().unwrap());
        })
        .collect::<()>();
    (left_list, right_list)
}

#[aoc(day1, part1)]
pub fn part1(input: &(Vec<usize>, Vec<usize>)) -> usize {
    let mut left_list = input.0.to_owned();
    left_list.sort_unstable();
    let mut right_list = input.1.to_owned();
    right_list.sort_unstable();
    let iter = zip(left_list, right_list);
    let result = iter.fold(0, |acc, (x, y)| acc + (x as i32 - y as i32).abs());
    result as usize
}

#[aoc(day1, part2)]
pub fn part2(input: &(Vec<usize>, Vec<usize>)) -> usize {
    let lhs = input.0.clone();
    let rhs = input.1.clone();

    // count all entries in the right hand side list
    let mut rhs_count: HashMap<usize, usize> = HashMap::new();
    for n in rhs.iter() {
        rhs_count.entry(*n).and_modify(|v| *v += 1).or_insert(1);
    }

    // iterate on the left hand side list and compute the weighted sum
    lhs.iter().fold(0, |acc, n| {
        if let Some(val) = rhs_count.get(n) {
            acc + (n * val)
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "3   4\n\
                                 4   3\n\
                                 2   5\n\
                                 1   3\n\
                                 3   9\n\
                                 3   3";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 11);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 31);
    }
}
