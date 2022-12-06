use itertools::Itertools;

fn find_unique_seq_of_len(input: &str, len: i32) -> i32 {
    let mut counter = 0;
    for set in input.chars().collect_vec().windows(len as usize) {
        if set.iter().all_unique() {
            break;
        }
        counter += 1
    }

    return len + counter;
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> i32 {
    find_unique_seq_of_len(input, 4)
}
#[aoc(day6, part2)]
pub fn solve_part2(input: &str) -> i32 {
    find_unique_seq_of_len(input, 14)
}
