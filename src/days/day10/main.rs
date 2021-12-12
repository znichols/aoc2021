use std::env;
use std::fs;
use std::collections::VecDeque;

fn score_line(line: &str) -> i64 {
    let mut chunk_stack: VecDeque<char> = VecDeque::new();
    for c in line.chars() {
        match c {
            ']' => {
                if chunk_stack.is_empty() || chunk_stack.pop_back() != Some('[') {
                    return 57;
                }
            },
            ')' => {
                if chunk_stack.is_empty() || chunk_stack.pop_back() != Some('(') {
                    return 3;
                }
            },
            '}' => {
                if chunk_stack.is_empty() || chunk_stack.pop_back() != Some('{') {
                    return 1197;
                }
            },
            '>' => {
                if chunk_stack.is_empty() || chunk_stack.pop_back() != Some('<') {
                    return 25137;
                }
            },
            _ => chunk_stack.push_back(c)
        }
    }
    let mut score = 0;
    while !chunk_stack.is_empty() {
        score *= 5;
        score -= match chunk_stack.pop_back().unwrap() {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            _ => 4
        }
    }
    score
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filecontents = fs::read_to_string(&args[1]).unwrap();
    let scores: Vec<i64> = filecontents.lines().map(|l| score_line(l)).collect();
    let mut comp_scores: Vec<i64> = scores.iter().filter(|&&s| s < 0).copied().collect();
    comp_scores.sort_unstable();
    println!("{:?}, {:?}", 
        scores.iter().filter(|&&s| s > 0).sum::<i64>(),
        -comp_scores[comp_scores.len() / 2]
    );
}

