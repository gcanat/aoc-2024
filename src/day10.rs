use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, -1), (0, 1)];

#[aoc_generator(day10)]
fn parse(input: &str) -> (Vec<Vec<i32>>, HashSet<(i32, i32)>) {
    let grid_size = input.lines().count();
    let mut map = vec![vec![-1_i32; grid_size + 2]; grid_size + 2];
    let mut head_trails: HashSet<(i32, i32)> = HashSet::new();

    for (i, l) in input.lines().enumerate() {
        let _ = l
            .chars()
            .enumerate()
            .map(|(j, c)| {
                let val = c.to_digit(10).unwrap() as i32;
                if val == 0 {
                    head_trails.insert(((i + 1) as i32, (j + 1) as i32));
                }
                map[i + 1][j + 1] = val;
            })
            .collect::<Vec<_>>();
    }
    (map, head_trails)
}

fn explore_point(coord: &(i32, i32), grid: &[Vec<i32>], ends: &mut HashSet<(i32, i32)>) -> usize {
    let curr_val = grid[coord.0 as usize][coord.1 as usize];
    if curr_val == 9 {
        ends.insert((coord.0, coord.1));
        return 1;
    }
    let mut valid_paths: Vec<(i32, i32)> = Vec::new();
    for (dx, dy) in DIRECTIONS.iter() {
        let next_x = coord.0 + dx;
        let next_y = coord.1 + dy;
        let next_point = grid[next_x as usize][next_y as usize];
        if next_point - curr_val == 1 {
            valid_paths.push((next_x, next_y));
        }
    }
    if valid_paths.is_empty() {
        return 0;
    }
    let mut count = 0;
    for path in valid_paths {
        count += explore_point(&path, grid, ends);
    }
    count
}

#[aoc(day10, part1)]
fn part1(input: &(Vec<Vec<i32>>, HashSet<(i32, i32)>)) -> usize {
    let mut count = 0;
    let mut trail_iter = input.1.iter();
    let mut ends: HashSet<(i32, i32)> = HashSet::new();
    loop {
        let Some(curr_coord) = trail_iter.next() else {
            break;
        };
        let _ = explore_point(curr_coord, &input.0, &mut ends);
        count += ends.len();
        ends.clear();
    }
    count
}

#[aoc(day10, part2)]
fn part2(input: &(Vec<Vec<i32>>, HashSet<(i32, i32)>)) -> usize {
    let mut count = 0;
    let mut ends: HashSet<(i32, i32)> = HashSet::new();
    let mut trail_iter = input.1.iter();
    loop {
        let Some(curr_coord) = trail_iter.next() else {
            break;
        };
        count += explore_point(curr_coord, &input.0, &mut ends);
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "89010123\n\
                                 78121874\n\
                                 87430965\n\
                                 96549874\n\
                                 45678903\n\
                                 32019012\n\
                                 01329801\n\
                                 10456732";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 36);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 81);
    }
}
