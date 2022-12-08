use itertools::{rev, Itertools};

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|ln| ln.chars().map(|ch| ch.to_digit(10).unwrap()).collect_vec())
        .collect_vec()
}

fn check_left(board: &Vec<Vec<u32>>, pos: (usize, usize)) -> bool {
    let tree_height = board[pos.1][pos.0];

    for i in (0..pos.0).rev() {
        if board[pos.1][i] >= tree_height {
            return false;
        }
    }

    return true;
}

fn check_right(board: &Vec<Vec<u32>>, pos: (usize, usize), lim: usize) -> bool {
    let tree_height = board[pos.1][pos.0];
    for i in pos.0 + 1..lim {
        if board[pos.1][i] >= tree_height {
            return false;
        }
    }

    return true;
}

fn check_up(board: &Vec<Vec<u32>>, pos: (usize, usize)) -> bool {
    let tree_height = board[pos.1][pos.0];

    for i in (0..pos.1).rev() {
        if board[i][pos.0] >= tree_height {
            return false;
        }
    }

    return true;
}

fn check_down(board: &Vec<Vec<u32>>, pos: (usize, usize), lim: usize) -> bool {
    let tree_height = board[pos.1][pos.0];

    for i in pos.1 + 1..lim {
        if board[i][pos.0] >= tree_height {
            return false;
        }
    }

    return true;
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &Vec<Vec<u32>>) -> i32 {
    let mut sum: i32 = 0;

    let width = input[0].len();
    let height = input.len();

    for i in 1..height - 1 {
        for j in 1..width - 1 {
            let g = check_left(input, (j, i))
                || check_right(input, (j, i), width)
                || check_up(input, (j, i))
                || check_down(input, (j, i), height);

            if (g) {
                sum += 1
            }
        }
    }

    sum + 2 * width as i32 + 2 * height as i32 - 4
}

fn check_left_num(board: &Vec<Vec<u32>>, pos: (usize, usize)) -> i32 {
    let tree_height = board[pos.1][pos.0];
    let mut sum = 0;

    for i in (0..pos.0).rev() {
        if board[pos.1][i] >= tree_height {
            return sum + 1;
        }
        sum += 1;
    }

    return sum;
}

fn check_right_num(board: &Vec<Vec<u32>>, pos: (usize, usize), lim: usize) -> i32 {
    let tree_height = board[pos.1][pos.0];
    let mut sum = 0;
    for i in pos.0 + 1..lim {
        if board[pos.1][i] >= tree_height {
            return sum + 1;
        }
        sum += 1;
    }

    return sum;
}

fn check_up_num(board: &Vec<Vec<u32>>, pos: (usize, usize)) -> i32 {
    let tree_height = board[pos.1][pos.0];
    let mut sum = 0;
    for i in (0..pos.1).rev() {
        if board[i][pos.0] >= tree_height {
            return sum + 1;
        }
        sum += 1;
    }

    return sum;
}

fn check_down_num(board: &Vec<Vec<u32>>, pos: (usize, usize), lim: usize) -> i32 {
    let tree_height = board[pos.1][pos.0];
    let mut sum = 0;
    for i in pos.1 + 1..lim {
        if board[i][pos.0] >= tree_height {
            return sum + 1;
        }
        sum += 1;
    }

    return sum;
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &Vec<Vec<u32>>) -> i32 {
    let mut sum: i32 = 0;

    let width = input[0].len();
    let height = input.len();

    let mut best_scenic_score = 0;

    for i in 1..height - 1 {
        for j in 1..width - 1 {
            let e = check_up_num(input, (j, i));
            let q = check_left_num(input, (j, i));
            let r = check_down_num(input, (j, i), height);
            let w = check_right_num(input, (j, i), width);

            let local_score = q * w * e * r;
            if best_scenic_score < local_score {
                best_scenic_score = local_score
            }
        }
    }

    best_scenic_score
}
