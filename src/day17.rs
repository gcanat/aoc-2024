use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::VecDeque;

#[aoc_generator(day17)]
fn parse(input: &str) -> ([i64; 3], Vec<i64>) {
    let mut abc: [i64; 3] = [0, 0, 0];
    let mut program: Vec<i64> = Vec::new();
    for l in input.lines() {
        if let Some((head, num)) = l.split_once(":") {
            if head == "Register A" {
                println!("A: {}", num);
                abc[0] = num.trim().parse().unwrap();
            } else if head == "Register B" {
                println!("B: {}", num);
                abc[1] = num.trim().parse().unwrap();
            } else if head == "Register C" {
                println!("C: {}", num);
                abc[2] = num.trim().parse().unwrap();
            } else if head == "Program" {
                println!("Program: {}", num);
                program = num
                    .trim()
                    .split(',')
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect();
            }
        }
    }
    (abc, program)
}

struct Computer {
    a: i64,
    b: i64,
    c: i64,
    ip: usize,
    program: Vec<i64>,
    output: VecDeque<i64>,
}

impl Computer {
    fn new(program: Vec<i64>, abc: [i64; 3]) -> Self {
        Computer {
            a: abc[0],
            b: abc[1],
            c: abc[2],
            ip: 0,
            program,
            output: VecDeque::new(),
        }
    }

    fn get_combo_value(&self, operand: i64) -> i64 {
        match operand {
            0..=3 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Invalid combo operand"),
        }
    }

    fn run(&mut self) {
        while self.ip < self.program.len() {
            let opcode = self.program[self.ip];
            let operand = self.program[self.ip + 1];

            match opcode {
                0 => self.a >>= self.get_combo_value(operand),
                1 => self.b ^= operand,
                2 => self.b = self.get_combo_value(operand) % 8,
                3 => {
                    if self.a != 0 {
                        self.ip = operand as usize;
                        continue;
                    }
                }
                4 => self.b ^= self.c,
                5 => self.output.push_back(self.get_combo_value(operand) % 8),
                6 => self.b = self.a >> self.get_combo_value(operand),
                7 => self.c = self.a >> self.get_combo_value(operand),
                _ => panic!("Invalid opcode"),
            }
            self.ip += 2;
        }
    }
}

fn find_initial_a(program: Vec<i64>) -> i64 {
    let mut a = 0;
    for i in (0..program.len()).rev() {
        a <<= 3;
        loop {
            let mut computer = Computer::new(program.clone(), [a, 0, 0]);
            computer.run();
            let result = &computer.output.iter().copied().collect::<Vec<_>>();
            if result != &program[i..] {
                a += 1;
            } else {
                break;
            }
        }
    }
    a
}

#[aoc(day17, part1)]
fn part1((abc, program): &([i64; 3], Vec<i64>)) -> String {
    let mut computer = Computer::new(program.to_vec(), *abc);
    computer.run();
    computer
        .output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

#[aoc(day17, part2)]
fn part2((_abc, program): &([i64; 3], Vec<i64>)) -> i64 {
    find_initial_a(program.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &'static str = "Register A: 729\n\
                                  Register B: 0\n\
                                  Register C: 0\n\
                                  \n\
                                  Program: 0,1,5,4,3,0";

    const INPUT2: &'static str = "Register A: 2024\n\
                                 Register B: 0\n\
                                 Register C: 0\n\
                                 \n\
                                 Program: 0,3,5,4,3,0";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT1)), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT2)), 117440);
    }
}
