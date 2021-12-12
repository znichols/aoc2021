use ndarray::prelude::*;
use std::collections::HashSet;
use std::env;
use std::fs;

fn find_low_points(topo_map: &Array2<i32>) -> Vec<(usize, usize, i32)> {
    let mut r = Vec::new();
    let nrows = topo_map.shape()[0];
    let ncols = topo_map.shape()[1];

    let is_lowest = |i, j| {
        let mut is_lowest = true;
        let v = topo_map[[i, j]];
        for ii in i - 1..i + 2 {
            if ii != i && v >= topo_map[[ii, j]] {
                is_lowest = false;
            }
        }
        for jj in j - 1..j + 2 {
            if jj != j && v >= topo_map[[i, jj]] {
                is_lowest = false;
            }
        }
        is_lowest
    };

    for i in 1..nrows - 1 {
        for j in 1..ncols - 1 {
            if is_lowest(i, j) {
                r.push((i, j, topo_map[[i, j]]));
            }
        }
    }
    r
}

fn extend_basin(topo_map: &Array2<i32>, pos: (usize, usize), basin: &mut HashSet<(usize, usize)>) {
    if !basin.contains(&pos) && topo_map[pos] != 9 {
        basin.insert(pos);
        extend_basin(topo_map, (pos.0 - 1, pos.1), basin);
        extend_basin(topo_map, (pos.0 + 1, pos.1), basin);
        extend_basin(topo_map, (pos.0, pos.1 - 1), basin);
        extend_basin(topo_map, (pos.0, pos.1 + 1), basin);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filecontents = fs::read_to_string(&args[1]).unwrap();
    let nums: Vec<Vec<i32>> = filecontents
        .lines()
        .map(|l| l.chars().flat_map(|c| c.to_string().parse()).collect())
        .collect();
    let ncols = nums[0].len();
    let nrows = nums.len();
    let mut tmp_data = Vec::new();
    for i in 0..nrows {
        tmp_data.extend_from_slice(&nums[i]);
    }
    let x = Array2::from_shape_vec((nrows, ncols), tmp_data).unwrap();
    let mut topo_map = Array2::from_elem((nrows + 2, ncols + 2), 9);
    let mut slice = topo_map.slice_mut(s![1..nrows + 1, 1..ncols + 1]);
    slice.assign(&x);

    let low_points = find_low_points(&topo_map);
    let mut basins: Vec<HashSet<(usize, usize)>> = Vec::new();
    for low_point in &low_points {
        let mut basin: HashSet<(usize, usize)> = HashSet::new();
        extend_basin(&topo_map, (low_point.0, low_point.1), &mut basin);
        basins.push(basin);
    }
    let mut basin_sizes = basins.iter().map(|b| b.len() as i32).collect::<Vec<i32>>();
    basin_sizes.sort_by(|a, b| b.cmp(a));

    println!(
        "{:?}, {:?}",
        low_points.iter().map(|x| 1 + x.2).sum::<i32>(),
        basin_sizes[..3].iter().product::<i32>()
    );
}
