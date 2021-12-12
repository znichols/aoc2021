use std::env;
use hdrhistogram::Histogram;


fn main() {
    let args: Vec<String> = env::args().collect();

    let crab_pos: Vec<u64> = args[1].split(',').flat_map(|s| s.parse()).collect();
    let mut hist = Histogram::<u64>::new(4).unwrap();
    for pos in &crab_pos {
        hist += *pos;
    }
    let med = hist.value_at_percentile(50.0) as i64;

    let mut dist_sum = 0;
    for pos in &crab_pos {
        dist_sum += (med - *pos as i64).abs();
    }
    println!("{}, {}", med, dist_sum);


    let mean = hist.mean().floor();
    let mut dist_sum2 = 0.0;
    for pos in &crab_pos {
        let d = (mean - *pos as f64).abs();
        dist_sum2 += d * (d + 1.0) * 0.5;
    }

    println!("{}, {}", mean, dist_sum2);
}
