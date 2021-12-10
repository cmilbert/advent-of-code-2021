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

pub fn count_of_increasing_measurements(measurements: Vec<i32>) -> i32 {
    let mut number_of_increasing_readings = 0;
    let mut previous_value = measurements[0];
    for line in measurements {
        if line > previous_value {
            number_of_increasing_readings += 1;
        }
        previous_value = line;
    }
    return number_of_increasing_readings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_data() {
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_of_increasing_measurements(measurements), 7)
    }
}
