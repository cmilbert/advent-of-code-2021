use ::cmilbert_aoc_day3a::PowerDiagnostic;

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
    let mut power_diagnostic = PowerDiagnostic::new();
    let binary_input = read_lines_from_input_file("input.txt");

    power_diagnostic.calculate_gamma_rate(&binary_input);
    power_diagnostic.calculate_epsilon_rate(&binary_input);
    println!("Power consumption: {}", power_diagnostic.power_consumption);
}
