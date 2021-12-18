use std::{
    fs::File,
    io::{prelude::*, BufReader, Error, ErrorKind},
};

pub fn read_lines_from_input_file(input_file_name: &str) -> Result<Vec<i32>, Error> {
    let io = File::open(input_file_name)?;
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

pub fn count_of_increasing_measurements_sliding_window(
    measurements: Vec<i32>,
    window_size: usize,
) -> i32 {
    let mut number_of_increasing_measurements = 0;
    let mut current_sum;
    let mut previous_sum = 0;
    for i in 0..(measurements.len() - window_size) {
        current_sum = 0;

        for measurement in measurements.iter().skip(i).take(window_size) {
            current_sum += measurement;
        }

        if current_sum > previous_sum {
            number_of_increasing_measurements += 1;
        }

        previous_sum = current_sum;
    }
    number_of_increasing_measurements
}

#[cfg(test)]
mod tests_day1b {
    use super::*;

    #[test]
    fn test_sample_data() {
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(
            count_of_increasing_measurements_sliding_window(measurements, 3),
            5
        )
    }
}
