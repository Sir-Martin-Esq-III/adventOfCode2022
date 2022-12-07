use std::collections::{BTreeMap, HashMap};

use itertools::Itertools;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|ln| ln.to_string())
        .collect::<Vec<String>>()
}

fn is_command(line: &str) -> bool {
    line.starts_with("$ ")
}

const MAX_SIZE: i32 = 100000;

const TOTAL_DISK_SIZE: i32 = 70000000;

const MIN_DIFF: i32 = 30000000;

#[aoc(day7, part1)]
pub fn solve_part1(input: &[String]) -> i32 {
    let mut directories: BTreeMap<String, i32> = BTreeMap::new();

    let mut current_dir_path: Vec<String> = vec![];

    for cmd in input.iter() {
        if is_command(cmd) {
            let sp = cmd.split(' ').collect_vec();
            if sp.len() == 3 {
                if sp[2] == ".." {
                    current_dir_path.pop().unwrap();
                } else {
                    current_dir_path.push(sp[2].to_string());
                    directories.entry(current_dir_path.join(" / ")).or_insert(0);
                }
            }
        } else {
            let sp = cmd.split(' ').collect_vec();
            if sp[0] == "dir" {
                directories.entry(sp[1].to_string()).or_insert(0);
            } else {
                for i in 0..=current_dir_path.len() {
                    directories
                        .entry(current_dir_path[0..i].join(" / "))
                        .and_modify(|size_counter| *size_counter += sp[0].parse::<i32>().unwrap());
                }
            }
        }
    }

    directories.values().filter(|val| **val <= MAX_SIZE).sum()
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &[String]) -> i32 {
    let mut directories: BTreeMap<String, i32> = BTreeMap::new();

    let mut current_dir_path: Vec<String> = vec![];

    for cmd in input.iter() {
        if is_command(cmd) {
            let sp = cmd.split(' ').collect_vec();
            if sp.len() == 3 {
                if sp[2] == ".." {
                    current_dir_path.pop().unwrap();
                } else {
                    current_dir_path.push(sp[2].to_string());
                    directories.entry(current_dir_path.join(" / ")).or_insert(0);
                }
            }
        } else {
            let sp = cmd.split(' ').collect_vec();
            if sp[0] == "dir" {
                directories.entry(sp[1].to_string()).or_insert(0);
            } else {
                for i in 0..=current_dir_path.len() {
                    directories
                        .entry(current_dir_path[0..i].join(" / "))
                        .and_modify(|size_counter| *size_counter += sp[0].parse::<i32>().unwrap());
                }
            }
        }
    }
    // println!("{:#?}", directories);

    let diff = TOTAL_DISK_SIZE - directories.get("/").unwrap();

    let d2 = MIN_DIFF - diff;

    println!("{:?}", diff);
    println!("{:?}", d2);

    let mut minVal = 0;

    let mut smallest_local_diff = MIN_DIFF;
    for (k, v) in directories.iter() {
        let local_diff = v - d2;
        if local_diff >= 0 && local_diff < smallest_local_diff {
            minVal = *v;
            smallest_local_diff = local_diff;
        }
    }
    minVal
}
//4499457
