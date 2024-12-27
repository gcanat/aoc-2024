use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;
use std::collections::HashMap;

/// Seems like a good opportunity to fool around with cmp/ordering traits.
/// If our `Rule` struct implements the Ord trait, we can just call `sort()`
/// on a `Vec<Rule>` to get it sorted.
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
        Some(self.cmp(other))
    }
}

impl Ord for Rule {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.after.iter().any(|c| c == &other.before) {
            Ordering::Less
        } else {
            Ordering::Greater
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
            let k: usize = entries.next().unwrap().parse().unwrap();
            let v: usize = entries.next().unwrap().parse().unwrap();
            rules
                .entry(k)
                .and_modify(|c| c.after.push(v))
                .or_insert(Rule {
                    before: k,
                    after: vec![v],
                });
            rules.entry(v).or_insert_with(|| Rule {
                before: v,
                after: vec![],
            });
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

fn sort_update(update: &[usize], rules: &Rules) -> Vec<usize> {
    let mut res = update
        .iter()
        .map(|x| rules.get(x).unwrap())
        .collect::<Vec<_>>();
    res.sort();
    res.iter().map(|r| r.before).collect::<Vec<_>>()
}

fn get_middle(update: &[usize]) -> usize {
    let mid = update.len() / 2;
    update[mid]
}

#[aoc(day5, part1)]
fn part1((rules, updates): &(Rules, Updates)) -> usize {
    updates
        .iter()
        .filter(|u| {
            let sorted = sort_update(u, rules);
            *u == &sorted
        })
        .map(|u| get_middle(u))
        .sum()
}

#[aoc(day5, part2)]
fn part2((rules, updates): &(Rules, Updates)) -> usize {
    updates.iter().fold(0, |acc, u| {
        let sorted = sort_update(u, rules);
        if u != &sorted {
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
