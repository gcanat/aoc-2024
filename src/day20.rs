use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

type Grid = Vec<Vec<char>>;
type PathMap = HashMap<(i32, i32), i32>;

struct RaceInfo {
    start: Pos,
    end: Pos,
    dim: (i32, i32),
}

#[derive(Clone, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}

#[aoc_generator(day20)]
fn parse(input: &str) -> (Grid, RaceInfo) {
    let n_rows = input.lines().count();
    let n_cols = input.lines().next().unwrap().chars().count();
    let mut start = Pos { x: 0, y: 0 };
    let mut end = Pos { x: 0, y: 0 };
    let mut grid: Grid = vec![vec!['#'; n_cols]; n_rows];
    for (i, l) in input.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            if c == 'S' {
                start.x = i as i32;
                start.y = j as i32;
            } else if c == 'E' {
                end.x = i as i32;
                end.y = j as i32;
            }
            grid[i][j] = c;
        }
    }
    (
        grid,
        RaceInfo {
            start,
            end,
            dim: (n_rows as i32, n_cols as i32),
        },
    )
}

#[aoc(day20, part1)]
fn part1((grid, race_info): &(Grid, RaceInfo)) -> usize {
    solve(grid, race_info, 100, 2)
}

#[aoc(day20, part2)]
fn part2((grid, race_info): &(Grid, RaceInfo)) -> usize {
    solve(grid, race_info, 100, 20)
}

fn solve(grid: &Grid, race_info: &RaceInfo, min_save: i32, max_iter: i32) -> usize {
    let (h, w) = race_info.dim;
    let (start, end) = (race_info.start.clone(), race_info.end.clone());

    let mut path_map: PathMap = HashMap::new();
    path_map.insert((end.x, end.y), 0);
    let mut dist = 0;
    let mut curr_pos = end;
    let mut curr_dir = (0, 0);
    loop {
        for (dx, dy) in DIRECTIONS.iter() {
            let next_pos = Pos {
                x: curr_pos.x + dx,
                y: curr_pos.y + dy,
            };
            // stay inside the grid, dont run into a wall & dont go backwards
            if bound_check(&next_pos, (h, w))
                && !is_wall(&next_pos, grid)
                && get_opposite(dx, dy) != curr_dir
            {
                dist += 1;
                path_map.insert((next_pos.x, next_pos.y), dist);
                curr_pos = next_pos;
                curr_dir = (*dx, *dy);
                break;
            }
        }
        if curr_pos == start {
            break;
        }
    }

    let mut savings: Vec<i32> = Vec::new();

    for ((x, y), dist1) in path_map.iter() {
        let pos1 = Pos { x: *x, y: *y };
        for cheat_duration in 2..(max_iter + 1) {
            find_cheat(&pos1, dist1, &path_map, &mut savings, cheat_duration);
        }
    }

    savings
        .iter()
        .filter(|v| *v > &(min_save - 1))
        .count()
}

fn bound_check(p: &Pos, (h, w): (i32, i32)) -> bool {
    (p.x > 0) && (p.x < h - 1) && (p.y > 0) && (p.y < w - 1)
}

fn is_wall(p: &Pos, grid: &Grid) -> bool {
    grid[p.x as usize][p.y as usize] == '#'
}

fn get_opposite(dx: &i32, dy: &i32) -> (i32, i32) {
    let mut opp = (0, 0);
    if dx != &0 {
        opp.0 = -dx;
    } else if dy != &0 {
        opp.1 = -dy;
    }
    opp
}

fn find_cheat(
    pos1: &Pos,
    dist1: &i32,
    path_map: &PathMap,
    savings: &mut Vec<i32>,
    cheat_duration: i32,
) {
    // Find all positions that are at `cheat_duration` distance
    for x in 0..(cheat_duration + 1) {
        let y = cheat_duration - x;
        for (i, (xm, ym)) in [(1, 1), (-1, 1), (1, -1), (-1, -1)].iter().enumerate() {
            // when x or y is 0 we can skip some redundant (xm, ym)
            if ((x == 0) && ((i == 1) || (i == 3))) || ((y == 0) && ((i == 2) || (i == 3))) {
                continue;
            }
            let pos2 = Pos {
                x: pos1.x + (x * xm),
                y: pos1.y + (y * ym),
            };
            // does this point to a position in path_map ?
            if let Some(dist2) = path_map.get(&(pos2.x, pos2.y)) {
                // only test one way
                if dist1 < dist2 {
                    continue;
                }
                // is it a potential shortcut ?
                // ie, L1 distance should be smaller than (dist1 - dist2 - cheat_duration)
                let diff = dist1 - dist2 - cheat_duration;
                let l1_dist = pos1.x.abs_diff(pos2.x) + pos1.y.abs_diff(pos2.y);
                if (l1_dist as i32) < diff {
                    savings.push(diff);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "###############\n\
                                 #...#...#.....#\n\
                                 #.#.#.#.#.###.#\n\
                                 #S#...#.#.#...#\n\
                                 #######.#.#.###\n\
                                 #######.#.#...#\n\
                                 #######.#.###.#\n\
                                 ###..E#...#...#\n\
                                 ###.#######.###\n\
                                 #...###...#...#\n\
                                 #.#####.#.###.#\n\
                                 #.#...#.#.#...#\n\
                                 #.#.#.#.#.#.###\n\
                                 #...#...#...###\n\
                                 ###############";

    #[test]
    fn part1_example() {
        let (grid, race_info) = parse(INPUT);
        assert_eq!(solve(&grid, &race_info, 64, 2), 1);
    }

    #[test]
    fn part2_example() {
        let (grid, race_info) = parse(INPUT);
        assert_eq!(solve(&grid, &race_info, 72, 20), 29);
    }
}
