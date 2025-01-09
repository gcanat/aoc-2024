use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashSet, VecDeque};

#[aoc_generator(day18, part1)]
fn parse1(input: &str) -> HashSet<(i32, i32)> {
    let n = input.lines().count();
    // accomadate for test case
    let mut take_n = 1024;
    if n < 30 {
        take_n = 12;
    }
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            (x, y)
        })
        .take(take_n)
        .collect()
}

#[aoc_generator(day18, part2)]
fn parse2(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect()
}

fn shortest_path(target: (i32, i32), bounds: [i32; 2], bytes_coord: &HashSet<(i32, i32)>) -> i32 {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back(((0, 0), 0));
    visited.insert((0, 0));
    let mut shortest = 0;
    while let Some((pos, steps)) = queue.pop_front() {
        if pos == target {
            shortest = steps;
            break;
        }

        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let new_pos = (pos.0 + dx, pos.1 + dy);
            if (new_pos.0 > -1)
                && (new_pos.0 < bounds[0])
                && (new_pos.1 > -1)
                && (new_pos.1 < bounds[1])
                && !bytes_coord.contains(&new_pos)
                && !visited.contains(&new_pos)
            {
                queue.push_back((new_pos, steps + 1));
                visited.insert(new_pos);
            }
        }
    }
    shortest
}

#[aoc(day18, part1)]
fn part1(bytes_coord: &HashSet<(i32, i32)>) -> i32 {
    let max = bytes_coord.iter().max_by(|x, y| x.0.cmp(&y.0)).unwrap().0;
    let target = (max, max);
    let bounds = [max + 1, max + 1];
    shortest_path(target, bounds, bytes_coord)
}

#[aoc(day18, part2)]
fn part2(bytes_vec: &[(i32, i32)]) -> String {
    let mut target = (70, 70);
    let mut start = 1025;
    if bytes_vec.len() < 30 {
        target = (6, 6);
        start = 13;
    }
    let bounds = [target.0 + 1, target.1 + 1];
    for i in start..bytes_vec.len() {
        let bytes_coord: HashSet<(i32, i32)> = bytes_vec.iter().take(i).copied().collect();
        let shortest = shortest_path(target, bounds, &bytes_coord);
        if shortest == 0 {
            return format!("{:?}", bytes_vec[i - 1]);
        }
    }
    panic!("No byte found that stopped us from reaching the target");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "5,4\n\
                                 4,2\n\
                                 4,5\n\
                                 3,0\n\
                                 2,1\n\
                                 6,3\n\
                                 2,4\n\
                                 1,5\n\
                                 0,6\n\
                                 3,3\n\
                                 2,6\n\
                                 5,1\n\
                                 1,2\n\
                                 5,5\n\
                                 2,5\n\
                                 6,5\n\
                                 1,4\n\
                                 0,4\n\
                                 6,4\n\
                                 1,1\n\
                                 6,1\n\
                                 1,0\n\
                                 0,5\n\
                                 1,6\n\
                                 2,0";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse1(INPUT)), 22);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse2(INPUT)), "(6, 1)");
    }
}
