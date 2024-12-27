use aoc_runner_derive::{aoc, aoc_generator};

type Matrices = Vec<[usize; 4]>;
type Prizes = Vec<[usize; 2]>;

#[aoc_generator(day13)]
fn parse(input: &str) -> (Matrices, Prizes) {
    let mut matrices: Matrices = Vec::new();
    let mut prizes: Prizes = Vec::new();
    let sections = input.split("\n\n");
    for sec in sections {
        let mut lines = sec.split("\n");

        let a = lines.next().unwrap().split_once(",").unwrap();
        let a0 = a.0.split_once("+").unwrap().1.parse::<usize>().unwrap();
        let a1 = a.1.split_once("+").unwrap().1.parse::<usize>().unwrap();

        let b = lines.next().unwrap().split_once(",").unwrap();
        let b0 = b.0.split_once("+").unwrap().1.parse::<usize>().unwrap();
        let b1 = b.1.split_once("+").unwrap().1.parse::<usize>().unwrap();

        matrices.push([a0, a1, b0, b1]);

        let pr = lines.next().unwrap().split_once(",").unwrap();
        let x = pr.0.split_once("=").unwrap().1.parse::<usize>().unwrap();
        let y = pr.1.split_once("=").unwrap().1.parse::<usize>().unwrap();
        prizes.push([x, y]);
    }
    (matrices, prizes)
}

fn check(x: f64) -> bool {
    if x < 0. {
        false
    } else {
        (x.round() - x).abs() < 10_f64.powi(-3)
    }
}

fn solve(mat: &[usize; 4], y: &[usize; 2]) -> Option<(usize, usize)> {
    let det = (mat[0] as f64 * mat[3] as f64) - (mat[1] as f64 * mat[2] as f64);
    if det == 0. {
        None
    } else {
        let mat_inv: [f64; 4] = [
            mat[3] as f64 / det,
            -(mat[1] as f64) / det,
            -(mat[2] as f64) / det,
            mat[0] as f64 / det,
        ];
        let res_0 = (y[0] as f64 * mat_inv[0]) + (y[1] as f64 * mat_inv[2]);
        let res_1 = (y[0] as f64 * mat_inv[1]) + (y[1] as f64 * mat_inv[3]);
        if check(res_0) && check(res_1) {
            Some((res_0.round() as usize, res_1.round() as usize))
        } else {
            None
        }
    }
}

#[aoc(day13, part1)]
fn part1((matrices, prizes): &(Matrices, Prizes)) -> usize {
    let n = prizes.len();
    let mut total = 0;
    for i in 0..n {
        match solve(&matrices[i], &prizes[i]) {
            None => {}
            Some((a, b)) => {
                total += a * 3 + b;
            }
        }
    }
    total
}

#[aoc(day13, part2)]
fn part2((matrices, prizes): &(Matrices, Prizes)) -> usize {
    let n = prizes.len();
    let mut total = 0;
    for i in 0..n {
        let new_target = [prizes[i][0] + 10000000000000, prizes[i][1] + 10000000000000];
        match solve(&matrices[i], &new_target) {
            None => {}
            Some((a, b)) => {
                total += a * 3 + b;
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = "Button A: X+94, Y+34\n\
                                 Button B: X+22, Y+67\n\
                                 Prize: X=8400, Y=5400\n\
                                 \n\
                                 Button A: X+26, Y+66\n\
                                 Button B: X+67, Y+21\n\
                                 Prize: X=12748, Y=12176\n\
                                 \n\
                                 Button A: X+17, Y+86\n\
                                 Button B: X+84, Y+37\n\
                                 Prize: X=7870, Y=6450\n\
                                 \n\
                                 Button A: X+69, Y+23\n\
                                 Button B: X+27, Y+71\n\
                                 Prize: X=18641, Y=10279";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 480);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 875318608908);
    }
}
