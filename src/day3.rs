use itertools::Itertools;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|ln| ln.to_string())
        .collect::<Vec<String>>()
}

fn get_value_for_character(character: char) -> i32 {
    if character.is_uppercase() {
        character as i32 - 38
    } else {
        character as i32 - 96
    }
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[String]) -> i32 {
    let mut sum: i32 = 0;
    for rs in input.iter() {
        let mid = rs.len() / 2;

        let set1 = rs[0..mid].chars().unique().collect::<Vec<char>>();

        for c in set1.iter() {
            if rs[mid..].chars().unique().any(|x| x == *c) {
                sum += get_value_for_character(*c);
            }
        }
    }

    sum
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[String]) -> i32 {
    let mut sum: i32 = 0;

    for rs_set in input.chunks(3) {
        let set1 = rs_set[0].chars().unique().collect::<Vec<char>>();

        for c in set1.iter() {
            if rs_set[1].chars().unique().any(|x| x == *c)
                && rs_set[2].chars().unique().any(|x| x == *c)
            {
                sum += get_value_for_character(*c);
            }
        }
    }

    sum
}
