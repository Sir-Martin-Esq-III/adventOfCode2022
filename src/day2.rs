const WIN: i32 = 6;
const DRAW: i32 = 3;
const LOSE: i32 = 0;

#[derive(Debug)]
pub enum Move {
    Rock,
    Paper,
    Scissor,
}

impl Move {
    fn value(&self) -> i32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissor => 3,
        }
    }
    fn new(val: &str) -> Move {
        match val {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissor,
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissor,
            _ => unreachable!(),
        }
    }

    fn find_winning(&self) -> Move {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissor,
            Move::Scissor => Move::Rock,
        }
    }
    fn find_draw(&self) -> &Move {
        return self;
    }
    fn find_lose(&self) -> Move {
        match self {
            Move::Rock => Move::Scissor,
            Move::Paper => Move::Rock,
            Move::Scissor => Move::Paper,
        }
    }
}

fn determine_outcome(game: &(Move, Move)) -> i32 {
    match game.0 {
        Move::Paper => match game.1 {
            Move::Paper => DRAW,
            Move::Scissor => WIN,
            Move::Rock => LOSE,
        },
        Move::Rock => match game.1 {
            Move::Paper => WIN,
            Move::Scissor => LOSE,
            Move::Rock => DRAW,
        },
        Move::Scissor => match game.1 {
            Move::Paper => LOSE,
            Move::Scissor => DRAW,
            Move::Rock => WIN,
        },
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(Move, Move)> {
    input
        .lines()
        .map(|ln| {
            let t = ln.split(" ").collect::<Vec<&str>>();
            return (Move::new(t[0]), Move::new(t[1]));
        })
        .collect::<Vec<(Move, Move)>>()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Vec<(Move, Move)>) -> i32 {
    let mut score = 0;
    for game in input.iter() {
        score += game.1.value() + determine_outcome(game);
    }
    score
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Vec<(Move, Move)>) -> i32 {
    let mut score = 0;
    for game in input.iter() {
        match game.1 {
            Move::Rock => score += game.0.find_lose().value() + LOSE,
            Move::Paper => score += game.0.find_draw().value() + DRAW,
            Move::Scissor => score += game.0.find_winning().value() + WIN,
        }
    }
    score
}
