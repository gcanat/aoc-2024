use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

fn parse_mult(input: &str) -> Vec<(usize, usize)> {
    let mut ops: Vec<(usize, usize)> = Vec::new();
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    for (_, [lhs, rhs]) in re.captures_iter(input).map(|c| c.extract()) {
        ops.push((lhs.parse::<usize>().unwrap(), rhs.parse::<usize>().unwrap()));
    }
    ops
}

#[aoc_generator(day3, part1)]
pub fn gen_d3_p1(input: &str) -> Vec<(usize, usize)> {
    parse_mult(input)
}

#[aoc(day3, part1)]
pub fn solve_d3_p1(mult: &Vec<(usize, usize)>) -> usize {
    mult.iter().fold(0, |acc, (a, b)| acc + (a * b))
}

#[aoc_generator(day3, part2)]
pub fn gen_d3_p2(input: &str) -> Vec<(usize, usize)> {
    let re = Regex::new(r"(?s)don't\(\).*?(do\(\)|$)").unwrap();
    let filtered_input = re.replace_all(input, "");
    parse_mult(&filtered_input)
}

#[aoc(day3, part2)]
pub fn solve_d3_p2(mult: &Vec<(usize, usize)>) -> usize {
    mult.iter().fold(0, |acc, (a, b)| acc + (a * b))
}
