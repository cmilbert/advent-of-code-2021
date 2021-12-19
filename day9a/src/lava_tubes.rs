use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

const RADIX: u32 = 10;

pub struct LavaTubes {
    pub input_values: Vec<Vec<u32>>,
    pub risk_level: u32,
}

impl Default for LavaTubes {
    fn default() -> Self {
        Self::new()
    }
}

impl LavaTubes {
    pub fn new() -> Self {
        LavaTubes {
            input_values: Vec::new(),
            risk_level: 0,
        }
    }

    pub fn read_lines_from_input_file(&mut self, filename: impl AsRef<Path>) {
        let file = File::open(filename).expect("no such file");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let mut line_values: Vec<u32> = Vec::new();
            for chracter in line.unwrap().chars() {
                line_values.push(chracter.to_digit(RADIX).unwrap());
            }
            self.input_values.push(line_values);
        }
    }

    pub fn is_low_point(&self, row_position: usize, col_position: usize) -> bool {
        let is_low_point_above: bool;
        let is_low_point_below: bool;
        let is_low_point_left: bool;
        let is_low_point_right: bool;

        let this_value: u32 = self.input_values[row_position][col_position];

        if row_position != 0 {
            // Check above
            let compare_to: u32 = self.input_values[row_position - 1][col_position];
            is_low_point_above = this_value < compare_to;
        } else {
            is_low_point_above = true;
        }

        if row_position + 1 < self.input_values.len() {
            // Check Below
            let compare_to: u32 = self.input_values[row_position + 1][col_position];
            is_low_point_below = this_value < compare_to;
        } else {
            is_low_point_below = true;
        }

        if col_position != 0 {
            // Check Left
            let compare_to: u32 = self.input_values[row_position][col_position - 1];
            is_low_point_left = this_value < compare_to;
        } else {
            is_low_point_left = true;
        }

        if col_position + 1 < self.input_values[row_position].len() {
            // Check Right
            let compare_to: u32 = self.input_values[row_position][col_position + 1];
            is_low_point_right = this_value < compare_to;
        } else {
            is_low_point_right = true;
        }

        is_low_point_above && is_low_point_below && is_low_point_left && is_low_point_right
    }

    pub fn calculate_risk_level(&mut self) -> u32 {
        for row in 0..self.input_values.len() {
            for col in 0..self.input_values[row].len() {
                if self.is_low_point(row, col) {
                    self.risk_level += 1 + self.input_values[row][col];
                }
            }
        }
        self.risk_level
    }
}

#[cfg(test)]
mod tests_day9a {
    use super::*;

    #[test]
    fn test_read_lines_from_input_file() {
        let mut lava_tubes = LavaTubes::new();
        lava_tubes.read_lines_from_input_file("sample_input.txt");

        assert_eq!(lava_tubes.input_values.len(), 5);
        assert_eq!(lava_tubes.input_values[0].len(), 10);
    }

    #[test]
    fn test_calculate_risk() {
        let mut lava_tubes = LavaTubes::new();
        lava_tubes.read_lines_from_input_file("sample_input.txt");

        assert_eq!(lava_tubes.calculate_risk_level(), 15);
    }
}
