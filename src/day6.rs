use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::fmt;

const GRID_SIZE: usize = 130;
type Map = [[usize; GRID_SIZE]; GRID_SIZE];

#[derive(Debug)]
struct TrailResult {
    count: usize,
    cycle: bool,
    path: Vec<(i32, i32)>,
}

impl fmt::Display for TrailResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.count, self.cycle)
    }
}

impl PartialEq for TrailResult {
    fn eq(&self, other: &Self) -> bool {
        (self.count == other.count) && (self.cycle == other.cycle)
    }
}

#[aoc_generator(day6)]
fn parse(input: &str) -> (Map, (i32, i32)) {
    let mut grid = [[0; GRID_SIZE]; GRID_SIZE];
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

/// Check if next move will lead us to an obstacle.
fn check_next_pos(curr_pos: &(i32, i32), direction: &(i32, i32), grid: &Map) -> bool {
    let next_pos = move_pos(curr_pos, direction);
    grid[next_pos.0 as usize][next_pos.1 as usize] != 1
}

/// Check if we are going outside the map.
fn check_boundaries(curr_pos: &(i32, i32), direction: &(i32, i32), grid_size: &i32) -> bool {
    let next_pos = move_pos(curr_pos, direction);
    (&next_pos.0 == grid_size)
        || (&next_pos.0 < &0)
        || (&next_pos.1 == grid_size)
        || (&next_pos.1 < &0)
}

/// Update the position.
fn move_pos(curr_pos: &(i32, i32), direction: &(i32, i32)) -> (i32, i32) {
    (curr_pos.0 + direction.0, curr_pos.1 + direction.1)
}

/// Simple mapping to know in which direction we need to rotate.
fn direction_mappings() -> HashMap<(i32, i32), (i32, i32)> {
    let mut direction_map: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    direction_map.insert((-1, 0), (0, 1));
    direction_map.insert((0, 1), (1, 0));
    direction_map.insert((1, 0), (0, -1));
    direction_map.insert((0, -1), (-1, 0));
    direction_map
}

/// Keep track of the visited positions and the directions we were going to
/// when passing on that position.
fn visited_map(
    start_pos: &(i32, i32),
    direction: &(i32, i32),
) -> HashMap<(i32, i32), Vec<(i32, i32)>> {
    let mut visited: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
    visited.insert(*start_pos, vec![*direction]);
    visited
}

/// Make sure we move to a position without an obstacle on it.
fn validate_rotation(
    curr_pos: &(i32, i32),
    direction: &mut (i32, i32),
    map: &Map,
    direction_map: &HashMap<(i32, i32), (i32, i32)>,
) -> (i32, i32) {
    let mut rotation_count = 0;
    loop {
        *direction = *direction_map.get(&direction).unwrap();
        if check_next_pos(&curr_pos, &direction, &map) {
            return move_pos(&curr_pos, &direction);
        } else {
            rotation_count += 1;
        }
        if rotation_count > 2 {
            panic!("Surrounded by obstacles, nowhere to go!");
        }
    }
}

#[aoc(day6, part1)]
fn part1((map, guard_pos): &(Map, (i32, i32))) -> TrailResult {
    let grid_size = GRID_SIZE as i32;
    // mapping for direction changes
    let direction_map = direction_mappings();
    let mut direction = (-1_i32, 0_i32);
    let mut curr_pos = *guard_pos;

    // keep track of visited positions
    let mut visited = visited_map(&curr_pos, &direction);

    let mut cycle = false;
    let mut uniq_points: Vec<(i32, i32)> = Vec::new();

    loop {
        // check if next move will put us out of the grid
        if check_boundaries(&curr_pos, &direction, &grid_size) {
            break;
        }
        // check if next move will throw us into an obstacle
        if check_next_pos(&curr_pos, &direction, &map) {
            // no obstacle, we can proceed
            curr_pos = move_pos(&curr_pos, &direction);
        } else {
            // obstacle: we need to change direction but we need to make
            // sure we move to a valid position, otherwise rotate once more.
            curr_pos = validate_rotation(&curr_pos, &mut direction, &map, &direction_map);
        }
        let res = visited.get_mut(&curr_pos);
        match res {
            None => {
                // first time we visit this position
                let _ = visited.insert((curr_pos.0, curr_pos.1), vec![(direction.0, direction.1)]);
                uniq_points.push((curr_pos.0, curr_pos.1));
            }
            Some(dirs) => {
                if dirs.iter().any(|x| x == &direction) {
                    // we already passed through that position and in the same direction
                    // meaning we will start to go in circle :)
                    cycle = true;
                    break;
                } else {
                    // already passed here, but in different direction
                    dirs.push(direction);
                }
            }
        }
    }
    // get all visited points
    TrailResult {
        count: uniq_points.len() + 1,
        cycle,
        path: uniq_points,
    }
}

#[aoc(day6, part2)]
fn part2((map, guard_pos): &(Map, (i32, i32))) -> usize {
    let mut cycle_count = 0;
    let first_trail = part1(&(*map, (guard_pos.0, guard_pos.1)));
    let path = first_trail.path;
    let mut new_map = map.clone();
    for i in 0..path.len() {
        new_map[path[i].0 as usize][path[i].1 as usize] = 1;
        let trail = part1(&(new_map, (guard_pos.0, guard_pos.1)));
        if trail.cycle {
            cycle_count += 1;
        }
        new_map[path[i].0 as usize][path[i].1 as usize] = 0;
    }
    cycle_count
}

#[cfg(test)]
mod tests {
    use super::*;
    // FIXME: need to modify GRID_SIZE to 10 before running the tests!

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
        assert_eq!(
            part1(&parse(INPUT)),
            TrailResult {
                count: 41,
                cycle: false,
                path: vec![(0, 0)],
            }
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 6);
    }
}
