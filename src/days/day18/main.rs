use std::collections::VecDeque;
use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::ops::Add;

#[derive(Clone)]
enum FishElement {
    E(i32),
    SubNum(FishNum),
}

impl fmt::Debug for FishElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FishElement::E(x) => write!(f, "{}", x),
            FishElement::SubNum(s) => write!(f, "{:?}", s),
        }
    }
}

impl FishElement {
    fn is_subnum(&self) -> bool {
        matches!(self, &FishElement::SubNum(_))
    }

    fn is_shallow_subnum(&self) -> bool {
        self.is_subnum() && self.get_vals_if_simple().is_some()
    }

    fn get_vals_if_simple(&self) -> Option<(i32, i32)> {
        match &*self {
            FishElement::SubNum(s) => s.get_vals_if_simple(),
            _ => None,
        }
    }

    fn add(&mut self, e: i32) -> Result<(), Box<dyn Error>> {
        match self {
            FishElement::E(x) => {
                *x += e;
                Ok(())
            }
            _ => Err("Set Error".into()),
        }
    }

    fn get_subnum_mut(&mut self) -> Result<&mut FishNum, Box<dyn Error>> {
        match self {
            FishElement::SubNum(s) => Ok(s),
            _ => Err("Get Error".into()),
        }
    }
}

#[derive(Clone)]
struct FishNum {
    left: Box<FishElement>,
    right: Box<FishElement>,
}

impl fmt::Debug for FishNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?}, {:?}]", self.left, self.right)
    }
}

impl FishNum {
    fn from_str(input: &str) -> Result<FishNum, Box<dyn Error>> {
        let mut i: usize = 0;
        if input.chars().nth(i) != Some('[') {
            return Err("Parse Error".into());
        }

        let parse_element = |mut i| match input.chars().nth(i).unwrap() {
            '[' => {
                let start = i;
                let mut pcount = 1;
                while pcount > 0 {
                    i += 1;
                    match input.chars().nth(i).unwrap() {
                        '[' => pcount += 1,
                        ']' => pcount -= 1,
                        _ => {}
                    }
                }
                let subnum = FishNum::from_str(&input[start..i + 1]).unwrap();
                (FishElement::SubNum(subnum), i + 1)
            }
            _ => (FishElement::E(input[i..i + 1].parse().unwrap()), i + 1),
        };

        i += 1;
        let left = parse_element(i);
        i = left.1;
        let right = parse_element(i + 1);
        Ok(FishNum {
            left: left.0.into(),
            right: right.0.into(),
        })
    }

    fn get_vals_if_simple(&self) -> Option<(i32, i32)> {
        let l = match *self.left {
            FishElement::E(e) => e,
            _ => -1,
        };
        let r = match *self.right {
            FishElement::E(e) => e,
            _ => -1,
        };

        if l >= 0 && r >= 0 {
            Some((l, r))
        } else {
            None
        }
    }

    fn explodewalk<'a>(
        &'a mut self,
        depth: usize,
        left_e: Option<&'a mut FishElement>,
        right_residual: Option<i32>,
    ) -> (Option<i32>, Option<&'a mut FishElement>) {
        // check left
        if right_residual.is_none() && depth >= 3 && self.left.is_shallow_subnum() {
            // first explosion on a left side
            let residual = self.left.get_vals_if_simple().unwrap();
            left_e.map(|e| e.add(residual.0));
            self.left = FishElement::E(0).into();

            match *self.right {
                FishElement::E(_) => {
                    self.right.add(residual.1).unwrap();
                    (Some(0), None)
                }
                FishElement::SubNum(_) => self.right.get_subnum_mut().unwrap().explodewalk(
                    depth + 1,
                    None,
                    Some(residual.1),
                ),
            }
        } else {
            let mut new_right_residual: Option<i32> = None;
            let mut new_left_e: Option<&'a mut FishElement> = None;

            //walk left
            if self.left.is_subnum() {
                let x = self.left.get_subnum_mut().unwrap().explodewalk(
                    depth + 1,
                    left_e,
                    right_residual,
                );
                new_right_residual = x.0;
                new_left_e = x.1;
            } else if right_residual.is_some() {
                self.left.add(right_residual.unwrap()).unwrap();
                new_right_residual = Some(0);
                new_left_e = left_e;
            } else {
                new_left_e = Some(&mut self.left);
            }

            // check right
            if right_residual.is_none() && depth >= 3 && self.right.is_shallow_subnum() {
                // first explosion on a right side
                let residual = self.right.get_vals_if_simple().unwrap();
                new_left_e.map(|e| e.add(residual.0));
                self.right = FishElement::E(0).into();
                (Some(residual.1), None)
            } else {
                if new_right_residual.is_none() {
                    new_right_residual = right_residual;
                }

                if self.right.is_subnum() {
                    let x = self.right.get_subnum_mut().unwrap().explodewalk(
                        depth + 1,
                        new_left_e,
                        new_right_residual,
                    );
                    new_right_residual = x.0;
                    new_left_e = x.1;
                } else if new_right_residual.is_some() {
                    self.right.add(new_right_residual.unwrap()).unwrap();
                    new_right_residual = Some(0);
                } else {
                    new_left_e = Some(&mut self.right);
                }
                (new_right_residual, new_left_e)
            }
        }
    }

    fn splitwalk(&mut self) -> bool {
        let mut done = false;
        match *self.left {
            FishElement::E(e) => {
                if e > 9 {
                    let a = e / 2;
                    let b = e - a;
                    self.left = FishElement::SubNum(FishNum {
                        left: FishElement::E(a).into(),
                        right: FishElement::E(b).into(),
                    })
                    .into();
                    done = true;
                }
            }
            _ => {
                done = self.left.get_subnum_mut().unwrap().splitwalk();
            }
        }

        if !done {
            match *self.right {
                FishElement::E(e) => {
                    if e > 9 {
                        let a = e / 2;
                        let b = e - a;
                        self.right = FishElement::SubNum(FishNum {
                            left: FishElement::E(a).into(),
                            right: FishElement::E(b).into(),
                        })
                        .into();
                        done = true;
                    }
                }
                _ => {
                    done = self.right.get_subnum_mut().unwrap().splitwalk();
                }
            }
        }
        done
    }

    fn reduce(&mut self) -> bool {
        let delta = self.explodewalk(0, None, None);
        match delta.0 {
            None => self.splitwalk(),
            _ => true,
        }
    }

    fn magnitude(&self, lmul: i32, rmul: i32) -> i32 {
        let l = match &*self.left {
            FishElement::E(e) => *e,
            FishElement::SubNum(s) => s.magnitude(lmul, rmul),
        };
        let r = match &*self.right {
            FishElement::E(e) => *e,
            FishElement::SubNum(s) => s.magnitude(lmul, rmul),
        };
        lmul * l + rmul * r
    }

    fn add(left: FishNum, right: FishNum) -> FishNum {
        let mut r = FishNum {
            left: FishElement::SubNum(left).into(),
            right: FishElement::SubNum(right).into(),
        };

        while r.reduce() {}
        r
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let mut fishnums: VecDeque<FishNum> = VecDeque::new();
    let input = fs::read_to_string(&args[1])?;
    for line in input.lines() {
        fishnums.push_back(FishNum::from_str(line)?);
    }

    let mut maxsum = 0;
    for i in 0..fishnums.len() {
        for j in 0..fishnums.len() {
            if i == j {
                continue;
            }
            let a = fishnums[i].clone();
            let b = fishnums[j].clone();
            let s = FishNum::add(a, b).magnitude(3, 2);
            if s > maxsum {
                maxsum = s
            }
        }
    }

    let mut num = fishnums.pop_front().unwrap();
    for n in fishnums {
        num = FishNum::add(num, n);
    }

    println!("{:?}, {:?}", num.magnitude(3, 2), maxsum);

    Ok(())
}
