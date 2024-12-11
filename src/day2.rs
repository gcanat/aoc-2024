use aoc_runner_derive::{aoc, aoc_generator};

// - The levels are either all increasing or all decreasing.
// - Any two adjacent levels differ by at least one and at most three.

#[aoc_generator(day2)]
pub fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn validate_level(range: &[usize], is_decrease: bool) -> bool {
    let diff = range[1] as i32 - range[0] as i32;
    if diff == 0 {
        return false;
    } else if diff.abs() > 3 {
        return false;
    } else if (diff > 0) && is_decrease {
        return false;
    } else if (diff < 0) && !is_decrease {
        return false;
    } else {
        return true;
    }
}

fn validate_report(report: &Vec<usize>) -> (usize, usize) {
    let mut is_valid = 1;
    let line_len = report.len();
    let is_decrease = (report[1] as i32 - report[0] as i32) < 0;
    let mut bad_lvl = 0;
    for i in 1..line_len {
        let res = validate_level(&report[i - 1..i + 1], is_decrease);
        if !res {
            is_valid = 0;
            bad_lvl = i;
            break;
        }
    }
    (is_valid, bad_lvl)
}

#[aoc(day2, part1)]
pub fn part1(table: &[Vec<usize>]) -> usize {
    table
        .iter()
        .map(|x| {
            let (is_valid, _) = validate_report(&x);
            is_valid
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(table: &[Vec<usize>]) -> usize {
    table
        .into_iter()
        .map(|x| {
            let (mut is_valid, bad_lvl) = validate_report(x);
            if is_valid == 0 {
                // remove one item from the report and see if it's valid
                // no need to go further than bad_lvl
                for i in 0..bad_lvl + 1 {
                    let mut y = x.clone();
                    y.remove(i);
                    let res = validate_report(&y);
                    is_valid = res.0;
                    if is_valid == 1 {
                        break;
                    }
                }
            }
            is_valid
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "7 6 4 2 1\n\
                                 1 2 7 8 9\n\
                                 9 7 6 2 1\n\
                                 1 3 2 4 5\n\
                                 8 6 4 4 1\n\
                                 1 3 6 7 9";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 4);
    }
}
