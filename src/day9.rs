use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{BTreeSet, HashMap};

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<(usize, usize)> {
    let mut parsed: Vec<(usize, usize)> = Vec::new();
    let mut blk = 0;
    let mut iter = input.chars().enumerate();
    loop {
        match iter.next() {
            Some((i, c)) => {
                let val = c.to_digit(10).unwrap() as usize;
                if (i % 2) == 0 {
                    blk = val;
                } else {
                    parsed.push((blk, val));
                }
            }
            None => {
                parsed.push((blk, 0));
                break;
            }
        }
    }
    parsed
}

#[aoc(day9, part1)]
fn part1(input: &[(usize, usize)]) -> usize {
    let mut res: Vec<usize> = Vec::new();
    let mut j = input.len() - 1;
    let (mut curr_tail_blk, _curr_tail_free) = input[j];

    for (i, (blk, mut free)) in input.iter().enumerate() {
        if i == j {
            if curr_tail_blk > 0 {
                res.extend(vec![j; curr_tail_blk]);
            }
            break;
        }
        let curr_blk = vec![i; *blk];
        res.extend(curr_blk);
        while free > 0 {
            if free >= curr_tail_blk {
                free -= curr_tail_blk;
                res.extend(vec![j; curr_tail_blk]);
                j -= 1;
                curr_tail_blk = input[j].0;
            } else {
                curr_tail_blk -= free;
                res.extend(vec![j; free]);
                free = 0;
            }
        }
    }
    res.iter().enumerate().map(|(i, v)| i * v).sum()
}

#[aoc(day9, part2)]
fn part2(input: &[(usize, usize)]) -> usize {
    // hashmap with block_id and block range: start_pos, end_pos
    let mut block_pos: HashMap<usize, (usize, usize)> = HashMap::new();
    // BTreeSet with the blocks of free space
    let mut free_block: BTreeSet<(usize, usize)> = BTreeSet::new();
    let mut curr_pos = 0;
    for (i, (blk_size, free)) in input.iter().enumerate() {
        block_pos.insert(i, (curr_pos, curr_pos + blk_size));
        curr_pos += blk_size;
        if free > &0 {
            free_block.insert((curr_pos, curr_pos + free));
            curr_pos += free;
        }
    }

    let mut j = input.len() - 1;
    while j > 0 {
        let (start_pos, end_pos) = block_pos.get(&j).unwrap();
        let blk_len = end_pos - start_pos;
        let mut v_opt = None;
        for (x, y) in free_block.iter() {
            // no need to continue
            if x > start_pos {
                break;
            }
            // find a free block that can fit
            if (y - x) >= blk_len {
                v_opt = Some((*x, *y));
                break;
            }
        }
        if let Some(v) = v_opt {
            let new_end = v.0 + blk_len;
            block_pos.insert(j, (v.0, new_end));
            free_block.remove(&v);
            if new_end < v.1 {
                // there is still some remaning free space in that block
                free_block.insert((new_end, v.1));
            }
        }
        j -= 1;
    }

    block_pos
        .into_iter()
        .map(|(k, (start, end))| {
            let mut acc = 0;
            for i in start..end {
                acc += k * i;
            }
            acc
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "2333133121414131402";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 1928);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 2858);
    }
}
