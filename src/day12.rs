use itertools::Itertools;

use pathfinding::directed::dijkstra::dijkstra;

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> (Vec<Vec<u8>>, (i32, i32), (i32, i32)) {
    let mut end_point: (i32, i32) = (0, 0);
    let mut start_point: (i32, i32) = (0, 0);
    let grid = input
        .lines()
        .enumerate()
        .map(|(idx, ln)| {
            ln.chars()
                .enumerate()
                .map(|(char_idx, char)| {
                    if char == 'E' {
                        end_point = (char_idx as i32, idx as i32);
                        return b'z';
                    }
                    if char == 'S' {
                        start_point = (char_idx as i32, idx as i32);
                        return b'a';
                    }
                    char as u8
                })
                .collect_vec()
        })
        .collect_vec();

    (grid, start_point, end_point)
}

fn within_bounds(val: &i32, lower: &i32, upper: &i32) -> bool {
    *val <= *upper && *val > *lower
}

fn successor((x, y): &(i32, i32), grid: &Vec<Vec<u8>>) -> Vec<(i32, i32)> {
    let mut points: Vec<(i32, i32)> = vec![];

    let current_point_value = grid[*y as usize][*x as usize];

    if within_bounds(y, &0, &(grid.len() as i32 - 1)) && grid[(y - 1) as usize][*x as usize] <= current_point_value + 1 {
        points.push((*x, y - 1))
    }

    if within_bounds(y, &-1, &(grid.len() as i32 - 2)) && grid[(y + 1) as usize][*x as usize] <= current_point_value + 1 {
        points.push((*x, y + 1))
    }

    if within_bounds(x, &0, &(grid[0].len() as i32 - 1)) && grid[*y as usize][(x - 1) as usize] <= current_point_value + 1 {
        points.push((x - 1, *y))
    }
    if within_bounds(x, &-1, &(grid[0].len() as i32 - 2)) && grid[*y as usize][(x + 1) as usize] <= current_point_value + 1 {
        points.push((x + 1, *y))
    }

    points
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &(Vec<Vec<u8>>, (i32, i32), (i32, i32))) -> usize {
    let res = dijkstra(
        &input.1,
        move |f| {
            successor(f, &input.0).into_iter().map(|p| (p, 1))
        },
        |f| *f == input.2,
    );

    res.unwrap().0.len() - 1
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &(Vec<Vec<u8>>, (i32, i32), (i32, i32))) -> i32 {
    let mut ans: Vec<i32> = vec![];
    for (y, row) in input.0.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            if *val as i32 == 97 {
                let local_res = dijkstra(
                    &(x as i32, y as i32),
                    move |f| {
                        successor(f, &input.0).into_iter().map(|p| (p, 1))
                    },
                    |f| *f == input.2,
                );

                if let Some(res) = local_res {
                    ans.push(res.1);
                }
            }
        }
    }

    ans.sort();

    *ans.first().unwrap()
}
