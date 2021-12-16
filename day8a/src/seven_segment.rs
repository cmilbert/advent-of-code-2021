use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

const UNIQUE_DIGIT_SIGNAL_COUNTS: [usize; 4] = [
    2, // 1 Digit
    4, // 4 Digit
    3, // 7 Digit
    7, // 8 Digit
];

pub struct SevenSegment {
    pub input_values: Vec<Vec<String>>,
    pub output_values: Vec<Vec<String>>,
}

impl SevenSegment {
    pub fn new() -> Self {
        SevenSegment {
            input_values: Vec::new(),
            output_values: Vec::new(),
        }
    }

    pub fn read_lines_from_input_file(&mut self, filename: impl AsRef<Path>) {
        let file = File::open(filename).expect("no such file");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            // Output values: splt on pipe, split right hand side on space
            // Input values: split on pipe, split left hand side on space
            let line_input_output_strings: Vec<String> = line
                .unwrap()
                .split("|")
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect();

            let line_input_values: Vec<String> = line_input_output_strings[0]
                .split(" ")
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect();
            self.input_values.push(line_input_values);

            let line_output_values: Vec<String> = line_input_output_strings[1]
                .split(" ")
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect();
            self.output_values.push(line_output_values);
        }
    }

    pub fn count_1_4_7_8_output_values(&self) -> usize {
        let mut count: usize = 0;
        for i in 0..self.output_values.len() {
            for j in 0..self.output_values[i].len() {
                let output_value_length: usize = self.output_values[i][j].len();
                if UNIQUE_DIGIT_SIGNAL_COUNTS
                    .iter()
                    .any(|v| v == &output_value_length)
                {
                    count += 1;
                }
            }
        }
        return count;
    }
}

#[cfg(test)]
mod tests_day6a {
    use super::*;

    #[test]
    fn test_read_lines_from_input_file() {
        let mut seven_segment = SevenSegment::new();
        seven_segment.read_lines_from_input_file("sample_input.txt");

        assert_eq!(seven_segment.input_values.len(), 10);
        assert_eq!(seven_segment.input_values[0].len(), 10);

        assert_eq!(seven_segment.output_values.len(), 10);
        assert_eq!(seven_segment.output_values[0].len(), 4);
    }

    #[test]
    fn test_calculate_1_4_7_8_in_output_values() {
        let mut seven_segment = SevenSegment::new();
        seven_segment.read_lines_from_input_file("sample_input.txt");

        assert_eq!(seven_segment.count_1_4_7_8_output_values(), 26);
    }
}
