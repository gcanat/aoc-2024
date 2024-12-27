use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

// types for part1
type GridX = Vec<Vec<usize>>;
type SubGridX = [[usize; 7]; 7];
type CoordsX = [[[usize; 2]; 3]; 8];

// types for part2
type GridA = Vec<Vec<usize>>;
type SubGridA = [[usize; 3]; 3];
type CoordsA = [[[usize; 2]; 2]; 2];

// coords for valid XMAS sequence in a 7x7 sub-grid
// assuming X is in the center, ie coord [3,3]
const COORDS: CoordsX = [
    // upper vertical
    [[2, 3], [1, 3], [0, 3]],
    // upper right diag
    [[2, 4], [1, 5], [0, 6]],
    // right horizontal
    [[3, 4], [3, 5], [3, 6]],
    // down right diag
    [[4, 4], [5, 5], [6, 6]],
    // down vertical
    [[4, 3], [5, 3], [6, 3]],
    // down left diag
    [[4, 2], [5, 1], [6, 0]],
    // left horizontal
    [[3, 2], [3, 1], [3, 0]],
    // upper left diag
    [[2, 2], [1, 1], [0, 0]],
];

// coords for cross MAS, ie 'A' is in the center, 'M' or 'S'
// are in the corners.
const COORDS_A: CoordsA = [[[0, 0], [2, 2]], [[2, 0], [0, 2]]];

fn create_lookup() -> HashMap<&'static char, usize> {
    let mut lookup: HashMap<&char, usize> = HashMap::new();
    for (i, c) in ['X', 'M', 'A', 'S'].iter().enumerate() {
        lookup.insert(c, i + 1);
    }
    lookup
}

#[aoc_generator(day4, part1)]
fn parse_p1(input: &str) -> GridX {
    let lookup = create_lookup();
    let grid_size = input.lines().count() + 6;
    // let mut grid = [[0; GRID_SIZE_X]; GRID_SIZE_X];
    let mut grid = vec![vec![0; grid_size]; grid_size];
    for (i, l) in input.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            grid[i + 3][j + 3] = *lookup.get(&c).unwrap_or(&0_usize);
        }
    }
    grid
}

fn is_xmas(slice: &[usize; 3]) -> bool {
    // we already know that we have an 'X' at the begining
    // so we just need to check if the rest is "MAS"
    slice == &[2, 3, 4]
}

fn get_slice(sub_grid: &SubGridX, coords: &[[usize; 2]; 3]) -> [usize; 3] {
    let mut slice = [0_usize; 3];
    for (i, coo) in coords.iter().enumerate() {
        slice[i] = sub_grid[coo[0]][coo[1]];
    }
    slice
}

fn search_subgrid(sub_grid: &SubGridX) -> usize {
    let mut count = 0;
    for coo in &COORDS {
        let slice = get_slice(sub_grid, coo);
        if is_xmas(&slice) {
            count += 1;
        }
    }
    count
}

/// Create a 7x7 SubGrid with X in the center.
fn create_subgridx(input: &GridX, corner: &[usize; 2]) -> SubGridX {
    let mut subgrid: SubGridX = [[0; 7]; 7];
    for i in 0..7 {
        for j in 0..7 {
            subgrid[i][j] = input[corner[0] + i][corner[1] + j];
        }
    }
    subgrid
}

#[aoc(day4, part1)]
fn part1(input: &GridX) -> usize {
    let mut count = 0;
    let n = input.len() - 6;
    for i in 0..n {
        for j in 0..n {
            if input[i + 3][j + 3] == 1 {
                let subgrid = create_subgridx(input, &[i, j]);
                count += search_subgrid(&subgrid);
            }
        }
    }
    count
}

fn is_mas(slice: &[usize; 2]) -> bool {
    // we already know that we have an 'A' in the middle
    // so we just need to check if the rest is "MS" or "SM"
    (slice == &[2, 4]) || (slice == &[4, 2])
}

fn get_slice_a(sub_grid: &SubGridA, coords: &[[usize; 2]; 2]) -> [usize; 2] {
    let mut slice = [0_usize; 2];
    for (i, coo) in coords.iter().enumerate() {
        slice[i] = sub_grid[coo[0]][coo[1]];
    }
    slice
}

fn search_subgrid_a(sub_grid: &SubGridA) -> usize {
    let mut count = 0;
    for coo in &COORDS_A {
        let slice = get_slice_a(sub_grid, coo);
        if is_mas(&slice) {
            count += 1;
        }
    }
    if count == 2 {
        1
    } else {
        0
    }
}

/// Create a 3x3 SubGrid with A in the center.
fn create_subgrid_a(input: &GridA, corner: &[usize; 2]) -> SubGridA {
    let mut subgrid: SubGridA = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            subgrid[i][j] = input[corner[0] + i][corner[1] + j];
        }
    }
    subgrid
}

#[aoc_generator(day4, part2)]
fn parse_p2(input: &str) -> GridA {
    let lookup = create_lookup();
    let grid_size = input.lines().count() + 2;
    let mut grid = vec![vec![0; grid_size]; grid_size];
    for (i, l) in input.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            grid[i + 1][j + 1] = *lookup.get(&c).unwrap_or(&0_usize);
        }
    }
    grid
}

#[aoc(day4, part2)]
fn part2(input: &GridA) -> usize {
    let mut count = 0;
    let n = input.len() - 2;
    for i in 0..n {
        for j in 0..n {
            if input[i + 1][j + 1] == 3 {
                let subgrid = create_subgrid_a(input, &[i, j]);
                count += search_subgrid_a(&subgrid);
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "MMMSXXMASM\n\
                                 MSAMXMSMSA\n\
                                 AMXSXMAAMM\n\
                                 MSAMASMSMX\n\
                                 XMASAMXAMM\n\
                                 XXAMMXXAMA\n\
                                 SMSMSASXSS\n\
                                 SAXAMASAAA\n\
                                 MAMMMXMMMM\n\
                                 MXMXAXMASX";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_p1(INPUT)), 18);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_p2(INPUT)), 9);
    }
}
