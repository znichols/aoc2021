use std::cmp;
use std::collections::HashSet;
use std::env;
use std::fs;

fn coord_fold(coords: &[(i32, i32)], fold: &(&str, i32)) -> Vec<(i32, i32)> {
    if fold.0 == "x" {
        let xmax = coords.iter().map(|c| c.0).max().unwrap();
        let offset = cmp::max(fold.1, xmax - fold.1);
        coords
            .iter()
            .map(|c| (offset - (c.0 - fold.1).abs(), c.1))
            .collect()
    } else {
        let ymax = coords.iter().map(|c| c.1).max().unwrap();
        let offset = cmp::max(fold.1, ymax - fold.1);
        coords
            .iter()
            .map(|c| (c.0, offset - (c.1 - fold.1).abs()))
            .collect()
    }
}

fn display_coords(coords: &[(i32, i32)]) {
    let xmax = coords.iter().map(|c| c.0).max().unwrap() + 1;
    let ymax = coords.iter().map(|c| c.1).max().unwrap() + 1;
    let mut display: Vec<Vec<char>> = (0..ymax).map(|_| vec![' '; xmax as usize]).collect();
    for coord in coords {
        display[coord.1 as usize][coord.0 as usize] = '#';
    }
    for line in display {
        println!("{:?}", line.into_iter().collect::<String>());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();
    let groups: Vec<&str> = input.split("\n\n").collect();
    let coords: Vec<(i32, i32)> = groups[0]
        .lines()
        .map(|l| {
            let splt: Vec<&str> = l.split(',').collect();
            (splt[0].parse().unwrap(), splt[1].parse().unwrap())
        })
        .collect();
    let folds: Vec<(&str, i32)> = groups[1]
        .lines()
        .map(|l| {
            let splt1: Vec<&str> = l.split_whitespace().collect();
            let splt2: Vec<&str> = splt1[2].split('=').collect();
            (splt2[0], splt2[1].parse().unwrap())
        })
        .collect();

    let mut new_coords = coord_fold(&coords, &folds[0]);
    println!("{}", new_coords.iter().collect::<HashSet<_>>().len());
    for fold in &folds[1..] {
        new_coords = coord_fold(&new_coords, fold);
    }
    display_coords(&new_coords);
}
