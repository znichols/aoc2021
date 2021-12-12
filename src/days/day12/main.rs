use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

fn bfs_to_end(
    cave_map: &HashMap<&str, HashSet<&str>>,
    visited: &HashMap<String, usize>,
    loc: &str,
    small_cave_twice: bool,
) -> Vec<Vec<String>> {
    let mut r: Vec<Vec<String>> = Vec::new();
    let mut new_visited = visited.clone();
    if loc.to_lowercase() == *loc {
        *new_visited.entry(loc.to_string()).or_insert(0) += 1;
    }
    if loc == "end" {
        let endline = vec!["end".to_string()];
        r.push(endline);
    } else if cave_map.contains_key(loc) {
        for next_loc in &cave_map[loc] {
            if new_visited.contains_key(*next_loc) {
                if small_cave_twice && (*next_loc).len() < 3 {
                    if *new_visited.values().max().unwrap() > 1 {
                        continue;
                    }
                } else {
                    continue;
                }
            }
            let next_paths = bfs_to_end(cave_map, &new_visited, next_loc, small_cave_twice);
            for mut path in next_paths {
                if path[path.len() - 1] == "end" {
                    let mut new_path = vec![loc.to_string()];
                    new_path.append(&mut path);
                    r.push(new_path);
                }
            }
        }
    }
    r
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let cave_lines = fs::read_to_string(&args[1]).unwrap();
    let mut cave_map: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in cave_lines.lines() {
        let l = line.split('-').collect::<Vec<&str>>();
        cave_map
            .entry(l[0])
            .or_insert_with(HashSet::new)
            .insert(l[1]);
        cave_map
            .entry(l[1])
            .or_insert_with(HashSet::new)
            .insert(l[0]);
    }

    let r = bfs_to_end(&cave_map, &HashMap::new(), "start", false);
    let r2 = bfs_to_end(&cave_map, &HashMap::new(), "start", true);
    println!("{:?}, {:?}", r.len(), r2.len());
}
