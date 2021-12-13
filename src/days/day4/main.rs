use std::env;
use std::fs;

#[derive(Debug)]
struct Board {
    size: usize,
    numbers: Vec<Vec<i32>>,
    marks: Vec<Vec<bool>>,
    done: bool
}

impl Board {
    fn mark(&mut self, n: i32) {
        for i in 0..self.size {
            for j in 0..self.size {
                if self.numbers[i][j] == n {
                    self.marks[i][j] = true;
                }
            }
        }
    }

    fn sum_unmarked(&self) -> i32 {
        let mut r = 0;
        for i in 0..self.size {
            for j in 0..self.size {
                if !self.marks[i][j] {
                    r += self.numbers[i][j];
                }
            }
        }
        r
    }

    fn check(&self) -> bool {
        let mut vert = vec![true; self.size];
        let mut horz = vec![true; self.size];
        for i in 0..self.size {
            for j in 0..self.size {
                vert[i] &= self.marks[i][j];
                horz[j] &= self.marks[i][j];
            }
        }
        let v = vert.iter().copied().reduce(|a, b| a || b).unwrap();
        let h = horz.iter().copied().reduce(|a, b| a || b).unwrap();
        v || h
    }

    fn from_slice(input: &[&str]) -> Option<Board> {
        if input.is_empty() || input[0].is_empty() {
            return None
        }
        let mut numbers: Vec<Vec<i32>> = Vec::new();
        let mut i = 0;
        while i < input.len() && !input[i].is_empty() {
            let v: Vec<i32> = input[i].split(' ').flat_map(|s| s.parse()).collect();
            numbers.push(v);
            i += 1;
        }

        let size = numbers.len();
        let marks = vec![vec![false; size]; size];
        Some(
            Board {
                size,
                numbers,
                marks,
                done: false
            }
        )
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filecontents = fs::read_to_string(&args[1]).unwrap();
    let lines: Vec<&str> = filecontents.lines().collect();

    let call_nums: Vec<i32> = lines[0].split(',').flat_map(|s| s.parse()).collect();

    let mut i = 2;
    let mut boards: Vec<Board> = Vec::new();
    while let Some(board) = Board::from_slice(&lines[i..]) {
        i += 1 + board.size;
        boards.push(board);
        if i >= lines.len() {
            break
        }
    }

    let mut finished_count = 0;
    let l = boards.len();
    for n in call_nums {
        for board in boards.iter_mut().filter(|b| !b.done) {
            board.mark(n);
            if board.check() {
                if finished_count == 0 {
                    println!("First winning board score: {:?}", n * board.sum_unmarked());
                } else if finished_count == l - 1 {
                    println!("Last winning board score: {:?}", n * board.sum_unmarked());
                }
                board.done = true;
                finished_count += 1;
            }
        }
    }
} 
