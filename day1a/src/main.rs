use ::cmilbert_aoc_day1a::{count_of_increasing_measurements, read_lines_from_input_file};
use std::io::Error;

fn main() -> Result<(), Error> {
    let measurements = read_lines_from_input_file("input.txt")?;
    let increasing_measurements_count = count_of_increasing_measurements(measurements);
    println!("Increasing readings: {}", increasing_measurements_count);
    Ok(())
}
