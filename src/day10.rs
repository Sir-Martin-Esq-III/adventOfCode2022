use itertools::Itertools;

pub enum Instruction {
    Addx(usize, i32),
    Noop(usize),
}

impl Instruction {
    fn new(instr: &str) -> Instruction {
        let sp = instr.split_ascii_whitespace().collect_vec();
        match sp[0] {
            "noop" => Instruction::Noop(1),
            "addx" => Instruction::Addx(2, sp[1].parse::<i32>().unwrap()),
            _ => panic!("no instruction with this name exists"),
        }
    }
}

//20(2n-1)
fn calc_next_special_cycle(n: usize) -> i32 {
    20 * ((2 * n) - 1) as i32
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(Instruction::new)
        .collect::<Vec<Instruction>>()
}

fn draw_to_crt(reg: i32, cycle: i32, crt: &mut [char; 240]) {
    let adj_center = cycle - (cycle / 40) * 40;

    if (reg - 1..=reg + 1).contains(&adj_center) {
        crt[cycle as usize] = '█'
    }
}

fn print_crt(crt: &mut [char; 240]) {
    for i in 1..crt.len() + 1 {
        print!("{}", crt[i - 1]);
        if i % 40 == 0 {
            println!("\n");
        }
    }
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[Instruction]) -> i32 {
    let mut current_cycle = 0;
    let mut register = 1;
    let mut signal_strength: Vec<i32> = vec![];
    let mut next_special_cycle = calc_next_special_cycle(signal_strength.len() + 1);

    for instr in input {
        match instr {
            Instruction::Noop(cycle_amount) => {
                if current_cycle == next_special_cycle as usize {
                    signal_strength.push(register * next_special_cycle);
                    next_special_cycle = calc_next_special_cycle(signal_strength.len() + 1);
                }
                current_cycle += cycle_amount;
            }
            Instruction::Addx(cycle_amount, val) => {
                if current_cycle + *cycle_amount >= next_special_cycle as usize {
                    signal_strength.push(register * next_special_cycle);
                    next_special_cycle = calc_next_special_cycle(signal_strength.len() + 1);
                }
                current_cycle += *cycle_amount;
                register += *val;
            }
        }
    }

    return signal_strength.iter().sum();
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &[Instruction]) -> u8 {
    let mut crt = ['⠀'; 240];
    let mut current_cycle: usize = 0;
    let mut register = 1;

    for instr in input {
        match instr {
            Instruction::Noop(cycle_amount) => {
                draw_to_crt(register, current_cycle as i32, &mut crt);
                current_cycle += cycle_amount;
            }
            Instruction::Addx(cycle_amount, val) => {
                for _ in 0..*cycle_amount {
                    draw_to_crt(register, current_cycle as i32, &mut crt);
                    current_cycle += 1_usize;
                }
                register += *val;
            }
        }
    }

    print_crt(&mut crt);
    1
}
