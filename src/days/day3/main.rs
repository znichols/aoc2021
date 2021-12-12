use std::env;
use std::fs;

fn bit_count(input: &[&str], num_bits: usize, use_floor: bool) -> Vec<char> {
    let mut pos_count = vec![0; num_bits];
    let line_count = input.len();

    for line in input {
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                pos_count[i] += 1;
            }
        }
    }

    pos_count.iter().map(
        | x | {
            match use_floor {
                true => if *x * 2 > line_count {'1'} else {'0'},
                _ => if *x * 2 >= line_count {'1'} else {'0'}
            }
        }
    ).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filecontents = fs::read_to_string(&args[1]).unwrap();
    let lines: Vec<&str> = filecontents.lines().collect();
    let num_bits = lines[0].len();

    let gamma_bits = bit_count(&lines, num_bits, true);
    let gamma = i32::from_str_radix(&gamma_bits.iter().collect::<String>(), 2).unwrap();
    let epsilon = 2i32.pow(num_bits as u32) - 1 - gamma;

    let mut o2_lines = lines.clone();
    let mut co2_lines = lines.clone();
    for i in 0..num_bits {
        if o2_lines.len() > 1 {
            let cmp_bits = bit_count(&o2_lines, num_bits, false);
            o2_lines = o2_lines.iter().filter(
                |s| s.chars().nth(i).unwrap() == cmp_bits[i]
            ).map(|s| *s).collect();
        }
        if co2_lines.len() > 1 {
            let cmp_bits = bit_count(&co2_lines, num_bits, false);
            co2_lines = co2_lines.iter().filter(
                |s| s.chars().nth(i).unwrap() != cmp_bits[i]
            ).map(|s| *s).collect();
        }
    }
    let o2 = i32::from_str_radix(o2_lines[0], 2).unwrap();
    let co2 = i32::from_str_radix(co2_lines[0], 2).unwrap();
    
    println!("{:?}", epsilon * gamma);
    println!("{:?}", o2 * co2);
}
