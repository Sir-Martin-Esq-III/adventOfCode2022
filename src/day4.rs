use itertools::Itertools;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Vec<(i32, i32)>> {
    input
        .lines()
        .map(|ln| {
            ln.split(',')
                .map(|sp| {
                    sp.split('-')
                        .map(|g| g.parse::<i32>().unwrap())
                        .collect_tuple::<(i32, i32)>()
                        .unwrap()
                })
                .collect::<Vec<(i32, i32)>>()
        })
        .collect::<Vec<Vec<(i32, i32)>>>()
}

fn is_fully_overlapping(s1: (i32, i32), s2: (i32, i32)) -> bool {
    let rng = s1.0..=s1.1;
    let rng2 = s2.0..=s2.1;

    if rng.contains(&s2.0) && rng.contains(&s2.1) {
        return true;
    }
    if rng2.contains(&s1.0) && rng2.contains(&s1.1) {
        return true;
    }
    false
}

fn is_partially_overlapping(s1: (i32, i32), s2: (i32, i32)) -> bool {
    let rng = s1.0..=s1.1;
    let rng2 = s2.0..=s2.1;

    if rng.contains(&s2.0) || rng.contains(&s2.1) {
        return true;
    }
    if rng2.contains(&s1.0) || rng2.contains(&s1.1) {
        return true;
    }
    false
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[Vec<(i32, i32)>]) -> i32 {
    let mut overlapping_set_count = 0;
    for pair in input.iter() {
        if is_fully_overlapping(pair[0], pair[1]) {
            overlapping_set_count += 1
        }
    }

    overlapping_set_count
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[Vec<(i32, i32)>]) -> i32 {
    let mut overlapping_set_count = 0;
    for pair in input.iter() {
        if is_partially_overlapping(pair[0], pair[1]) {
            overlapping_set_count += 1
        }
    }

    overlapping_set_count
}
