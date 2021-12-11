use ::cmilbert_aoc_day1b::{
    count_of_increasing_measurements_sliding_window, read_lines_from_input_file,
};
use std::io::Error;

fn main() -> Result<(), Error> {
    let measurements = read_lines_from_input_file("input.txt")?;
    let increasing_measurements_count =
        count_of_increasing_measurements_sliding_window(measurements, 3);
    println!("Increasing readings: {}", increasing_measurements_count);
    Ok(())
}
