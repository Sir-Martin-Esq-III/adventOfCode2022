use std::collections::{BTreeMap};

use itertools::Itertools;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|ln| ln.to_string())
        .collect::<Vec<String>>()
}

enum Commands {
    Cd(String),
    Ls,
}

impl TryFrom<&String> for Commands {
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        if value.ends_with("ls") {
            Ok(Commands::Ls)
        } else {
            let dir_name = value.split_ascii_whitespace().last().unwrap();
            Ok(Commands::Cd(dir_name.to_string()))
        }
    }

    type Error = ();
}

fn is_command(line: &str) -> bool {
    line.starts_with("$ ")
}

const MAX_SIZE: i32 = 100000;
const TOTAL_DISK_SIZE: i32 = 70000000;
const MIN_DIFF: i32 = 30000000;

fn get_dir_sizes(cmds: &[String]) -> BTreeMap<String, i32> {
    let mut directories: BTreeMap<String, i32> = BTreeMap::new();

    let mut current_dir_path: Vec<String> = vec![];

    for cmd in cmds.iter() {
        if is_command(cmd) {
            let command: Commands = cmd.try_into().unwrap();
            match command {
                Commands::Cd(dir_name) => {
                    if dir_name == *".." {
                        current_dir_path.pop();
                    } else {
                        current_dir_path.push(dir_name);
                        directories.entry(current_dir_path.join(" / ")).or_insert(0);
                    }
                }
                _ => {
                    continue;
                }
            }
        } else {
            let sp = cmd.split_ascii_whitespace().collect_vec();
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

    directories
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[String]) -> i32 {
    get_dir_sizes(input)
        .values()
        .filter(|val| **val <= MAX_SIZE)
        .sum()
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &[String]) -> i32 {
    let dirs = get_dir_sizes(input);

    println!("{:#?}", dirs);

    let diff = TOTAL_DISK_SIZE - dirs.get("/").unwrap();

    println!("diff {:?}", diff);

    let d2 = MIN_DIFF - diff;

    let mut smallest_to_delete = 0;

    let mut smallest_local_diff = MIN_DIFF;
    for (k, v) in dirs.iter() {
        let local_diff = v - d2;
        if local_diff >= 0 && local_diff < smallest_local_diff {
            println!("{:?} {:?} {:?}", k, v, d2);
            smallest_to_delete = *v;
            smallest_local_diff = local_diff;
        }
    }
    smallest_to_delete
}
