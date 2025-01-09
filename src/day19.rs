use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

fn char_to_digit(c: char) -> usize {
    match c {
        'w' => 0,
        'u' => 1,
        'b' => 2,
        'r' => 3,
        'g' => 4,
        _ => 5,
    }
}

#[aoc_generator(day19)]
fn parse(input: &str) -> (HashSet<Vec<usize>>, Vec<Vec<usize>>) {
    let mut parts = input.split("\n\n");
    let patterns: HashSet<Vec<usize>> = parts
        .next()
        .unwrap()
        .split(", ")
        .map(|pat| pat.chars().map(char_to_digit).collect())
        .collect();
    let designs: Vec<Vec<usize>> = parts
        .next()
        .unwrap()
        .lines()
        .map(|des| des.chars().map(char_to_digit).collect())
        .collect();
    (patterns, designs)
}

fn can_make(
    design: &[usize],
    patterns: &HashSet<Vec<usize>>,
    cache: &mut HashMap<Vec<usize>, bool>,
) -> bool {
    if design.is_empty() {
        return true;
    }
    if let Some(&result) = cache.get(design) {
        return result;
    }

    let result = patterns
        .iter()
        .filter(|&pattern| design.starts_with(pattern))
        .any(|pattern| can_make(&design[pattern.len()..], patterns, cache));

    cache.insert(design.to_vec(), result);
    result
}

#[aoc(day19, part1)]
fn part1((patterns, designs): &(HashSet<Vec<usize>>, Vec<Vec<usize>>)) -> usize {
    let mut cache: HashMap<Vec<usize>, bool> = HashMap::new();
    let mut count = 0;

    for design in designs {
        if can_make(design, patterns, &mut cache) {
            count += 1;
        }
    }
    count
}

fn count_ways(design: &[usize], patterns: &HashSet<Vec<usize>>, cache: &mut HashMap<Vec<usize>, usize>) -> usize {
    if design.is_empty() {
        return 1;
    }
    if let Some(&count) = cache.get(design) {
        return count;
    }

    let count = patterns.iter()
        .filter(|&pattern| design.starts_with(pattern))
        .map(|pattern| count_ways(&design[pattern.len()..], patterns, cache))
        .sum();

    cache.insert(design.to_vec(), count);
    count
}

#[aoc(day19, part2)]
fn part2((patterns, designs): &(HashSet<Vec<usize>>, Vec<Vec<usize>>)) -> usize {
    let mut cache: HashMap<Vec<usize>, usize> = HashMap::new();
    let mut count = 0;

    for design in designs {
        count += count_ways(design, patterns, &mut cache);
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "r, wr, b, g, bwu, rb, gb, br\n\
                                 \n\
                                 brwrr\n\
                                 bggr\n\
                                 gbbr\n\
                                 rrbgbr\n\
                                 ubwu\n\
                                 bwurrg\n\
                                 brgr\n\
                                 bbrgwb";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 6);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 16);
    }
}

