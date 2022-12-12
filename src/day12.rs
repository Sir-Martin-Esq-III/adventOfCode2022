use itertools::Itertools;

use pathfinding::directed::{astar::astar, bfs::bfs, dijkstra::dijkstra};

enum PathfindingAlg {
    BFS,
    DJI,
    AST,
}

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

    if within_bounds(y, &0, &(grid.len() as i32 - 1))
        && grid[(y - 1) as usize][*x as usize] <= current_point_value + 1
    {
        points.push((*x, y - 1))
    }

    if within_bounds(y, &-1, &(grid.len() as i32 - 2))
        && grid[(y + 1) as usize][*x as usize] <= current_point_value + 1
    {
        points.push((*x, y + 1))
    }

    if within_bounds(x, &0, &(grid[0].len() as i32 - 1))
        && grid[*y as usize][(x - 1) as usize] <= current_point_value + 1
    {
        points.push((x - 1, *y))
    }
    if within_bounds(x, &-1, &(grid[0].len() as i32 - 2))
        && grid[*y as usize][(x + 1) as usize] <= current_point_value + 1
    {
        points.push((x + 1, *y))
    }

    points
}

fn run_path_finding_alg(
    algorithm: PathfindingAlg,
    start_pos: (i32, i32),
    end_pos: (i32, i32),
    grid: &Vec<Vec<u8>>,
) -> Option<i32> {
    match algorithm {
        PathfindingAlg::AST => {
            //     let res = astar(
            //         &start_pos,
            //         move |f| successor(f, grid),
            //         |F| F,
            //         |f| *f == end_pos,
            //     );

            //     match res {
            //         Some(r) => return Some((r.0.len() - 1).try_into().unwrap()),
            //         None => return None,
            //     }
            todo!()
        }
        PathfindingAlg::DJI => {
            let res = dijkstra(
                &start_pos,
                move |f| successor(f, grid).into_iter().map(|p| (p, 1)),
                |f| *f == end_pos,
            );

            match res {
                Some(r) => return Some((r.1).try_into().unwrap()),
                None => return None,
            }
        }
        PathfindingAlg::BFS => {
            let res = bfs(&start_pos, move |f| successor(f, grid), |f| *f == end_pos);

            match res {
                Some(r) => return Some((r.len() - 1).try_into().unwrap()),
                None => return None,
            }
        }
    }
}

#[aoc(day12, part1, dijkstra)]
pub fn solve_part1_dijkstra(input: &(Vec<Vec<u8>>, (i32, i32), (i32, i32))) -> i32 {
    run_path_finding_alg(PathfindingAlg::DJI, input.1, input.2, &input.0).unwrap()
}

#[aoc(day12, part2, dijkstra)]
pub fn solve_part2_dijkstra(input: &(Vec<Vec<u8>>, (i32, i32), (i32, i32))) -> i32 {
    let mut ans: Vec<i32> = vec![];
    for (y, row) in input.0.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            if *val as i32 == 97 {
                let local_res = run_path_finding_alg(
                    PathfindingAlg::DJI,
                    (x as i32, y as i32),
                    input.2,
                    &input.0,
                );

                if let Some(res) = local_res {
                    ans.push(res);
                }
            }
        }
    }
    ans.sort();
    *ans.first().unwrap()
}

#[aoc(day12, part1, bfs)]
pub fn solve_part1_bfs(input: &(Vec<Vec<u8>>, (i32, i32), (i32, i32))) -> i32 {
    run_path_finding_alg(PathfindingAlg::BFS, input.1, input.2, &input.0).unwrap()
}

#[aoc(day12, part2, bfs)]
pub fn solve_part2_bfs(input: &(Vec<Vec<u8>>, (i32, i32), (i32, i32))) -> i32 {
    let mut ans: Vec<i32> = vec![];
    for (y, row) in input.0.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            if *val as i32 == 97 {
                let local_res = run_path_finding_alg(
                    PathfindingAlg::BFS,
                    (x as i32, y as i32),
                    input.2,
                    &input.0,
                );

                if let Some(res) = local_res {
                    ans.push(res);
                }
            }
        }
    }
    ans.sort();
    *ans.first().unwrap()
}

// #[aoc(day12, part1, ast)]
// pub fn solve_part1_ast(input: &(Vec<Vec<u8>>, (i32, i32), (i32, i32))) -> i32 {
//     run_path_finding_alg(PathfindingAlg::AST, input.1, input.2, &input.0).unwrap()
// }

// #[aoc(day12, part2, ast)]
// pub fn solve_part2_ast(input: &(Vec<Vec<u8>>, (i32, i32), (i32, i32))) -> i32 {
//     let mut ans: Vec<i32> = vec![];
//     for (y, row) in input.0.iter().enumerate() {
//         for (x, val) in row.iter().enumerate() {
//             if *val as i32 == 97 {
//                 let local_res = run_path_finding_alg(
//                     PathfindingAlg::AST,
//                     (x as i32, y as i32),
//                     input.2,
//                     &input.0,
//                 );

//                 if let Some(res) = local_res {
//                     ans.push(res);
//                 }
//             }
//         }
//     }
//     ans.sort();
//     *ans.first().unwrap()
// }
