use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;
use std::collections::HashMap;

/// Seems like a good opportunity to fool around with cmp/ordering traits

struct Rule {
    before: usize,
    after: Vec<usize>,
}

type Rules = HashMap<usize, Rule>;
type Updates = Vec<Vec<usize>>;

impl PartialEq for Rule {
    fn eq(&self, other: &Self) -> bool {
        !self.after.iter().any(|c| c == &other.before)
            && !other.after.iter().any(|c| c == &self.before)
    }
}

impl PartialOrd for Rule {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.after.iter().any(|c| c == &other.before) {
            Some(Ordering::Less)
        } else {
            None
        }
    }
}

impl Ord for Rule {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(other) {
            Some(ord) => ord,
            None => Ordering::Greater,
        }
    }
}

impl Eq for Rule {}

#[aoc_generator(day5)]
fn parse(input: &str) -> (Rules, Updates) {
    let mut rules: Rules = HashMap::new();
    let mut updates: Updates = Vec::new();
    for l in input.lines() {
        if l.contains("|") {
            let mut entries = l.split('|');
            let k = entries.next().unwrap();
            let v = entries.next().unwrap();
            let k_int = k.parse::<usize>().unwrap();
            let v_int = v.parse::<usize>().unwrap();
            rules
                .entry(k_int)
                .and_modify(|c| c.after.push(v_int))
                .or_insert(Rule {
                    before: k_int,
                    after: vec![v_int],
                });
            if rules.get(&v_int).is_none() {
                rules.insert(
                    v_int,
                    Rule {
                        before: v_int,
                        after: vec![],
                    },
                );
            }
        } else if l.contains(",") {
            let entries = l
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            updates.push(entries);
        }
    }
    (rules, updates)
}

fn sort_update(update: &Vec<usize>, rules: &Rules) -> Vec<usize> {
    let mut res = update
        .iter()
        .map(|x| rules.get(x).unwrap())
        .collect::<Vec<_>>();
    res.sort();
    res.iter().map(|r| r.before).collect::<Vec<_>>()
}

fn get_middle(update: &Vec<usize>) -> usize {
    let mid = update.len() / 2;
    update[mid]
}

#[aoc(day5, part1)]
fn part1(input: &(Rules, Updates)) -> usize {
    input
        .1
        .iter()
        .filter(|u| {
            let sorted = sort_update(u, &input.0);
            *u == &sorted
        })
        .map(get_middle)
        .sum()
}

#[aoc(day5, part2)]
fn part2(input: &(Rules, Updates)) -> usize {
    input.1.iter().fold(0, |acc, u| {
        let sorted = sort_update(u, &input.0);
        if !(u == &sorted) {
            acc + get_middle(&sorted)
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "47|53\n\
        97|13\n\
        97|61\n\
        97|47\n\
        75|29\n\
        61|13\n\
        75|53\n\
        29|13\n\
        97|29\n\
        53|29\n\
        61|53\n\
        97|53\n\
        61|29\n\
        47|13\n\
        75|47\n\
        97|75\n\
        47|61\n\
        75|61\n\
        47|29\n\
        75|13\n\
        53|13\n\
        \n\
        75,47,61,53,29\n\
        97,61,53,29,13\n\
        75,29,13\n\
        75,97,47,61,53\n\
        61,13,29\n\
        97,13,75,29,47";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 143);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 123);
    }
}
