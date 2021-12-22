use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;

fn run_game(start_pos: &[usize]) -> usize {
    let mut r = 0..1000000000;
    let mut pos = start_pos.to_vec();
    let mut points = vec![0; start_pos.len()];
    let mut cur_player = 0;
    let mut n_rolls = 0;
    while *points.iter().max().unwrap() < 1000 {
        let this_roll: usize = (0..3).map(|_| r.next().unwrap() % 100 + 1).sum();
        pos[cur_player] = (pos[cur_player] + this_roll) % 10;
        points[cur_player] += if pos[cur_player] == 0 {
            10
        } else {
            pos[cur_player]
        };
        n_rolls += 3;
        cur_player = (cur_player + 1) % pos.len();
    }
    n_rolls * points[cur_player]
}

#[derive(PartialEq, Eq, Hash)]
struct State {
    pos1: u8,
    pos2: u8,
    score1: u16,
    score2: u16,
}

impl State {
    fn update_p1(&self, roll: u8, update_score: bool) -> State {
        let new_pos = (self.pos1 + roll) % 10;
        match update_score {
            true => {
                let new_score = self.score1 as u16 + if new_pos == 0 { 10 } else { new_pos as u16 };
                State {
                    pos1: new_pos,
                    score1: new_score,
                    ..*self
                }
            }
            false => State {
                pos1: new_pos,
                ..*self
            },
        }
    }

    fn update_p2(&self, roll: u8, update_score: bool) -> State {
        let new_pos = (self.pos2 + roll) % 10;
        match update_score {
            true => {
                let new_score = self.score2 as u16 + if new_pos == 0 { 10 } else { new_pos as u16 };
                State {
                    pos2: new_pos,
                    score2: new_score,
                    ..*self
                }
            }
            false => State {
                pos2: new_pos,
                ..*self
            },
        }
    }
}

fn run_quantum_game(start_pos: &[usize]) -> usize {
    let mut game_states: HashMap<State, usize> = HashMap::new();
    game_states.insert(
        State {
            pos1: start_pos[0] as u8,
            pos2: start_pos[1] as u8,
            score1: 0,
            score2: 0,
        },
        1,
    );
    let mut p1_win_count = 0;
    let mut p2_win_count = 0;

    while !game_states.is_empty() {
        //player 1 goes
        for _ in 0..2 {
            let mut new_game_states: HashMap<State, usize> = HashMap::new();
            for (state, count) in &game_states {
                for roll in 1..4 {
                    let new_state = state.update_p1(roll, false);
                    *new_game_states.entry(new_state).or_insert(0) += count;
                }
            }
            game_states = new_game_states;
        }
        let mut new_game_states: HashMap<State, usize> = HashMap::new();
        for (state, count) in &game_states {
            for roll in 1..4 {
                let new_state = state.update_p1(roll, true);
                if new_state.score1 >= 21 {
                    p1_win_count += *count;
                } else {
                    *new_game_states.entry(new_state).or_insert(0) += count;
                }
            }
        }
        game_states = new_game_states;
        if game_states.is_empty() {
            break;
        }
        //player 2 goes
        for _ in 0..2 {
            let mut new_game_states: HashMap<State, usize> = HashMap::new();
            for (state, count) in &game_states {
                for roll in 1..4 {
                    let new_state = state.update_p2(roll, false);
                    *new_game_states.entry(new_state).or_insert(0) += count;
                }
            }
            game_states = new_game_states;
        }
        let mut new_game_states: HashMap<State, usize> = HashMap::new();
        for (state, count) in &game_states {
            for roll in 1..4 {
                let new_state = state.update_p2(roll, true);
                if new_state.score2 >= 21 {
                    p2_win_count += *count;
                } else {
                    *new_game_states.entry(new_state).or_insert(0) += count;
                }
            }
        }
        game_states = new_game_states;
    }
    std::cmp::max(p1_win_count, p2_win_count)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])?;
    let lines = input.lines();

    let start_pos: Vec<_> = lines
        .map(|line| line.split(": ").nth(1).unwrap().parse::<usize>().unwrap())
        .collect();

    println!("{}", run_game(&start_pos));
    println!("{}", run_quantum_game(&start_pos));

    Ok(())
}
