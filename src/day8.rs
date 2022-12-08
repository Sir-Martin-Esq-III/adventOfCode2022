use core::panic;
use std::ops::Range;

use itertools::{rev, Itertools};

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|ln| ln.chars().map(|ch| ch.to_digit(10).unwrap()).collect_vec())
        .collect_vec()
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn check_direction(
    board: &[Vec<u32>],
    pos: (usize, usize),
    direction: Direction,
    bound: Option<usize>,
) -> bool {
    let current_tree_height = board[pos.1][pos.0];
    let bnd = bound.unwrap_or(0 as usize);

    match direction {
        Direction::Up => {
            for i in pos.1 + 1..bnd {
                if board[i][pos.0] >= current_tree_height {
                    return false;
                }
            }

            return true;
        }
        Direction::Down => {
            for i in (bnd..pos.1).rev() {
                if board[i][pos.0] >= current_tree_height {
                    return false;
                }
            }

            return true;
        }
        Direction::Left => {
            for i in (bnd..pos.0).rev() {
                if board[pos.1][i] >= current_tree_height {
                    return false;
                }
            }
            return true;
        }
        Direction::Right => {
            for i in pos.0 + 1..bnd {
                if board[pos.1][i] >= current_tree_height {
                    return false;
                }
            }

            return true;
        }
    }
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &[Vec<u32>]) -> i32 {
    let mut sum: i32 = 0;

    let width = input[0].len();
    let height = input.len();

    for i in 1..height - 1 {
        for j in 1..width - 1 {
            if check_direction(input, (j, i), Direction::Left, None)
                || check_direction(input, (j, i), Direction::Right, Some(width))
                || check_direction(input, (j, i), Direction::Up, None)
                || check_direction(input, (j, i), Direction::Down, Some(height))
            {
                sum += 1
            }
        }
    }

    sum + 2 * width as i32 + 2 * height as i32 - 4
}

fn check_scenic(
    board: &[Vec<u32>],
    pos: (usize, usize),
    direction: Direction,
    bound: Option<usize>,
) -> i32 {
    let current_tree_height = board[pos.1][pos.0];
    let bnd = bound.unwrap_or(0 as usize);
    let mut sum: i32 = 0;

    match direction {
        Direction::Up => {
            for i in (bnd..pos.1).rev() {
                if board[i][pos.0] >= current_tree_height {
                    return sum + 1;
                }
                sum += 1;
            }

            return sum;
        }
        Direction::Down => {
            for i in pos.1 + 1..bnd {
                if board[i][pos.0] >= current_tree_height {
                    return sum + 1;
                }
                sum += 1;
            }

            return sum;
        }

        Direction::Left => {
            for i in (bnd..pos.0).rev() {
                if board[pos.1][i] >= current_tree_height {
                    return sum + 1;
                }
                sum += 1;
            }

            return sum;
        }
        Direction::Right => {
            for i in pos.0 + 1..bnd {
                if board[pos.1][i] >= current_tree_height {
                    return sum + 1;
                }
                sum += 1;
            }

            return sum;
        }
    }
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &[Vec<u32>]) -> i32 {
    let width = input[0].len();
    let height = input.len();

    let mut best_scenic_score = 0;

    for i in 1..height - 1 {
        for j in 1..width - 1 {
            let local_score = check_scenic(input, (j, i), Direction::Left, None)
                * check_scenic(input, (j, i), Direction::Right, Some(width))
                * check_scenic(input, (j, i), Direction::Up, None)
                * check_scenic(input, (j, i), Direction::Down, Some(height));

            if best_scenic_score < local_score {
                best_scenic_score = local_score
            }
        }
    }

    best_scenic_score
}
