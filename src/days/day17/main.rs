use std::collections::{HashMap, HashSet};
use std::env;
use std::error::Error;
use std::fs;

fn get_bounds(line: &str) -> ((i32, i32), (i32, i32)) {
    let groups: Vec<Vec<i32>> = line
        .split(": ")
        .nth(1)
        .iter()
        .flat_map(|s| s.split(", "))
        .map(|s| {
            s.split('=')
                .nth(1)
                .iter()
                .flat_map(|s| s.split("..").map(|s| s.parse::<i32>().unwrap()))
                .collect::<Vec<i32>>()
        })
        .collect();
    ((groups[0][0], groups[0][1]), (groups[1][0], groups[1][1]))
}

fn get_t_vy_map(ybounds: (i32, i32), tmax: usize) -> HashMap<usize, Vec<i32>> {
    let mut r = HashMap::new();

    fn compute_v0(steps: usize, pos: i32) -> f64 {
        let y = pos as f64;
        let t = steps as f64;
        y / t + t * 0.5 - 0.5
    }

    for steps in 1..tmax {
        let a = compute_v0(steps, ybounds.1);
        let b = compute_v0(steps, ybounds.0);
        let mut v = b.ceil() as i32;
        while v <= a.floor() as i32 {
            r.entry(steps).or_insert_with(Vec::new).push(v);
            v += 1;
        }
    }

    r
}

fn get_t_vx_map(xbounds: (i32, i32), tmax: usize) -> HashMap<usize, Vec<i32>> {
    let mut r = HashMap::new();

    let steps_for_v = |v: i32| {
        let mut step_vec: Vec<usize> = Vec::new();
        let mut x = 0;
        let mut xv = v;
        let mut steps = 0;
        while x <= xbounds.1 {
            if x >= xbounds.0 {
                if xv == 0 {
                    for t in steps..tmax {
                        step_vec.push(t);
                    }
                    break;
                }
                step_vec.push(steps);
            } else if xv == 0 {
                break;
            }
            x += xv;
            xv -= xv.signum();
            steps += 1;
        }
        step_vec
    };

    for v in 1..xbounds.1 + 1 {
        for t in steps_for_v(v) {
            r.entry(t).or_insert_with(Vec::new).push(v);
        }
    }

    r
}

fn valid_vel_count(
    t_vx_map: &HashMap<usize, Vec<i32>>,
    t_vy_map: &HashMap<usize, Vec<i32>>,
) -> usize {
    let mut vel_set: HashSet<(i32, i32)> = HashSet::new();

    for (t, yv_vec) in t_vy_map {
        if !t_vy_map.contains_key(t) {
            continue;
        }
        let vx_vec = &t_vx_map[t];
        for vy in yv_vec {
            for vx in vx_vec {
                vel_set.insert((*vx, *vy));
            }
        }
    }

    vel_set.len()
}

fn max_yheight(t_vy_map: &HashMap<usize, Vec<i32>>) -> i32 {
    let max_v = t_vy_map.values().flatten().max().unwrap();
    let tmax = t_vy_map.keys().max().unwrap();
    (1..*tmax)
        .map(|t: usize| {
            let tf = t as f64;
            (tf * (*max_v as f64 + 0.5) - 0.5 * tf * tf) as i32
        })
        .max()
        .unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let bounds = get_bounds(&fs::read_to_string(&args[1])?);

    let t_vy_map = get_t_vy_map(bounds.1, 500);
    let t_vx_map = get_t_vx_map(bounds.0, 500);

    println!("{}", max_yheight(&t_vy_map));
    println!("{}", valid_vel_count(&t_vx_map, &t_vy_map));
    Ok(())
}
