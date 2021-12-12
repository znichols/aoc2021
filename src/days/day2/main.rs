use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filecontents = fs::read_to_string(&args[1]).unwrap();

    let mut pos = (0, 0);
    let mut aim = 0;
    for line in filecontents.lines() {
        let mut x = line.split(' ');
        let dir = x.next().unwrap();
        let dist: i32 = x.next().unwrap().parse().unwrap();
        match dir {
            "forward" => {
                pos.0 += dist;
                pos.1 += aim * dist;
            },
            "down" => aim += dist,
            _ => aim -= dist
        }
    }

    println!("{:?}", pos.0 * pos.1);
}

