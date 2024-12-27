use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

type Map = Vec<Vec<char>>;

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
const DIAGS: [(i32, i32); 4] = [(-1, 1), (1, 1), (1, -1), (-1, -1)];

#[aoc_generator(day12)]
fn parse(input: &str) -> (Map, HashSet<char>) {
    let grid_size = input.lines().count();
    let mut char_set: HashSet<char> = HashSet::new();
    let mut grid = vec![vec!['.'; grid_size + 2]; grid_size + 2];
    input.lines().enumerate().for_each(|(i, l)| {
        l.chars().enumerate().for_each(|(j, c)| {
            grid[i + 1][j + 1] = c;
            char_set.insert(c);
        })
    });
    (grid, char_set)
}

fn explore_point(
    curr_pos: &(i32, i32),
    grid: &Map,
    visited: &mut HashSet<(i32, i32)>,
    mut tile_count: usize,
    mut border_count: usize,
    mut corner_count: usize,
) -> (usize, usize, usize) {
    let curr_char = grid[curr_pos.0 as usize][curr_pos.1 as usize];
    // number of borders not in contact with same char
    let mut borders = 4;
    // new points of the same char to explore after that
    let mut next_pos_explore: Vec<(i32, i32)> = Vec::new();

    // find first state for is_prev_border
    let prev_dir = DIRECTIONS[3];
    let next_pos = ((curr_pos.0 + prev_dir.0), (curr_pos.1 + prev_dir.1));
    let next_char = grid[next_pos.0 as usize][next_pos.1 as usize];
    let mut is_prev_border = next_char != curr_char;

    for (dir, diag) in DIRECTIONS.iter().zip(DIAGS) {
        let next_pos = ((curr_pos.0 + dir.0), (curr_pos.1 + dir.1));
        let next_char = grid[next_pos.0 as usize][next_pos.1 as usize];
        if next_char == curr_char {
            borders -= 1;
            if visited.get(&next_pos).is_none() {
                visited.insert(next_pos);
                next_pos_explore.push(next_pos);
                tile_count += 1;
            }
            if !is_prev_border {
                // need to check what's the char in the diagonal to the current position
                // to check for "inside" corners
                let diag_pos = ((curr_pos.0 + diag.0), (curr_pos.1 + diag.1));
                let diag_char = grid[diag_pos.0 as usize][diag_pos.1 as usize];
                if diag_char != curr_char {
                    corner_count += 1;
                }
            }
            is_prev_border = false;
        } else {
            if is_prev_border {
                // "outside" corner
                corner_count += 1;
            }
            is_prev_border = true;
        }
    }
    border_count += borders;
    for pos in next_pos_explore.iter() {
        (tile_count, border_count, corner_count) =
            explore_point(pos, grid, visited, tile_count, border_count, corner_count);
    }
    (tile_count, border_count, corner_count)
}

#[aoc(day12, part1)]
fn part1((grid, _char_set): &(Map, HashSet<char>)) -> usize {
    let grid_size = grid.len();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut total = 0;
    for i in 1..(grid_size - 1) {
        for j in 1..(grid_size - 1) {
            let curr_pos = (i as i32, j as i32);
            if !visited.contains(&curr_pos) {
                visited.insert(curr_pos);
                let (tile_count, border_count, _corner_count) =
                    explore_point(&curr_pos, grid, &mut visited, 1, 0, 0);
                total += tile_count * border_count;
            }
        }
    }
    total
}

#[aoc(day12, part2)]
fn part2((grid, _char_set): &(Map, HashSet<char>)) -> usize {
    let grid_size = grid.len();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut total = 0;
    for i in 1..(grid_size - 1) {
        for j in 1..(grid_size - 1) {
            let curr_pos = (i as i32, j as i32);
            if !visited.contains(&curr_pos) {
                visited.insert(curr_pos);
                let (tile_count, _border_count, corner_count) =
                    explore_point(&curr_pos, grid, &mut visited, 1, 0, 0);
                total += tile_count * corner_count;
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "RRRRIICCFF\n\
                                 RRRRIICCCF\n\
                                 VVRRRCCFFF\n\
                                 VVRCCCJFFF\n\
                                 VVVVCJJCFE\n\
                                 VVIVCCJJEE\n\
                                 VVIIICJJEE\n\
                                 MIIIIIJJEE\n\
                                 MIIISIJEEE\n\
                                 MMMISSJEEE";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 1930);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 1206);
    }
}
