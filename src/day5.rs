use std::collections::VecDeque;

use itertools::Itertools;
use regex::Regex;

#[derive(Clone)]
pub struct SupplyStack {
    crane: Vec<VecDeque<char>>,
    instr: Vec<String>,
}
#[derive(PartialEq, Eq)]
pub enum CraneType {
    CrateMover9000,
    CrateMover9001,
}

impl SupplyStack {
    pub fn run_instr(&mut self, crane_type: CraneType) -> &mut Self {
        for instruction in self.instr.iter() {
            let g = instruction
                .replace("move ", "")
                .replace(" from ", ",")
                .replace(" to ", ",");

            let values = g
                .split(',')
                .map(|val| val.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            let mut new = self.crane[values[1] as usize - 1]
                .drain(..values[0] as usize)
                .collect_vec();

            if crane_type == CraneType::CrateMover9001 {
                new.reverse();
            }

            for i in 0..values[0] {
                self.crane[values[2] as usize - 1].push_front(new[i as usize]);
            }
        }

        self
    }

    pub fn get_vals_of_top_crates(&self) -> String {
        let mut word_vec: Vec<char> = vec![];
        for row in self.crane.iter() {
            word_vec.push(*row.get(0).unwrap());
        }
        word_vec.iter().join("")
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> SupplyStack {
    let inputs = input.split("\n\n").collect_vec();
    let instr = inputs[1].lines().map(|ln| ln.to_string()).collect_vec();

    let mut parsed_board: Vec<VecDeque<char>> = vec![VecDeque::new(); 9];

    for row in inputs[0].lines() {
        for (idx_tracker, set) in row.chars().collect_vec().chunks(4).enumerate() {
            let r = set.iter().unique().join("");
            if r != " " {
                parsed_board[idx_tracker].push_back(r.chars().nth(1).unwrap());
            }
        }
    }

    parsed_board.iter_mut().for_each(|vec| {
        vec.pop_back();
    });

    SupplyStack {
        crane: parsed_board,
        instr,
    }
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &SupplyStack) -> String {
    let mut t = input.to_owned();

    t.run_instr(CraneType::CrateMover9000)
        .get_vals_of_top_crates()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &SupplyStack) -> String {
    let mut t = input.to_owned();

    t.run_instr(CraneType::CrateMover9001)
        .get_vals_of_top_crates()
}
