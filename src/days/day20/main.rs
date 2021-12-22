use ndarray::prelude::*;
use std::env;
use std::error::Error;
use std::fs;

fn expand_img_borders(img: &Array2<char>, fill_char: char) -> Array2<char> {
    let mut new_img = Array2::from_elem((img.nrows() + 16, img.ncols() + 16), fill_char);
    new_img
        .slice_mut(s![8..img.nrows() + 8, 8..img.ncols() + 8])
        .assign(img);
    new_img
}

fn enhance(
    img: &Array2<char>,
    key: &[char],
    fill_char: char,
) -> Result<Array2<char>, Box<dyn Error>> {
    let expanded_img = expand_img_borders(img, fill_char);
    let mut new_img = Array2::from_elem((img.nrows() + 8, img.ncols() + 8), '0');

    for i in 0..new_img.nrows() {
        for j in 0..new_img.ncols() {
            let n_str: String = expanded_img.slice(s![i..i + 3, j..j + 3]).iter().collect();
            let n = usize::from_str_radix(&n_str, 2)?;
            new_img[[i, j]] = key[n];
        }
    }

    Ok(new_img)
}

fn count_pixels(img: &Array2<char>) -> usize {
    img.iter()
        .map(|c| match *c {
            '1' => 1,
            _ => 0,
        })
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])?;
    let mut lines = input.lines();

    let cmap = |c| match c {
        '.' => '0',
        _ => '1',
    };

    let key: Vec<char> = lines.next().unwrap().chars().map(cmap).collect();
    lines.next();
    let img_chars: Vec<Vec<char>> = lines.map(|l| l.chars().map(cmap).collect()).collect();
    let img = Array2::from_shape_vec(
        (img_chars.len(), img_chars[0].len()),
        img_chars.iter().flatten().copied().collect(),
    )?;

    let mut new_img = enhance(&img, &key, '0')?;
    let mut fill_char = key[0];
    new_img = enhance(&new_img, &key, fill_char)?;
    let p2 = count_pixels(&new_img);
    for _ in 2..50 {
        let fill_ind_str: String = vec![fill_char; 9].iter().collect();
        fill_char = key[usize::from_str_radix(&fill_ind_str, 2)?];
        new_img = enhance(&new_img, &key, fill_char)?;
    }
    let p50 = count_pixels(&new_img);

    println!("{}, {}", p2, p50);

    Ok(())
}
