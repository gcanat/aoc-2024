use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

type Grid = Vec<Vec<char>>;
type Move = (isize, isize);

fn parse_moves(moves_input: &str) -> Vec<Move> {
    let n_rows = moves_input.lines().count();
    let n_cols = moves_input.lines().next().unwrap().chars().count();
    let mut moves: Vec<Move> = Vec::with_capacity(n_rows * n_cols);
    for l in moves_input.lines() {
        for c in l.chars() {
            let next_move = match c {
                '<' => (0, -1),
                'v' => (1, 0),
                '^' => (-1, 0),
                '>' => (0, 1),
                _ => (0, 0),
            };
            moves.push(next_move);
        }
    }
    moves
}

#[aoc_generator(day15, part1)]
fn parse1(input: &str) -> (Grid, Move, Vec<Move>) {
    let (grid_input, moves_input) = input.split_once("\n\n").unwrap();

    let n_rows = grid_input.lines().count();
    let n_cols = grid_input.lines().next().unwrap().chars().count();
    let mut grid: Grid = vec![vec!['.'; n_cols]; n_rows];
    let mut curr_pos = (0, 0);
    for (i, l) in grid_input.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            if (c == '#') || (c == 'O') || (c == '@') {
                grid[i][j] = c;
            }
            if c == '@' {
                curr_pos = (i as isize, j as isize);
            }
        }
    }
    let moves = parse_moves(moves_input);

    (grid, curr_pos, moves)
}

#[aoc_generator(day15, part2)]
fn parse2(input: &str) -> (Grid, Move, Vec<Move>) {
    let (grid_input, moves_input) = input.split_once("\n\n").unwrap();
    let grid_input = grid_input
        .replace(".", "..")
        .replace("O", "[]")
        .replace("#", "##")
        .replace("@", "@.");
    let n_rows = grid_input.lines().count();
    let n_cols = grid_input.lines().next().unwrap().chars().count();
    let mut grid: Grid = vec![vec!['.'; n_cols]; n_rows];
    let mut curr_pos = (0, 0);
    for (i, l) in grid_input.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            grid[i][j] = c;
            if c == '@' {
                curr_pos = (i as isize, j as isize);
            }
        }
    }
    let moves = parse_moves(moves_input);
    (grid, curr_pos, moves)
}

fn make_move(
    curr_pos: &Move,
    next_move: &Move,
    grid: &mut Grid,
    mut robot_pos: Move,
    move_ok: &mut bool,
) -> Move {
    let curr_item = grid[curr_pos.0 as usize][curr_pos.1 as usize];
    let next_pos = (curr_pos.0 + next_move.0, curr_pos.1 + next_move.1);
    let next_item = grid[next_pos.0 as usize][next_pos.1 as usize];
    if next_item == '#' {
        *move_ok = false;
    } else if next_item == 'O' {
        let _ = make_move(&next_pos, next_move, grid, robot_pos, move_ok);
    }
    if *move_ok {
        grid[next_pos.0 as usize][next_pos.1 as usize] = curr_item;
        if curr_item == '@' {
            robot_pos = (next_pos.0, next_pos.1);
            grid[curr_pos.0 as usize][curr_pos.1 as usize] = '.';
        }
    }
    robot_pos
}

fn move_robot(curr_pos: &mut Move, next_move: &Move, grid: &mut Grid, move_ok: &mut bool) {
    let mut box_to_move: HashMap<Move, char> = HashMap::new();
    let next_pos = (curr_pos.0 + next_move.0, curr_pos.1 + next_move.1);
    let next_item = grid[next_pos.0 as usize][next_pos.1 as usize];
    if next_item == '#' {
        *move_ok = false;
    } else if (next_item == ']') || (next_item == '[') {
        box_to_move.insert(next_pos, next_item);
        explore_box(&next_pos, next_move, grid, move_ok, &mut box_to_move);
    }
    if *move_ok {
        move_boxes(&box_to_move, next_move, grid);
        grid[next_pos.0 as usize][next_pos.1 as usize] = '@';
        grid[curr_pos.0 as usize][curr_pos.1 as usize] = '.';
        *curr_pos = next_pos;
    }
}

fn move_boxes(boxes: &HashMap<Move, char>, direction: &Move, grid: &mut Grid) {
    // reset all positions
    boxes.iter().for_each(|(pos, c)| {
        grid[pos.0 as usize][pos.1 as usize] = '.';
        if c == &'[' {
            grid[pos.0 as usize][(pos.1 + 1) as usize] = '.';
        } else {
            grid[pos.0 as usize][(pos.1 - 1) as usize] = '.';
        }
    });
    // update new positions
    boxes.iter().for_each(|(pos, c)| {
        grid[(pos.0 + direction.0) as usize][(pos.1 + direction.1) as usize] = *c;
        if c == &'[' {
            grid[(pos.0 + direction.0) as usize][(pos.1 + 1 + direction.1) as usize] = ']';
        } else {
            grid[(pos.0 + direction.0) as usize][(pos.1 - 1 + direction.1) as usize] = '[';
        }
    });
}

fn explore_box(
    pos: &Move,
    next_move: &Move,
    grid: &Grid,
    move_ok: &mut bool,
    box_to_move: &mut HashMap<Move, char>,
) {
    if next_move.0 == 0 {
        // Horizontal direction, only one box can be moved.
        // But they have a width of 2 so need to account for that.
        let next_pos = (pos.0, pos.1 + next_move.1 * 2);
        let next_item = grid[next_pos.0 as usize][next_pos.1 as usize];
        if next_item == '#' {
            *move_ok = false;
        } else if (next_item == ']') || (next_item == '[') {
            box_to_move.insert(next_pos, next_item);
            explore_box(&next_pos, next_move, grid, move_ok, box_to_move);
        }
    } else {
        // Vertical move. Can move multiple boxes.
        let curr_item = grid[pos.0 as usize][pos.1 as usize];
        // adjust current pos to '['
        let pos = (pos.0, if curr_item == '[' { pos.1 } else { pos.1 - 1 });
        let mut to_explore: Vec<Move> = Vec::new();
        // check what's next
        for i in [-1, 0, 1].iter() {
            let next_pos = (pos.0 + next_move.0, pos.1 + i);
            let next_item = grid[next_pos.0 as usize][next_pos.1 as usize];
            if (i != &-1) && (next_item == '#') {
                *move_ok = false;
            } else if next_item == '[' {
                box_to_move.insert(next_pos, next_item);
                to_explore.push(next_pos);
            }
        }
        if !to_explore.is_empty() {
            for nxt_pos in to_explore.iter() {
                explore_box(nxt_pos, next_move, grid, move_ok, box_to_move);
            }
        }
    }
}

fn compute_gps(grid: &Grid, c: char) -> usize {
    let mut total = 0;
    let n_row = grid.len();
    let n_col = grid[0].len();
    for (i, row) in grid.iter().enumerate().take(n_row) {
        for (j, cc) in row.iter().enumerate().take(n_col) {
            if cc == &c {
                total += 100 * i + j;
            }
        }
    }
    total
}

#[aoc(day15, part1)]
fn part1((grid, curr_pos, moves): &(Grid, Move, Vec<Move>)) -> usize {
    let mut grid = grid.clone();
    let mut robot_pos = *curr_pos;
    for mv in moves.iter() {
        robot_pos = make_move(&robot_pos, mv, &mut grid, robot_pos, &mut true);
    }
    compute_gps(&grid, 'O')
}

#[aoc(day15, part2)]
fn part2((grid, curr_pos, moves): &(Grid, Move, Vec<Move>)) -> usize {
    let mut grid = grid.clone();
    let mut robot_pos = *curr_pos;
    for mv in moves.iter() {
        let mut move_ok = true;
        move_robot(&mut robot_pos, mv, &mut grid, &mut move_ok);
    }
    compute_gps(&grid, '[')
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "##########\n\
                                 #..O..O.O#\n\
                                 #......O.#\n\
                                 #.OO..O.O#\n\
                                 #..O@..O.#\n\
                                 #O#..O...#\n\
                                 #O..O..O.#\n\
                                 #.OO.O.OO#\n\
                                 #....O...#\n\
                                 ##########\n\
                                 \n\
                                 <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^\n\
                                 vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v\n\
                                 ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<\n\
                                 <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^\n\
                                 ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><\n\
                                 ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^\n\
                                 >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^\n\
                                 <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>\n\
                                 ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>\n\
                                 v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse1(INPUT)), 10092);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse2(INPUT)), 9021);
    }

    const INPUT2: &'static str = "#######\n\
                                 #...#.#\n\
                                 #.....#\n\
                                 #.....#\n\
                                 #.....#\n\
                                 #.....#\n\
                                 #.OOO@#\n\
                                 #.OOO.#\n\
                                 #..O..#\n\
                                 #.....#\n\
                                 #.....#\n\
                                 #######\n\
                                 \n\
                                 v<vv<<^^^^^";

    #[test]
    fn part2_example2() {
        assert_eq!(part2(&parse2(INPUT2)), 2339);
    }

    const INPUT3: &'static str = "########\n\
                                 #......#\n\
                                 #OO....#\n\
                                 #.O....#\n\
                                 #.O....#\n\
                                 ##O....#\n\
                                 #O..O@.#\n\
                                 #......#\n\
                                 ########\n\
                                 \n\
                                 <^^<<>^^^<v";

    #[test]
    fn part2_example3() {
        assert_eq!(part2(&parse2(INPUT3)), 2827);
    }

    const INPUT4: &'static str = "######\n\
                                 #....#\n\
                                 #..#.#\n\
                                 #....#\n\
                                 #.O..#\n\
                                 #.OO@#\n\
                                 #.O..#\n\
                                 #....#\n\
                                 ######\n\
                                 \n\
                                 <vv<<^^^";

    #[test]
    fn part2_example4() {
        assert_eq!(part2(&parse2(INPUT4)), 1216);
    }
}
