use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use num::integer::gcd;
use std::collections::{HashMap, HashSet};

type Antennas = HashMap<char, Vec<(i32, i32)>>;

/// Return true if point coords are within grid bounds
fn validate_point(x: &(i32, i32), grid_size: &i32) -> bool {
    !((x.0 < 0) || (x.0 > (grid_size - 1)) || (x.1 < 0) || (x.1 > (grid_size - 1)))
}

/// Given to antenna coords, find the 2 antinodes
fn find_antinode(x: &(i32, i32), y: &(i32, i32), grid_size: &i32) -> Vec<(i32, i32)> {
    // let ant_dist = distance(x, y);
    let mut antinodes: Vec<(i32, i32)> = Vec::new();
    let ant_vec = ((x.0 - y.0), (x.1 - y.1));
    let point1 = ((x.0 + ant_vec.0), (x.1 + ant_vec.1));
    let point2 = ((y.0 - ant_vec.0), (y.1 - ant_vec.1));
    if validate_point(&point1, grid_size) {
        antinodes.push(point1);
    }
    if validate_point(&point2, grid_size) {
        antinodes.push(point2);
    }
    antinodes
}

#[aoc_generator(day8)]
fn parse(input: &str) -> (Antennas, usize) {
    let mut antennas: Antennas = HashMap::new();
    for (i, l) in input.lines().enumerate() {
        let _ = l
            .chars()
            .enumerate()
            .map(|(j, c)| {
                if c != '.' {
                    antennas
                        .entry(c)
                        .and_modify(|v| v.push((i as i32, j as i32)))
                        .or_insert(vec![(i as i32, j as i32)]);
                }
            })
            .collect_vec();
    }
    (antennas, input.lines().count())
}

#[aoc(day8, part1)]
fn part1(input: &(Antennas, usize)) -> usize {
    let grid_size = input.1 as i32;
    let mut uniq_antinodes: HashSet<(i32, i32)> = HashSet::new();
    for (_k, v) in input.0.iter() {
        if v.len() < 2 {
            continue;
        }
        for combi in v.iter().combinations(2) {
            let antinodes = find_antinode(combi[0], combi[1], &grid_size);
            for a in antinodes.iter() {
                uniq_antinodes.insert(*a);
            }
        }
    }
    uniq_antinodes.len()
}

fn find_antinode2(x: &(i32, i32), y: &(i32, i32), grid_size: &i32) -> Vec<(i32, i32)> {
    let mut antinodes: Vec<(i32, i32)> = Vec::new();
    // we use first point as the base. This point is an antinode.
    antinodes.push(*x);
    let ant_vec = ((x.0 - y.0), (x.1 - y.1));

    // Find the smallest "unit" vector for this direction
    let vec_gcd = gcd(ant_vec.0, ant_vec.1);
    let ant_vec = (ant_vec.0 / vec_gcd, ant_vec.1 / vec_gcd);

    // iterate in one direction and stop when we are out of bound
    let mut curr_point = *x;
    loop {
        curr_point = ((curr_point.0 + ant_vec.0), (curr_point.1 + ant_vec.1));
        if validate_point(&curr_point, grid_size) {
            antinodes.push(curr_point);
        } else {
            break;
        }
    }
    // iterate in the opposite direction until we are out of bound
    let mut curr_point = *x;
    loop {
        curr_point = ((curr_point.0 - ant_vec.0), (curr_point.1 - ant_vec.1));
        if validate_point(&curr_point, grid_size) {
            antinodes.push(curr_point);
        } else {
            break;
        }
    }
    antinodes
}

#[aoc(day8, part2)]
fn part2(input: &(Antennas, usize)) -> usize {
    let grid_size = input.1 as i32;
    let mut uniq_antinodes: HashSet<(i32, i32)> = HashSet::new();
    for (_k, v) in input.0.iter() {
        if v.len() < 2 {
            continue;
        }
        for combi in v.iter().combinations(2) {
            let antinodes = find_antinode2(combi[0], combi[1], &grid_size);
            for a in antinodes.iter() {
                uniq_antinodes.insert(*a);
            }
        }
    }
    uniq_antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "............\n\
                                 ........0...\n\
                                 .....0......\n\
                                 .......0....\n\
                                 ....0.......\n\
                                 ......A.....\n\
                                 ............\n\
                                 ............\n\
                                 ........A...\n\
                                 .........A..\n\
                                 ............\n\
                                 ............";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 14);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 34);
    }
}
