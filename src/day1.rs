//Bit grim
use rayon::prelude::*;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Vec<i32>> {
    return input
        .split("\n\n")
        .map(|sp| {
            return sp
                .split('\n')
                .map(|input| return input.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
        })
        .collect::<Vec<Vec<i32>>>();
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Vec<Vec<i32>>) -> i32 {
    input
        .iter()
        .map(|val| return val.iter().sum())
        .max()
        .unwrap()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &Vec<Vec<i32>>) -> i32 {
    let mut sums = input
        .iter()
        .map(|val| val.iter().sum::<i32>())
        .collect::<Vec<i32>>();

    sums.sort();
    sums.iter().rev().take(3).sum()
}
