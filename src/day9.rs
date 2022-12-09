use std::{cmp, collections::HashSet};

use itertools::Itertools;

pub enum Movement {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

impl Movement {
    fn new(letter: char, number: i32) -> Movement {
        match letter {
            'U' => Movement::Up(number),
            'D' => Movement::Down(number),
            'L' => Movement::Left(number),
            'R' => Movement::Right(number),
            _ => panic!(),
        }
    }
}

type Vec2d = (i32, i32);

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Movement> {
    return input
        .lines()
        .map(|val| {
            let vals = val.split_whitespace().collect_vec();
            return Movement::new(
                vals[0].chars().next().unwrap(),
                vals[1].parse::<i32>().unwrap(),
            );
        })
        .collect_vec();
}

fn update_knot_position(p1: &Vec2d, p2: &Vec2d) -> Vec2d {
    let mut new_val: Vec2d = p2.clone();

    //Straight
    if p1.0 == new_val.0 && p1.1 > new_val.1 {
        new_val.1 += 1;
    } else if p1.0 == new_val.0 && p1.1 < new_val.1 {
        new_val.1 -= 1;
    } else if p1.0 > new_val.0 && p1.1 == new_val.1 {
        new_val.0 += 1;
    } else if p1.0 < new_val.0 && p1.1 == new_val.1 {
        new_val.0 -= 1;
        //diag
    } else if p1.0 > new_val.0 && p1.1 > new_val.1 {
        new_val.1 += 1;
        new_val.0 += 1;
    } else if p1.0 < new_val.0 && p1.1 < new_val.1 {
        new_val.1 -= 1;
        new_val.0 -= 1;
    } else if p1.0 > new_val.0 && p1.1 < new_val.1 {
        new_val.1 -= 1;
        new_val.0 += 1;
    } else if p1.0 < new_val.0 && p1.1 > new_val.1 {
        new_val.1 += 1;
        new_val.0 -= 1;
    }

    return new_val;
}

fn get_distance_between_positions(p1: &Vec2d, p2: &Vec2d) -> i32 {
    let x_diff = (p1.0 - p2.0).abs();
    let y_diff = (p1.1 - p2.1).abs();

    cmp::max(x_diff, y_diff)
}

fn update_rope(length: usize, rope: &mut Vec<Vec2d>, visited: &mut HashSet<Vec2d>) {
    for i in 0..length {
        if get_distance_between_positions(&rope[i], &rope[i + 1]) > 1 {
            rope[i + 1] = update_knot_position(&rope[i], &rope[i + 1]);
        }

        if i == length - 1 {
            visited.insert(rope[i + 1]);
        }
    }
}

fn solve(length: usize, input: &Vec<Movement>) -> usize {
    let mut rope: Vec<Vec2d> = vec![(0, 0); length];
    let mut visited_positions: HashSet<Vec2d> = HashSet::new();

    visited_positions.insert((0, 0));

    for row in input {
        match row {
            Movement::Up(distance) => {
                for _ in 0..*distance {
                    rope[0].1 += 1;
                    update_rope(length - 1, &mut rope, &mut visited_positions);
                }
            }
            Movement::Down(distance) => {
                for _ in 0..*distance {
                    rope[0].1 -= 1;
                    update_rope(length - 1, &mut rope, &mut visited_positions);
                }
            }
            Movement::Left(distance) => {
                for _ in 0..*distance {
                    rope[0].0 -= 1;
                    update_rope(length - 1, &mut rope, &mut visited_positions);
                }
            }
            Movement::Right(distance) => {
                for _ in 0..*distance {
                    rope[0].0 += 1;
                    update_rope(length - 1, &mut rope, &mut visited_positions);
                }
            }
        }
    }

    visited_positions.len()
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &Vec<Movement>) -> usize {
    solve(2, input)
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &Vec<Movement>) -> usize {
    solve(10, input)
}
