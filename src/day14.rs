use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

type Xy = Vec<[isize; 2]>;

#[aoc_generator(day14)]
fn parse(input: &str) -> (Xy, Xy) {
    let mut pos: Xy = Vec::new();
    let mut veloc: Xy = Vec::new();
    for l in input.lines() {
        let (p, v) = l.split_once(" ").unwrap();
        let p_clean = p.replace("p=", "");
        let v_clean = v.replace("v=", "");
        let val = p_clean.split_once(",").unwrap();
        pos.push([
            val.0.parse::<isize>().unwrap(),
            val.1.parse::<isize>().unwrap(),
        ]);
        let val = v_clean.split_once(",").unwrap();
        veloc.push([
            val.0.parse::<isize>().unwrap(),
            val.1.parse::<isize>().unwrap(),
        ]);
    }
    (pos, veloc)
}

fn check_bound(curr_pos: isize, grid_size: &isize) -> isize {
    if curr_pos < 0 {
        grid_size + curr_pos
    } else if curr_pos > (grid_size - 1) {
        curr_pos - grid_size
    } else {
        curr_pos
    }
}

fn move_robot(curr_pos: &[isize; 2], veloc: &[isize; 2], grid_size: &[isize; 2]) -> [isize; 2] {
    [
        check_bound(curr_pos[0] + veloc[0], &grid_size[0]),
        check_bound(curr_pos[1] + veloc[1], &grid_size[1]),
    ]
}

fn get_quadrant(curr_pos: &[isize; 2], grid_size: &[isize; 2]) -> Option<char> {
    let split_point = ((grid_size[0] - 1) / 2, (grid_size[1] - 1) / 2);
    if (curr_pos[0] == split_point.0) || (curr_pos[1] == split_point.1) {
        return None;
    }
    let x_pos = curr_pos[0] < split_point.0;
    let y_pos = curr_pos[1] < split_point.1;
    if x_pos && y_pos {
        return Some('a');
    } else if x_pos && !y_pos {
        return Some('b');
    } else if !x_pos && y_pos {
        return Some('c');
    } else {
        return Some('d');
    }
}

#[aoc(day14, part1)]
fn part1((pos, veloc): &(Xy, Xy)) -> usize {
    let mut grid_size: [isize; 2] = [101, 103];
    if pos.len() == 12 {
        grid_size = [11, 7];
    }
    let mut quad_count: HashMap<char, usize> = HashMap::new();

    let _ = pos
        .iter()
        .zip(veloc)
        .map(|(p, v)| {
            let mut curr_pos = *p;
            for _ in 0..100 {
                curr_pos = move_robot(&curr_pos, v, &grid_size);
            }
            let curr_quad = get_quadrant(&curr_pos, &grid_size);
            match curr_quad {
                Some(c) => {
                    quad_count.entry(c).and_modify(|e| *e += 1).or_insert(1);
                }
                None => {}
            }
        })
        .collect::<Vec<_>>();
    quad_count.iter().fold(1, |acc, (_k, v)| acc * v)
}

fn compute_std(pos: &[[isize; 2]]) -> f64 {
    let n = pos.len();
    let mean_pos = pos
        .iter()
        .fold([0, 0], |acc, v| [acc[0] + v[0], acc[1] + v[1]]);
    let mean_pos = [mean_pos[0] as f64 / n as f64, mean_pos[1] as f64 / n as f64];
    let dist = pos.iter().fold(0., |acc, v| {
        acc + (mean_pos[0] - v[0] as f64).abs() + (mean_pos[1] - v[1] as f64).abs()
    });
    dist / n as f64
}

#[aoc(day14, part2)]
fn part2((pos, veloc): &(Xy, Xy)) -> usize {
    let grid_size: [isize; 2] = [101, 103];
    let mut all_pos = pos.clone();
    let mut low_std = f64::MAX;
    let mut best_iter = 0;
    for k in 0..8000 {
        for i in 0..all_pos.len() {
            all_pos[i] = move_robot(&all_pos[i], &veloc[i], &grid_size);
        }
        let new_std = compute_std(&all_pos);
        if new_std < low_std {
            best_iter = k + 1;
            low_std = new_std;
        }
    }
    best_iter
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "p=0,4 v=3,-3\n\
                                 p=6,3 v=-1,-3\n\
                                 p=10,3 v=-1,2\n\
                                 p=2,0 v=2,-1\n\
                                 p=0,0 v=1,3\n\
                                 p=3,0 v=-2,-2\n\
                                 p=7,6 v=-1,-3\n\
                                 p=3,0 v=-1,-2\n\
                                 p=9,3 v=2,3\n\
                                 p=7,3 v=-1,2\n\
                                 p=2,4 v=2,-3\n\
                                 p=9,5 v=-3,-3";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 12);
    }
}
