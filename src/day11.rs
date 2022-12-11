use std::ops::AddAssign;
use std::{collections::VecDeque, fmt::Debug};

use num::traits::{One, Zero};

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
        return Ok(write!(
            f,
            "items: {:?} \n test_div: {:?} \n mnkyidx: {:?} \n  inspected_count{:?} \n",
            self.items, self.test_div, self.monkey_idx, self.inspected_counter
        )?);
    }
}

fn unoffical_generate_input_because_i_cant_bloody_use_mut_ref_as_an_input_to_the_solver_functions(
    input: &str,
) -> Vec<Monkey> {
    return input
        .split("\n\n")
        .map(|monkey| {
            let sections = monkey.split_terminator("\n").collect_vec();
            let op = sections[2].split(" = ").collect_vec()[1]
                .split_ascii_whitespace()
                .collect_vec();

            let op_closure: Box<dyn Fn(&u64) -> u64> = match op[1] {
                "+" => {
                    if op[2] == "old" {
                        Box::new(|val: &u64| val + val)
                    } else {
                        let num = op[2].parse::<u64>().unwrap();
                        Box::new(move |val: &u64| val + num.clone())
                    }
                }
                "*" => {
                    if op[2] == "old" {
                        Box::new(|val: &u64| val * val)
                    } else {
                        let num = op[2].parse::<u64>().unwrap();
                        Box::new(move |val: &u64| u64::from(val * &num))
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
                .map(|val| return val.parse::<u64>().unwrap())
                .collect();

            return Monkey::new(items, div, op_closure, monkey_idx);
        })
        .collect_vec();
}

// fn run_rounds(monkey_list: &mut Vec<Monkey>, rounds: i32) {
//     let common_multiple: u64 = monkey_list.iter().map(|monkey| monkey.test_div).product();

//     for i in 0..rounds {
//         for j in 0..monkey_list.len() {
//             for _ in 0..monkey_list[j].items.len() {
//                 let new_worry: u64 = (monkey_list[j].operation)(&monkey_list[j].items[0]);

//                 monkey_list[j].inspected_counter += 1;

//                 let new_monkey_index: usize;

//                 let new_worry = new_worry % common_multiple;

//                 if &new_worry % monkey_list[j].test_div as u64 == 0 {
//                     new_monkey_index = monkey_list[j].monkey_idx[0] as usize;
//                 } else {
//                     new_monkey_index = monkey_list[j].monkey_idx[1] as usize;
//                 }

//                 monkey_list[new_monkey_index].items.push_back(new_worry);

//                 monkey_list[j].items.pop_front();
//             }
//         }
//     }
// }

// #[aoc(day11, part1)]
// pub fn solve_part1(input: &str) -> i32 {
//     let rounds = 20;
//     let mut monkey_list = unoffical_generate_input_because_i_cant_bloody_use_mut_ref_as_an_input_to_the_solver_functions(input);

//     for _ in 0..rounds {
//         for j in 0..monkey_list.len() {
//             for _ in 0..monkey_list[j].items.len() {
//                 let new_worry: u32 = ((monkey_list[j].operation)(&monkey_list[j].items[0]) / 3)
//                     .try_into()
//                     .unwrap();

//                 monkey_list[j].inspected_counter += 1;

//                 let new_monkey_index: usize;

//                 if new_worry % monkey_list[j].test_div == 0 {
//                     new_monkey_index = monkey_list[j].monkey_idx[0] as usize;
//                 } else {
//                     new_monkey_index = monkey_list[j].monkey_idx[1] as usize;
//                 }

//                 monkey_list[new_monkey_index]
//                     .items
//                     .push_back(new_worry.try_into().unwrap());

//                 monkey_list[j].items.pop_front();
//             }
//         }
//     }

//     println!("{:#?}", monkey_list);
//     1
// }

#[aoc(day11, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let rounds = 10000;
    let mut monkey_list = unoffical_generate_input_because_i_cant_bloody_use_mut_ref_as_an_input_to_the_solver_functions(input);

    let common_multiple: u64 = monkey_list.iter().map(|monkey| monkey.test_div).product();

    for i in 0..rounds {
        // println!("round: {:?}", i);
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

    println!("{:#?}", monkey_list);
    1
}