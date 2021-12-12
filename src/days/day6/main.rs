use std::env;

fn update_fish(fish: &mut [u64]) {
    let new_fish_count = fish[0];
    for i in 1..fish.len() {
        fish[i - 1] = fish[i];
    }
    fish[6] += new_fish_count;
    fish[8] = new_fish_count;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let fish_ages: Vec<usize> = args[1].split(',').flat_map(|s| s.parse()).collect();
    let days_to_sim: i32 = args[2].parse().unwrap();

    let mut fish_buckets: Vec<u64> = vec![0; 9];
    for age in fish_ages {
        fish_buckets[age] += 1;
    }

    println!("{:?}", fish_buckets);
    for _ in 0..days_to_sim {
        update_fish(&mut fish_buckets);
    }
    println!("{:?}", fish_buckets);
    println!("{}", fish_buckets.iter().sum::<u64>());
}
