use std::env;
use std::fs;

fn count_increases(v: &[i32]) -> i32 {
    v[1..].iter()
        .zip(v.iter())
        .map(|x| if x.0 > x.1 { 1 } else { 0 })
        .sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filecontents = fs::read_to_string(&args[1]).unwrap();
    let v: Vec<i32> = filecontents.split('\n').flat_map(|s| s.parse()).collect();

    let vconv3: Vec<i32> = (0..v.len()-2).map(
        |i| v[i..i+3].iter().sum::<i32>()
    ).collect();

    println!("{:?}", count_increases(&v));
    println!("{:?}", count_increases(&vconv3));
}