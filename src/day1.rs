#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    return input
        .split("\n\n")
        .map(|sp| {
            return sp
                .split('\n')
                .map(|input| return input.parse::<i32>().unwrap())
                .sum::<i32>();
        })
        .collect();
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Vec<i32>) -> i32 {
    *input.iter().max().unwrap()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &Vec<i32>) -> i32 {
    let mut t = input.to_owned();
    t.sort_by(|a, b| b.cmp(a));
    t.iter().take(3).sum()
}
