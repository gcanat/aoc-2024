use aoc_runner_derive::{aoc, aoc_generator};
use rayon::prelude::*;

#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<(usize, Vec<usize>)> {
    input
        .lines()
        .map(|l| {
            let Some((result, numbers)) = l.split_once(':') else {
                return (0, vec![0]);
            };
            let target_num = result.parse::<usize>().unwrap();
            let num_list: Vec<usize> = numbers
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            return (target_num, num_list);
        })
        .collect::<Vec<_>>()
}

enum Operation {
    Add,
    Mult,
    Concat,
}

impl Operation {
    fn apply(self, x: usize, y: usize) -> usize {
        match self {
            Self::Add => x + y,
            Self::Mult => x * y,
            Self::Concat => x * (10_usize).pow(y.ilog10() + 1) + y,
        }
    }
}

fn apply_op(current: usize, next: &[usize], target: &usize, op: Operation, day2: bool) -> bool {
    if next.len() == 0 {
        return &current == target;
    }
    let new = op.apply(current, next[0]);
    if &new > target {
        return false;
    } else {
        let mult_res = apply_op(new, &next[1..], target, Operation::Mult, day2);
        let add_res = apply_op(new, &next[1..], target, Operation::Add, day2);
        if !day2 {
            return mult_res || add_res;
        } else {
            let concat_res = apply_op(new, &next[1..], target, Operation::Concat, day2);
            return mult_res || add_res || concat_res;
        }
    }
}

fn solve(input: &Vec<(usize, Vec<usize>)>, day2: bool) -> usize {
    input
        .par_iter()
        .map(|(x, y)| {
            // initial operation is multiplying 1 with first value.
            let res = apply_op(1, &y[..], x, Operation::Mult, day2);
            if res {
                return *x;
            } else {
                return 0;
            }
        })
        .sum()
}

#[aoc(day7, part1)]
fn part1(input: &Vec<(usize, Vec<usize>)>) -> usize {
    solve(input, false)
}

#[aoc(day7, part2)]
fn part2(input: &Vec<(usize, Vec<usize>)>) -> usize {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "190: 10 19\n\
                                 3267: 81 40 27\n\
                                 83: 17 5\n\
                                 156: 15 6\n\
                                 7290: 6 8 6 15\n\
                                 161011: 16 10 13\n\
                                 192: 17 8 14\n\
                                 21037: 9 7 18 13\n\
                                 292: 11 6 16 20";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 3749);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 11387);
    }
}
