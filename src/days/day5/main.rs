use std::cmp;
use std::env;
use std::fs;

#[derive(Debug, Eq, PartialEq)]
enum Orientation {
    West,
    East,
    North,
    South,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Other,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn step(&mut self, d: &Orientation) {
        match d {
            Orientation::South => self.y -= 1,
            Orientation::North => self.y += 1,
            Orientation::West => self.x -= 1,
            Orientation::East => self.x += 1,
            Orientation::NorthEast => {
                self.x += 1;
                self.y += 1
            }
            Orientation::NorthWest => {
                self.x -= 1;
                self.y += 1
            }
            Orientation::SouthEast => {
                self.x += 1;
                self.y -= 1
            }
            Orientation::SouthWest => {
                self.x -= 1;
                self.y -= 1
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn from_str(s: &str) -> Option<Line> {
        let fields: Vec<&str> = s.split(" -> ").collect();
        if fields.len() < 2 {
            None
        } else {
            let p1_split: Vec<&str> = fields[0].split(',').collect();
            let p1 = Point {
                x: p1_split[0].parse().unwrap(),
                y: p1_split[1].parse().unwrap(),
            };
            let p2_split: Vec<&str> = fields[1].split(',').collect();
            let p2 = Point {
                x: p2_split[0].parse().unwrap(),
                y: p2_split[1].parse().unwrap(),
            };
            Some(Line { p1, p2 })
        }
    }

    fn max_val(&self) -> u32 {
        cmp::max(
            cmp::max(self.p1.x, self.p1.y),
            cmp::max(self.p2.x, self.p2.y),
        )
    }

    fn orientation(&self) -> Orientation {
        if self.p1.x == self.p2.x {
            if self.p1.y < self.p2.y {
                Orientation::North
            } else {
                Orientation::South
            }
        } else if self.p1.y == self.p2.y {
            if self.p1.x < self.p2.x {
                Orientation::East
            } else {
                Orientation::West
            }
        } else if (self.p1.x as i32 - self.p2.x as i32).abs()
            == (self.p1.y as i32 - self.p2.y as i32).abs()
        {
            if self.p1.x < self.p2.x {
                if self.p1.y < self.p2.y {
                    Orientation::NorthEast
                } else {
                    Orientation::SouthEast
                }
            } else if self.p1.y < self.p2.y {
                Orientation::NorthWest
            } else {
                Orientation::SouthWest
            }
        } else {
            Orientation::Other
        }
    }

    fn mark(&self, map: &mut Vec<Vec<u32>>) {
        let o = self.orientation();
        if o == Orientation::Other {
            return;
        }
        let mut p = self.p1.clone();
        map[p.x as usize][p.y as usize] += 1;
        while p != self.p2 {
            p.step(&o);
            map[p.x as usize][p.y as usize] += 1;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filecontents = fs::read_to_string(&args[1]).unwrap();

    let vent_lines: Vec<Line> = filecontents.lines().flat_map(Line::from_str).collect();
    let area_size = vent_lines
        .iter()
        .map(|l| l.max_val())
        .reduce(cmp::max)
        .map(|x| x as usize)
        .unwrap()
        + 1;

    let mut map: Vec<Vec<u32>> = vec![vec![0; area_size]; area_size];
    for line in vent_lines {
        line.mark(&mut map);
    }

    let mut overlapped_count = 0;
    for i in 0..area_size {
        for j in 0..area_size {
            if map[i][j] > 1 {
                overlapped_count += 1;
            }
        }
    }

    println!("{}", overlapped_count);
}
