use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Debug)]
struct NumberResolver {
    code_strings: Vec<String>,
    code_sets: Vec<HashSet<char>>,
}

impl NumberResolver {
    fn new() -> NumberResolver {
        NumberResolver {
            code_strings: vec!["".to_string(); 10],
            code_sets: vec![HashSet::<char>::new(); 10],
        }
    }

    fn put_pattern(&mut self, pattern: &str, pos: usize) {
        self.code_strings[pos] = pattern.to_string();
        self.code_sets[pos] = pattern.chars().collect::<HashSet<char>>();
    }

    fn resolve(&mut self, pattern_str: &str) {
        let patterns: Vec<&str> = pattern_str.split_whitespace().collect();
        for s in patterns.iter() {
            match s.len() {
                2 => self.put_pattern(s, 1),
                3 => self.put_pattern(s, 7),
                4 => self.put_pattern(s, 4),
                7 => self.put_pattern(s, 8),
                _ => {}
            }
        }
        for s in patterns.iter() {
            match s.len() {
                5 => {
                    let code_set = s.chars().collect::<HashSet<char>>();
                    if code_set.intersection(&self.code_sets[7]).count() == 3 {
                        self.put_pattern(s, 3);
                    } else if code_set.intersection(&self.code_sets[4]).count() == 3 {
                        self.put_pattern(s, 5);
                    } else {
                        self.put_pattern(s, 2);
                    }
                }
                6 => {
                    let code_set = s.chars().collect::<HashSet<char>>();
                    if code_set.intersection(&self.code_sets[7]).count() == 2 {
                        self.put_pattern(s, 6);
                    } else if code_set.intersection(&self.code_sets[4]).count() == 4 {
                        self.put_pattern(s, 9);
                    } else {
                        self.put_pattern(s, 0);
                    }
                }
                _ => {}
            }
        }
    }

    fn look_up(&self, pattern: &str) -> usize {
        let code_set = pattern.chars().collect::<HashSet<char>>();
        let mut i = 0;
        while code_set.intersection(&self.code_sets[i]).count()
            != code_set.union(&self.code_sets[i]).count()
        {
            i += 1;
        }
        i
    }
}

fn main() {
    let unique_led_counts: HashSet<usize> = [2, 3, 4, 7].iter().cloned().collect();
    let args: Vec<String> = env::args().collect();
    let filecontents = fs::read_to_string(&args[1]).unwrap();

    let r1: Vec<_> = filecontents
        .lines()
        .map(|s| {
            s.split(" | ").collect::<Vec<_>>()[1]
                .split_whitespace()
                .filter(|s| unique_led_counts.contains(&s.len()))
                .count()
        })
        .collect();
    println!("{:?}", r1.iter().sum::<usize>());

    let mut sum = 0;
    for line in filecontents.lines() {
        let mut resolver = NumberResolver::new();
        let spt = line.split(" | ").collect::<Vec<_>>();
        resolver.resolve(spt[0]);

        sum += spt[1]
            .split_whitespace()
            .map(|s| resolver.look_up(s).to_string())
            .collect::<Vec<_>>()
            .join("")
            .parse::<i32>()
            .unwrap();
    }
    println!("{:?}", sum);
}
