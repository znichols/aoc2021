use ndarray::prelude::*;
use std::cmp;
use std::env;
use std::error::Error;
use std::fs;

#[derive(Clone, Copy)]
enum Direction {
    In,
    Out,
}

fn read_grid(input: &str) -> Result<Array2<i32>, Box<dyn Error>> {
    let nums: Vec<Vec<i32>> = input
        .lines()
        .map(|l| l.chars().flat_map(|c| c.to_string().parse()).collect())
        .collect();
    Ok(Array2::from_shape_vec(
        (nums.len(), nums[0].len()),
        nums.iter().flatten().copied().collect(),
    )?)
}

fn compute_path_cost(
    grid: &Array2<i32>,
    loc: (usize, usize),
    prev_cost_map: &Array2<i32>,
    cost_map: &mut Array2<i32>,
    dir: Direction,
) -> i32 {
    if cost_map[loc] >= 0 {
        cost_map[loc]
    } else {
        let mut neighbor_costs: Vec<i32> = Vec::new();
        let nrows = grid.shape()[0];
        let ncols = grid.shape()[1];
        match dir {
            Direction::In => {
                if loc.0 > 0 {
                    neighbor_costs.push(compute_path_cost(
                        grid,
                        (loc.0 - 1, loc.1),
                        prev_cost_map,
                        cost_map,
                        dir,
                    ));
                }
                if loc.1 > 0 {
                    neighbor_costs.push(compute_path_cost(
                        grid,
                        (loc.0, loc.1 - 1),
                        prev_cost_map,
                        cost_map,
                        dir,
                    ));
                }
            }
            _ => {
                if loc.0 < nrows - 1 {
                    neighbor_costs.push(compute_path_cost(
                        grid,
                        (loc.0 + 1, loc.1),
                        prev_cost_map,
                        cost_map,
                        dir,
                    ));
                }
                if loc.1 < ncols - 1 {
                    neighbor_costs.push(compute_path_cost(
                        grid,
                        (loc.0, loc.1 + 1),
                        prev_cost_map,
                        cost_map,
                        dir,
                    ));
                }
            }
        }
        let r = cmp::min(
            neighbor_costs.iter().min().unwrap_or(&(i32::MAX / 2)) + grid[loc],
            prev_cost_map[loc],
        );
        cost_map[loc] = r;
        r
    }
}

fn find_cheapest_path(grid: &Array2<i32>) -> i32 {
    let mut cost_map_in = Array::zeros(grid.raw_dim()) - 1;
    let mut cost_map_out = Array::zeros(grid.raw_dim()) + i32::MAX / 2;
    cost_map_in[(0, 0)] = 0;
    let endpos = (grid.shape()[0] - 1, grid.shape()[1] - 1);
    let mut cost = compute_path_cost(grid, endpos, &cost_map_out, &mut cost_map_in, Direction::In);
    loop {
        cost_map_out.fill(-1);
        cost_map_out[endpos] = cost;
        let _ = compute_path_cost(
            grid,
            (0, 0),
            &cost_map_in,
            &mut cost_map_out,
            Direction::Out,
        );
        cost_map_in.fill(-1);
        cost_map_in[(0, 0)] = 0;
        let new_cost =
            compute_path_cost(grid, endpos, &cost_map_out, &mut cost_map_in, Direction::In);
        if new_cost < cost {
            cost = new_cost;
        } else {
            break;
        }
    }
    cost
}

fn expand_grid(grid: &Array2<i32>) -> Array2<i32> {
    let nrows = grid.shape()[0];
    let ncols = grid.shape()[1];
    let mut r: Array2<i32> = Array::zeros((nrows * 5, ncols * 5));
    for i in 0..5 {
        for j in 0..5 {
            let mut s = r.slice_mut(s![i * nrows..(i + 1) * nrows, j * ncols..(j + 1) * ncols]);
            s.assign(grid);
            s += (i + j) as i32;
        }
    }
    r.iter_mut().for_each(|v| {
        if *v > 9 {
            *v = *v % 10 + 1
        }
    });
    r
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])?;
    let grid = read_grid(&input)?;

    let cost = find_cheapest_path(&grid);
    println!("{:?}", cost);

    let expanded_grid = expand_grid(&grid);
    let expanded_cost = find_cheapest_path(&expanded_grid);
    println!("{:?}", expanded_cost);

    Ok(())
}
