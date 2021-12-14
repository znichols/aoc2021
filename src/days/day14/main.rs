use std::collections::HashMap;
use std::env;
use std::fs;

fn update_seq(seq: &[char], insert_map: &HashMap<(char, char), Vec<char>>) -> Vec<char> {
    let l = seq.len();
    (1..l)
        .flat_map(|i| {
            let mut s: Vec<char> = Vec::new();
            if i == 1 {
                s.push(seq[0]);
            }
            let k = (seq[i - 1], seq[i]);
            if insert_map.contains_key(&k) {
                s.extend(&insert_map[&k]);
            }
            s.push(seq[i]);
            s
        })
        .collect()
}

fn compute_map(
    insert_maps: &mut HashMap<usize, HashMap<(char, char), Vec<char>>>,
    level_start: usize,
    level_add: usize,
) {
    let new_level = level_start + level_add;
    let start_map = &insert_maps[&level_start];
    let update_map = &insert_maps[&level_add];
    let new_map: HashMap<(char, char), Vec<char>> = start_map
        .iter()
        .map(|(k, v)| {
            let mut this_v = vec![k.0];
            this_v.extend(v);
            this_v.push(k.1);
            let updated_v = update_seq(&this_v, update_map);
            (*k, updated_v[1..updated_v.len() - 1].to_vec())
        })
        .collect();
    insert_maps.insert(new_level, new_map);
}

fn shortcut_count_diff(seq: &[char], insert_map: &HashMap<(char, char), Vec<char>>) -> usize {
    let insert_counts: HashMap<(char, char), HashMap<char, usize>> = insert_map
        .iter()
        .map(|(k, v)| {
            let mut char_counts: HashMap<char, usize> = HashMap::new();
            for c in v {
                *char_counts.entry(*c).or_default() += 1;
            }
            (*k, char_counts)
        })
        .collect();

    let mut char_counts: HashMap<char, usize> = HashMap::new();
    for c in seq {
        *char_counts.entry(*c).or_default() += 1;
    }
    for i in 1..seq.len() {
        let insert_count = &insert_counts[&(seq[i - 1], seq[i])];
        for (k, v) in insert_count {
            *char_counts.entry(*k).or_default() += v;
        }
    }

    char_counts.values().max().unwrap() - char_counts.values().min().unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();
    let groups: Vec<&str> = input.split("\n\n").collect();
    let seq: Vec<char> = groups[0].chars().collect();

    let mut insert_maps: HashMap<usize, HashMap<(char, char), Vec<char>>> = HashMap::new();
    insert_maps.insert(
        1,
        groups[1]
            .lines()
            .map(|l| {
                let mut s = l.split(" -> ");
                let mut k = s.next().unwrap().chars();
                let v = vec![s.next().unwrap().chars().next().unwrap()];
                ((k.next().unwrap(), k.next().unwrap()), v)
            })
            .collect(),
    );
    for i in 1..11 {
        compute_map(&mut insert_maps, i, 1);
    }

    let seq10 = update_seq(&seq, &insert_maps[&10]);
    let mut char_counts: HashMap<char, usize> = HashMap::new();
    for c in &seq10 {
        *char_counts.entry(*c).or_default() += 1;
    }
    println!(
        "{:?}",
        char_counts.values().max().unwrap() - char_counts.values().min().unwrap()
    );

    compute_map(&mut insert_maps, 10, 10);
    let seq20 = update_seq(&seq10, &insert_maps[&10]);
    println!("{:?}", shortcut_count_diff(&seq20, &insert_maps[&20]));
}
