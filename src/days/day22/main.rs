use ndarray::prelude::*;
use std::cmp::{max, min};
use std::env;
use std::error::Error;
use std::fs;

fn bound(x: i32) -> usize {
    min(max(0, x + 50), 101) as usize
}

fn engage(instruction: &(&str, Vec<(i32, i32)>), engine_cube: &mut Array3<bool>) {
    let mut s = engine_cube.slice_mut(s![
        bound(instruction.1[0].0)..bound(instruction.1[0].1),
        bound(instruction.1[1].0)..bound(instruction.1[1].1),
        bound(instruction.1[2].0)..bound(instruction.1[2].1),
    ]);

    match instruction.0 {
        "on" => s.fill(true),
        _ => s.fill(false),
    };
}

struct EngineBox {
    box_type: String,
    xspan: (i64, i64),
    yspan: (i64, i64),
    zspan: (i64, i64),
    layered_subboxes: Vec<EngineBox>,
}

impl EngineBox {
    fn from_instruction(instruction: &(&str, Vec<(i32, i32)>)) -> EngineBox {
        EngineBox {
            box_type: instruction.0.to_string(),
            xspan: (instruction.1[0].0 as i64, instruction.1[0].1 as i64),
            yspan: (instruction.1[1].0 as i64, instruction.1[1].1 as i64),
            zspan: (instruction.1[2].0 as i64, instruction.1[2].1 as i64),
            layered_subboxes: Vec::new(),
        }
    }

    fn get_overlap_box(&self, other: &EngineBox, box_type: &str) -> Option<EngineBox> {
        let xspan = (
            max(self.xspan.0, other.xspan.0),
            min(self.xspan.1, other.xspan.1),
        );
        let yspan = (
            max(self.yspan.0, other.yspan.0),
            min(self.yspan.1, other.yspan.1),
        );
        let zspan = (
            max(self.zspan.0, other.zspan.0),
            min(self.zspan.1, other.zspan.1),
        );
        if xspan.1 <= xspan.0 || yspan.1 <= yspan.0 || zspan.1 <= zspan.0 {
            None
        } else {
            Some(EngineBox {
                box_type: box_type.to_string(),
                xspan,
                yspan,
                zspan,
                layered_subboxes: Vec::new(),
            })
        }
    }

    fn check_layer(&mut self, other: &EngineBox) {
        if let Some(new_subbox) = self.get_overlap_box(other, "neg_subbox") {
            let mut new_intersections: Vec<EngineBox> = self
                .layered_subboxes
                .iter()
                .flat_map(|subbox| match subbox.box_type.as_str() {
                    "neg_subbox" => subbox.get_overlap_box(other, "pos_subbox"),
                    _ => subbox.get_overlap_box(other, "neg_subbox"),
                })
                .collect();
            self.layered_subboxes.push(new_subbox);
            self.layered_subboxes.append(&mut new_intersections);
        }
    }

    fn raw_area(&self) -> i64 {
        (self.xspan.1 - self.xspan.0)
            * (self.yspan.1 - self.yspan.0)
            * (self.zspan.1 - self.zspan.0)
    }

    fn real_area(&self) -> i64 {
        if self.box_type == "off" {
            0
        } else {
            let mut total_area = self.raw_area();
            for subbox in &self.layered_subboxes {
                total_area += match subbox.box_type.as_str() {
                    "pos_subbox" => subbox.raw_area(),
                    _ => -subbox.raw_area(),
                }
            }

            total_area
        }
    }
}

fn engine_sum(engine_cube: &Array3<bool>) -> u64 {
    engine_cube.iter().map(|e| if *e { 1 } else { 0 }).sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])?;
    let lines = input.lines();

    let instructions: Vec<_> = lines
        .map(|line| {
            let mut s = line.split(' ');
            let word = s.next().unwrap();
            let coords: Vec<_> = s
                .next()
                .unwrap()
                .split(',')
                .map(|s| {
                    let mut x = s
                        .split('=')
                        .nth(1)
                        .unwrap()
                        .split("..")
                        .map(|s| s.parse::<i32>().unwrap());
                    (x.next().unwrap(), x.next().unwrap() + 1)
                })
                .collect();
            (word, coords)
        })
        .collect();

    let mut engine_cube: Array3<bool> = Array3::from_elem((101, 101, 101), false);
    let mut engine_boxes: Vec<EngineBox> = Vec::new();
    for instruction in &instructions {
        engage(instruction, &mut engine_cube);
        let new_box = EngineBox::from_instruction(instruction);
        for old_box in &mut engine_boxes {
            old_box.check_layer(&new_box);
        }
        engine_boxes.push(new_box);
    }

    println!("{}", engine_sum(&engine_cube));
    let s: i64 = engine_boxes.iter().map(|b| b.real_area()).sum();
    println!("{}", s);

    Ok(())
}
