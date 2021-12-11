use ::cmilbert_aoc_day2b::Submarine;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn read_lines_from_input_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    let mut submarine = Submarine::new(0, 0, 0);
    let instructions = read_lines_from_input_file("input.txt");
    submarine.process_instructions(instructions);
    println!("Final position: {}", submarine.multiply_positions());
}
