use aoc_runner_derive::{aoc, aoc_generator};
use rayon::prelude::*;
use std::collections::HashSet;

// directions in that order: Up, Right, Down, Left.
const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

type Map = Vec<Vec<i32>>;

#[aoc_generator(day6)]
fn parse(input: &str) -> (Map, (i32, i32)) {
    let grid_size = input.lines().count();
    let mut grid = vec![vec![0; grid_size]; grid_size];
    let mut guard_pos = (0, 0);
    for (i, l) in input.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            if c == '#' {
                grid[i][j] = 1;
            } else if c == '^' {
                guard_pos = (i as i32, j as i32);
            }
        }
    }
    (grid, guard_pos)
}

/// Check where next move will lead us.
fn get_next_pos(
    curr_pos: &(i32, i32),
    direction: &(i32, i32),
    grid: &Map,
) -> Option<((i32, i32), bool)> {
    let next_pos = (curr_pos.0 + direction.0, curr_pos.1 + direction.1);
    let grid_size = grid.len() as i32;
    if (&next_pos.0 == &grid_size)
        || (&next_pos.0 < &0)
        || (&next_pos.1 == &grid_size)
        || (&next_pos.1 < &0)
    {
        None
    } else {
        let is_obstacle = grid[next_pos.0 as usize][next_pos.1 as usize] == 1;
        Some((next_pos, is_obstacle))
    }
}

fn solve1((map, guard_pos): &(Map, (i32, i32))) -> HashSet<(i32, i32)> {
    let mut dir_iter = DIRECTIONS.iter().cycle();
    let mut direction = dir_iter.next().unwrap();
    let mut curr_pos = *guard_pos;

    // keep track of visited positions
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((curr_pos.0, curr_pos.1));

    loop {
        let Some((next_pos, is_obstacle)) = get_next_pos(&curr_pos, &direction, &map) else {
            // guard went out of the map
            break;
        };
        // check if next move will throw us into an obstacle
        if !is_obstacle {
            // no obstacle, we can proceed
            curr_pos = next_pos;
            visited.insert(curr_pos);
        } else {
            // obstacle: we need to change direction an let the next iteration advance
            direction = dir_iter.next().unwrap();
        }
    }
    // get all visited points
    visited
}

fn solve2((map, guard_pos): &(Map, (i32, i32))) -> usize {
    let mut dir_iter = DIRECTIONS.iter().cycle();
    let mut direction = dir_iter.next().unwrap();
    let mut curr_pos = *guard_pos;

    // keep track of visited positions and directions
    let mut visited: HashSet<(i32, i32, i32, i32)> = HashSet::new();

    loop {
        let Some((next_pos, is_obstacle)) = get_next_pos(&curr_pos, &direction, &map) else {
            // guard went out of the map
            break;
        };
        // check if next move will throw us into an obstacle
        if !is_obstacle {
            // no obstacle, we can proceed
            curr_pos = next_pos;
        } else {
            // we just check if we previously hit that obstacle coming from the same direction
            // it is much cheaper than checking at every step.
            if visited.contains(&(curr_pos.0, curr_pos.1, direction.0, direction.1)) {
                return 1;
            }
            visited.insert((curr_pos.0, curr_pos.1, direction.0, direction.1));
            // obstacle: we need to change direction an let the next iteration advance
            direction = dir_iter.next().unwrap();
        }
    }
    0
}

#[aoc(day6, part1)]
fn part1(input: &(Map, (i32, i32))) -> usize {
    let visited = solve1(input);
    visited.len()
}

#[aoc(day6, part2)]
fn part2((map, guard_pos): &(Map, (i32, i32))) -> usize {
    let visited = solve1(&(map.clone(), (guard_pos.0, guard_pos.1)));
    visited
        .par_iter()
        .map(|(x, y)| {
            let mut new_map = map.clone();
            new_map[*x as usize][*y as usize] = 1;
            solve2(&(new_map, (guard_pos.0, guard_pos.1)))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "....#.....\n\
                                 .........#\n\
                                 ..........\n\
                                 ..#.......\n\
                                 .......#..\n\
                                 ..........\n\
                                 .#..^.....\n\
                                 ........#.\n\
                                 #.........\n\
                                 ......#...";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 41);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 6);
    }
}
