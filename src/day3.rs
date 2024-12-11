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
pub fn parse1(input: &str) -> Vec<(usize, usize)> {
    parse_mult(input)
}

#[aoc(day3, part1)]
pub fn part1(mult: &Vec<(usize, usize)>) -> usize {
    mult.iter().fold(0, |acc, (a, b)| acc + (a * b))
}

#[aoc_generator(day3, part2)]
pub fn parse2(input: &str) -> Vec<(usize, usize)> {
    let re = Regex::new(r"(?s)don't\(\).*?(do\(\)|$)").unwrap();
    let filtered_input = re.replace_all(input, "");
    parse_mult(&filtered_input)
}

#[aoc(day3, part2)]
pub fn part2(mult: &Vec<(usize, usize)>) -> usize {
    mult.iter().fold(0, |acc, (a, b)| acc + (a * b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part1(&parse1(input)), 161);
    }

    #[test]
    fn part2_example() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2(&parse2(input)), 48);
    }
}
