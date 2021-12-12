use ndarray::prelude::*;
use std::cmp;
use std::env;
use std::fs;

fn read_grid(input: &str) -> Array2<i32> {
    let nums: Vec<Vec<i32>> = input
        .lines()
        .map(|l| l.chars().flat_map(|c| c.to_string().parse()).collect())
        .collect();
    Array2::from_shape_vec(
        (nums.len(), nums[0].len()),
        nums.iter().flatten().copied().collect(),
    )
    .unwrap()
}

fn get_flash_inds(octo_grid: &Array2<i32>) -> Option<Vec<(usize, usize)>> {
    let flash_map: Vec<(usize, usize)> = octo_grid
        .indexed_iter()
        .filter(|(_, v)| **v > 9)
        .map(|(idx, _)| idx)
        .collect();

    if flash_map.is_empty() {
        None
    } else {
        Some(flash_map)
    }
}

fn perform_flash(octo_grid: &mut Array2<i32>, loc: (usize, usize)) {
    let nrows = octo_grid.shape()[0];
    let ncols = octo_grid.shape()[1];
    fn dec_if_pos(x: usize) -> usize {
        if x == 0 {
            0
        } else {
            x - 1
        }
    }

    let mut flash_region = octo_grid.slice_mut(s![
        dec_if_pos(loc.0)..cmp::min(nrows, loc.0 + 2),
        dec_if_pos(loc.1)..cmp::min(ncols, loc.1 + 2),
    ]);
    flash_region += 1;
}

fn flash_until_done(octo_grid: &mut Array2<i32>) -> usize {
    let mut all_flashed_inds = Vec::<(usize, usize)>::new();

    while let Some(flash_inds) = get_flash_inds(octo_grid) {
        for idx in &flash_inds {
            perform_flash(octo_grid, *idx);
        }
        all_flashed_inds.extend_from_slice(&flash_inds);
        for idx in &all_flashed_inds {
            octo_grid[*idx] = 0;
        }
    }

    all_flashed_inds.len()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut octo_grid = read_grid(&fs::read_to_string(&args[1]).unwrap());

    let mut flash_count = 0;
    for _ in 0..100 {
        octo_grid += 1;
        flash_count += flash_until_done(&mut octo_grid);
    }

    let mut first_sync = 100;
    while flash_until_done(&mut octo_grid) < octo_grid.len() {
        octo_grid += 1;
        first_sync += 1;
    }

    println!("{}, {}", flash_count, first_sync);
}
