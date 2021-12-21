use ndarray::prelude::*;
use std::collections::{HashMap, HashSet};
use std::env;
use std::error::Error;
use std::fs;

struct Rotations;

impl Rotations {
    fn x() -> Array2<i32> {
        arr2(&[[1, 0, 0], [0, 0, -1], [0, 1, 0]])
    }
    fn y() -> Array2<i32> {
        arr2(&[[0, 0, 1], [0, 1, 0], [-1, 0, 0]])
    }
    fn z() -> Array2<i32> {
        arr2(&[[0, -1, 0], [1, 0, 0], [0, 0, 1]])
    }

    fn permute(mat: &Array2<i32>) -> Vec<Array2<i32>> {
        let mut r: Vec<Array2<i32>> = Vec::new();
        let mut new_mat = mat.clone();
        for _ in 0..4 {
            for _ in 0..4 {
                new_mat = new_mat.dot(&Rotations::x());
                r.push(new_mat.clone());
            }
            new_mat = new_mat.dot(&Rotations::z());
        }
        new_mat = new_mat.dot(&Rotations::y());
        for _ in 0..4 {
            new_mat = new_mat.dot(&Rotations::x());
            r.push(new_mat.clone());
        }
        new_mat = new_mat.dot(&Rotations::y());
        new_mat = new_mat.dot(&Rotations::y());
        for _ in 0..4 {
            new_mat = new_mat.dot(&Rotations::x());
            r.push(new_mat.clone());
        }
        r
    }
}

fn rows_to_mat(rows: Vec<Array1<i32>>) -> Array2<i32> {
    let mut r = Array2::zeros((rows.len(), rows[0].len()));
    for (i, v) in rows.iter().enumerate() {
        r.slice_mut(s![i, ..]).assign(v);
    }
    r
}

fn count_overlap(
    ref_mat: &Array2<i32>,
    cand_mat: &Array2<i32>,
) -> (usize, HashSet<Array1<i32>>, Array1<i32>) {
    let mut translated_cand_vectors: HashSet<Array1<i32>> = HashSet::new();
    let mut max_overlap = 0;
    let mut this_dist: Array1<i32> = arr1(&[0, 0, 0]);
    let permutations = Rotations::permute(cand_mat);
    for p in &permutations {
        let mut dist_counts: HashMap<Array1<i32>, usize> = HashMap::new();

        for cand_row_ind in 0..p.nrows() {
            let v = p.slice(s![cand_row_ind, ..]).to_owned();
            for ref_row_ind in 0..ref_mat.nrows() {
                let ref_v = ref_mat.slice(s![ref_row_ind, ..]).to_owned();
                let d = ref_v - &v;
                *dist_counts.entry(d).or_insert(0) += 1;
            }
        }
        let this_max = dist_counts.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
        if *this_max.1 > max_overlap {
            max_overlap = *this_max.1;
            this_dist = this_max.0.clone();

            translated_cand_vectors = (0..p.nrows())
                .map(|i| p.slice(s![i, ..]).to_owned() + &this_dist)
                .collect();
        }
    }
    (max_overlap, translated_cand_vectors, this_dist)
}

fn merge_scans(ref_mat: &Array2<i32>, candidate_rows: HashSet<Array1<i32>>) -> Array2<i32> {
    let new_rows: Vec<_> = (0..ref_mat.nrows())
        .map(|i| ref_mat.slice(s![i, ..]).to_owned())
        .collect::<HashSet<_>>()
        .union(&candidate_rows)
        .map(|v| v.to_owned())
        .collect();

    rows_to_mat(new_rows)
}

fn max_dist(scan_coords: &[Array1<i32>]) -> i32 {
    let mut max = 0;
    for c1 in scan_coords {
        for c2 in scan_coords {
            let d = (c1 - c2).mapv(|e| e.abs()).sum();
            if d > max {
                max = d;
            }
        }
    }
    max
}

fn merge_all(scan_coord_vec: &[Array2<i32>]) -> (Array2<i32>, i32) {
    let mut visited: HashSet<usize> = HashSet::new();
    let mut ref_mat = scan_coord_vec[0].clone();
    let mut scan_coords = vec![arr1(&[0, 0, 0])];
    visited.insert(0);
    while visited.len() < scan_coord_vec.len() {
        let best = (1..scan_coord_vec.len())
            .filter(|i| !visited.contains(i))
            .map(|i| (i, count_overlap(&ref_mat, &scan_coord_vec[i])))
            .max_by(|a, b| a.1 .0.cmp(&b.1 .0))
            .unwrap();
        ref_mat = merge_scans(&ref_mat, best.1 .1);
        scan_coords.push(best.1 .2);
        visited.insert(best.0);
    }

    (ref_mat, max_dist(&scan_coords))
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])?;

    let mut scan_coord_vec: Vec<Array2<i32>> = Vec::new();
    for scanner_input in input.split("\n\n") {
        let mut lines = scanner_input.lines();
        lines.next();
        let nums: Vec<Vec<i32>> = lines
            .map(|l| {
                l.split(',')
                    .flat_map(|n| n.parse::<i32>())
                    .collect::<Vec<_>>()
            })
            .collect();
        let mat = Array2::from_shape_vec(
            (nums.len(), nums[0].len()),
            nums.iter().flatten().copied().collect(),
        )?;
        scan_coord_vec.push(mat);
    }

    let merged = merge_all(&scan_coord_vec);
    println!("{}, {}", merged.0.nrows(), merged.1);

    Ok(())
}
