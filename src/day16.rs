use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

type Grid = Vec<Vec<char>>;

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    position: [usize; 2],
    direction: [isize; 2],
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Struct to keep track of the actual path
#[derive(Clone, Eq, PartialEq, Debug)]
struct StateWAnc {
    cost: usize,
    position: [usize; 2],
    direction: [isize; 2],
    ancestors: Vec<[usize; 2]>,
}

impl Ord for StateWAnc {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for StateWAnc {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


#[aoc_generator(day16)]
fn parse(input: &str) -> (Grid, [usize; 2], [usize; 2]) {
    let n_rows = input.lines().count();
    let n_cols = input.lines().next().unwrap().chars().count();
    let mut grid: Grid = vec![vec!['#'; n_cols]; n_rows];
    let mut start: [usize; 2] = [0, 0];
    let mut end: [usize; 2] = [0, 0];
    for (i, l) in input.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            if c != '#' {
                grid[i][j] = c;
                if c == 'S' {
                    start = [i, j];
                } else if c == 'E' {
                    end = [i, j];
                }
            }
        }
    }
    (grid, start, end)
}

fn rotate(dir: &[isize; 2]) -> [[isize; 2]; 3] {
    match *dir {
        [0, 1] | [0, -1] => [*dir, [-1, 0], [1, 0]],
        [1, 0] | [-1, 0] => [*dir, [0, -1], [0, 1]],
        _ => [[0, 0], [0, 0], [0, 0]],
    }
}
fn find_neighbor(curr_state: &State, grid: &Grid) -> Vec<State> {
    let possib_move = rotate(&curr_state.direction);
    let mut moves: Vec<State> = Vec::new();
    for (i, mov) in possib_move.iter().enumerate() {
        let nxt_pos = [
            curr_state.position[0] as isize + mov[0],
            curr_state.position[1] as isize + mov[1],
        ];
        if grid[nxt_pos[0] as usize][nxt_pos[1] as usize] != '#' {
            moves.push(State {
                position: [nxt_pos[0] as usize, nxt_pos[1] as usize],
                direction: [mov[0], mov[1]],
                cost: if i == 0 {
                    curr_state.cost + 1
                } else {
                    curr_state.cost + 1001
                },
            });
        }
    }
    moves
}


fn check_cost(sub_dist: &[usize; 4], dir: &[isize; 2]) -> usize {
    match *dir {
        [0, 1] => sub_dist[0],
        [0, -1] => sub_dist[1],
        [1, 0] => sub_dist[2],
        [-1, 0] => sub_dist[3],
        _ => panic!("bad direction given"),
    }
}
fn update_cost(sub_dist: &mut [usize; 4], dir: &[isize; 2], value: usize) {
    match *dir {
        [0, 1] => {
            sub_dist[0] = value;
        }
        [0, -1] => {
            sub_dist[1] = value;
        }
        [1, 0] => {
            sub_dist[2] = value;
        }
        [-1, 0] => {
            sub_dist[3] = value;
        }
        _ => panic!("bad direction given"),
    }
}

#[aoc(day16, part1)]
fn part1((grid, start, end): &(Grid, [usize; 2], [usize; 2])) -> usize {
    let n_rows = grid.len();
    let n_cols = grid[0].len();

    // for each point there is 4 possible incoming directions
    let mut dist = vec![vec![[usize::MAX; 4]; n_cols]; n_rows];
    dist[start[0]][start[1]] = [0, 0, 0, 0];

    let mut heap = BinaryHeap::new();
    heap.push(State {
        cost: 0,
        position: *start,
        direction: [0, 1],
    });

    while let Some(curr_state) = heap.pop() {
        // first shortest path to target found
        if &curr_state.position == end {
            return curr_state.cost;
        }

        // Important as we may have already found a better way
        let prev_cost = check_cost(
            &dist[curr_state.position[0]][curr_state.position[1]],
            &curr_state.direction,
        );
        if curr_state.cost > prev_cost {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        let edges = find_neighbor(&curr_state, grid);
        for next in edges.iter() {
            // If so, add it to the frontier and continue
            let nxt_cost = check_cost(&dist[next.position[0]][next.position[1]], &next.direction);
            if next.cost < nxt_cost {
                // Relaxation, we have now found a better way
                update_cost(
                    &mut dist[next.position[0]][next.position[1]],
                    &next.direction,
                    next.cost,
                );
                heap.push(next.clone());
            }
        }
    }
    // If we reach this point, means no path to target was found.
    // Should not happen.
    panic!("No path to target found!");
}

/// find neighbors and keep track of ancestors
fn find_neighbor_with_anc(curr_state: &StateWAnc, grid: &Grid) -> Vec<StateWAnc> {
    let possib_move = rotate(&curr_state.direction);
    let mut moves: Vec<StateWAnc> = Vec::new();
    for (i, mov) in possib_move.iter().enumerate() {
        let nxt_pos = [
            curr_state.position[0] as isize + mov[0],
            curr_state.position[1] as isize + mov[1],
        ];
        if grid[nxt_pos[0] as usize][nxt_pos[1] as usize] != '#' {
            let mut new_ancestors = curr_state.ancestors.clone();
            new_ancestors.push(curr_state.position);
            moves.push(StateWAnc {
                position: [nxt_pos[0] as usize, nxt_pos[1] as usize],
                direction: [mov[0], mov[1]],
                cost: if i == 0 {
                    curr_state.cost + 1
                } else {
                    curr_state.cost + 1001
                },
                ancestors: new_ancestors,
            });
        }
    }
    moves
}

#[aoc(day16, part2)]
fn part2((grid, start, end): &(Grid, [usize; 2], [usize; 2])) -> usize {
    let n_rows = grid.len();
    let n_cols = grid[0].len();

    // for each point there is 4 possible incoming directions
    let mut dist = vec![vec![[usize::MAX; 4]; n_cols]; n_rows];
    dist[start[0]][start[1]] = [0, 0, 0, 0];

    let mut heap = BinaryHeap::new();
    heap.push(StateWAnc {
        cost: 0,
        position: *start,
        direction: [0, 1],
        ancestors: Vec::new(),
    });

    let mut path_to_end: Vec<Vec<[usize; 2]>> = Vec::new();
    let mut path_scores: Vec<usize> = Vec::new();

    while let Some(curr_state) = heap.pop() {
        if &curr_state.position == end {
            path_to_end.push(curr_state.ancestors.clone());
            path_scores.push(curr_state.cost);
        }

        let prev_cost = check_cost(
            &dist[curr_state.position[0]][curr_state.position[1]],
            &curr_state.direction,
        );
        if curr_state.cost > prev_cost {
            continue;
        }

        let edges = find_neighbor_with_anc(&curr_state, grid);
        for next in edges.iter() {
            // If so, add it to the frontier and continue
            let nxt_cost = check_cost(&dist[next.position[0]][next.position[1]], &next.direction);
            // relax condition here to find all path with same cost
            if next.cost <= nxt_cost {
                update_cost(
                    &mut dist[next.position[0]][next.position[1]],
                    &next.direction,
                    next.cost,
                );
                heap.push(next.clone());
            }
        }
    }

    let best_score = dist[end[0]][end[1]].iter().min().unwrap();

    // Find unique tiles from path_to_end
    let mut tiles: HashSet<[usize; 2]> = HashSet::new();
    for (path, score) in path_to_end.iter().zip(path_scores) {
        // we only care for paths with best score
        if &score == best_score {
            for pos in path.iter() {
                tiles.insert(*pos);
            }
        }
    }
    // we add one because the target tile is not included
    tiles.len() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &'static str = "###############\n\
                                  #.......#....E#\n\
                                  #.#.###.#.###.#\n\
                                  #.....#.#...#.#\n\
                                  #.###.#####.#.#\n\
                                  #.#.#.......#.#\n\
                                  #.#.#####.###.#\n\
                                  #...........#.#\n\
                                  ###.#.#####.#.#\n\
                                  #...#.....#.#.#\n\
                                  #.#.#.###.#.#.#\n\
                                  #.....#...#.#.#\n\
                                  #.###.#.#.#.#.#\n\
                                  #S..#.....#...#\n\
                                  ###############";

    const INPUT2: &'static str = "#################\n\
                                  #...#...#...#..E#\n\
                                  #.#.#.#.#.#.#.#.#\n\
                                  #.#.#.#...#...#.#\n\
                                  #.#.#.#.###.#.#.#\n\
                                  #...#.#.#.....#.#\n\
                                  #.#.#.#.#.#####.#\n\
                                  #.#...#.#.#.....#\n\
                                  #.#.#####.#.###.#\n\
                                  #.#.#.......#...#\n\
                                  #.#.###.#####.###\n\
                                  #.#.#...#.....#.#\n\
                                  #.#.#.#####.###.#\n\
                                  #.#.#.........#.#\n\
                                  #.#.#.#########.#\n\
                                  #S#.............#\n\
                                  #################";

    const INPUT3: &'static str = "##########\n\
                                  #.......E#\n\
                                  #.##.#####\n\
                                  #..#.....#\n\
                                  ##.#####.#\n\
                                  #S.......#\n\
                                  ##########";

    #[test]
    fn part1_example1() {
        assert_eq!(part1(&parse(INPUT1)), 7036);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1(&parse(INPUT2)), 11048);
    }

    #[test]
    fn part1_example3() {
        assert_eq!(part1(&parse(INPUT3)), 4013);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(part2(&parse(INPUT1)), 45);
    }

    #[test]
    fn part2_example2() {
        assert_eq!(part2(&parse(INPUT2)), 64);
    }

    #[test]
    fn part2_example3() {
        assert_eq!(part2(&parse(INPUT3)), 14);
    }
}
