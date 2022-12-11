

use std::{collections::VecDeque, fmt::Debug};

use num::traits::{Zero};

use itertools::Itertools;

pub struct Monkey {
    items: VecDeque<u64>,
    test_div: u64,
    operation: Box<dyn Fn(&u64) -> u64>,
    monkey_idx: [u32; 2],
    inspected_counter: u64,
}

impl Monkey {
    fn new(
        items: VecDeque<u64>,
        test_div: u64,
        operation: Box<dyn Fn(&u64) -> u64>,
        monkey_idx: [u32; 2],
    ) -> Monkey {
        Monkey {
            items,
            test_div,
            operation,
            monkey_idx,
            inspected_counter: Zero::zero(),
        }
    }
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(write!(
            f,
            "items: {:?} \n test_div: {:?} \n mnkyidx: {:?} \n  inspected_count{:?} \n",
            self.items, self.test_div, self.monkey_idx, self.inspected_counter
        )?)
    }
}

fn unoffical_generate_input_because_i_cant_bloody_use_mut_ref_as_an_input_to_the_solver_functions(
    input: &str,
) -> Vec<Monkey> {
    return input
        .split("\n\n")
        .map(|monkey| {
            let sections = monkey.split_terminator('\n').collect_vec();
            let op = sections[2].split(" = ").collect_vec()[1]
                .split_ascii_whitespace()
                .collect_vec();

            let op_closure: Box<dyn Fn(&u64) -> u64> = match op[1] {
                "+" => {
                    if op[2] == "old" {
                        Box::new(|val: &u64| val + val)
                    } else {
                        let num = op[2].parse::<u64>().unwrap();
                        Box::new(move |val: &u64| val + num)
                    }
                }
                "*" => {
                    if op[2] == "old" {
                        Box::new(|val: &u64| val * val)
                    } else {
                        let num = op[2].parse::<u64>().unwrap();
                        Box::new(move |val: &u64| (val * &num))
                    }
                }
                _ => panic!("operation not found"),
            };

            let div = sections[3].split(" by ").collect_vec()[1]
                .parse::<u64>()
                .unwrap();

            let monkey_idx = [
                sections[4].chars().last().unwrap().to_digit(10).unwrap(),
                sections[5].chars().last().unwrap().to_digit(10).unwrap(),
            ];

            let items = sections[1].split(": ").collect_vec()[1]
                .split(", ")
                .map(|val| val.parse::<u64>().unwrap())
                .collect();

            Monkey::new(items, div, op_closure, monkey_idx)
        })
        .collect_vec();
}

fn run_rounds(monkey_list: &mut Vec<Monkey>, rounds: i32) -> u64 {
    let common_multiple: u64 = monkey_list.iter().map(|monkey| monkey.test_div).product();

    for _ in 0..rounds {
        for j in 0..monkey_list.len() {
            for _ in 0..monkey_list[j].items.len() {
                let new_worry: u64 = (monkey_list[j].operation)(&monkey_list[j].items[0]);

                monkey_list[j].inspected_counter += 1;

                let new_monkey_index: usize;

                let new_worry = new_worry % common_multiple;

                if &new_worry % monkey_list[j].test_div as u64 == 0 {
                    new_monkey_index = monkey_list[j].monkey_idx[0] as usize;
                } else {
                    new_monkey_index = monkey_list[j].monkey_idx[1] as usize;
                }

                monkey_list[new_monkey_index].items.push_back(new_worry);

                monkey_list[j].items.pop_front();
            }
        }
    }

    let mut sorted_list = monkey_list
        .iter()
        .map(|monkey| monkey.inspected_counter)
        .collect_vec();

    sorted_list.sort();
    println!("{:?}", sorted_list);

    sorted_list[6] * sorted_list[7]
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &str) -> u64 {
    let mut monkey_list = unoffical_generate_input_because_i_cant_bloody_use_mut_ref_as_an_input_to_the_solver_functions(input);

    run_rounds(&mut monkey_list, 20)
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &str) -> u64 {
    let mut monkey_list = unoffical_generate_input_because_i_cant_bloody_use_mut_ref_as_an_input_to_the_solver_functions(input);

    run_rounds(&mut monkey_list, 10000)
}
