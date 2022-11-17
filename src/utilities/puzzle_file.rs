use std::fs::{File, self};


#[derive(Default)]
pub struct Puzzle {
    dividend: String,
    divisor: String,
    equations: Vec<Equation>,
    letters: String,
}


#[derive(Default)]
pub struct Equation {
    minuend: String,
    subtrahend: String,
    difference: String,
}

pub fn parse(path: &String) -> Puzzle {
    let pzl = Puzzle::default();
    let file = fs::read_to_string(path).expect("Could not find puzzle file");
    let buf_minuend = String::new();
    let buf_subtrahend = String::new();
    let mut lines: Vec<&str> = file.lines().collect();
    for i in 0..lines.len() {
        if lines[i].contains('=') {
            let buf_difference =
        }
    }
    pzl
}