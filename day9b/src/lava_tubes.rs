use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

const RADIX: u32 = 10;
const HIGH_POINT: u32 = 9;

pub struct LavaTubes {
    pub input_values: Vec<Vec<u32>>,
    pub risk_level: u32,
    pub basin_sizes: Vec<u32>,
    pub visited_points: Vec<(usize, usize)>,
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
            basin_sizes: Vec::new(),
            visited_points: Vec::new(),
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

    pub fn is_low_point_above(
        &self,
        row_position: usize,
        col_position: usize,
        compare_value: u32,
    ) -> bool {
        if row_position != 0 {
            let compare_to: u32 = self.input_values[row_position - 1][col_position];
            compare_value < compare_to
        } else {
            true
        }
    }

    pub fn is_low_point_below(
        &self,
        row_position: usize,
        col_position: usize,
        compare_value: u32,
    ) -> bool {
        if row_position + 1 < self.input_values.len() {
            let compare_to: u32 = self.input_values[row_position + 1][col_position];
            compare_value < compare_to
        } else {
            true
        }
    }

    pub fn is_low_point_left(
        &self,
        row_position: usize,
        col_position: usize,
        compare_value: u32,
    ) -> bool {
        if col_position != 0 {
            let compare_to: u32 = self.input_values[row_position][col_position - 1];
            compare_value < compare_to
        } else {
            true
        }
    }

    pub fn is_low_point_right(
        &self,
        row_position: usize,
        col_position: usize,
        compare_value: u32,
    ) -> bool {
        if col_position + 1 < self.input_values[row_position].len() {
            let compare_to: u32 = self.input_values[row_position][col_position + 1];
            compare_value < compare_to
        } else {
            true
        }
    }

    pub fn is_low_point(&self, row_position: usize, col_position: usize) -> bool {
        let compare_value: u32 = self.input_values[row_position][col_position];

        self.is_low_point_above(row_position, col_position, compare_value)
            && self.is_low_point_below(row_position, col_position, compare_value)
            && self.is_low_point_left(row_position, col_position, compare_value)
            && self.is_low_point_right(row_position, col_position, compare_value)
    }

    pub fn can_go_up(&self, row: usize, col: usize) -> bool {
        if row > 0 {
            return self.input_values[row - 1][col] < HIGH_POINT;
        }
        false
    }

    pub fn can_go_down(&self, row: usize, col: usize) -> bool {
        if row + 1 < self.input_values.len() {
            return self.input_values[row + 1][col] < HIGH_POINT;
        }
        false
    }

    pub fn can_go_left(&self, row: usize, col: usize) -> bool {
        if col > 0 {
            return self.input_values[row][col - 1] < HIGH_POINT;
        }
        false
    }

    pub fn can_go_right(&self, row: usize, col: usize) -> bool {
        if col + 1 < self.input_values[row].len() {
            return self.input_values[row][col + 1] < HIGH_POINT;
        }
        false
    }

    pub fn check_position_and_move(&mut self, row: usize, col: usize) -> u32 {
        let mut return_value: u32 = 1;
        self.visited_points.push((row, col));

        if self.can_go_up(row, col) {
            let new_row: usize = row - 1;
            let new_col: usize = col;
            if !self.visited_points.contains(&(new_row, new_col)) {
                return_value += self.check_position_and_move(new_row, new_col);
            }
        }

        if self.can_go_down(row, col) {
            let new_row: usize = row + 1;
            let new_col: usize = col;
            if !self.visited_points.contains(&(new_row, new_col)) {
                return_value += self.check_position_and_move(new_row, new_col);
            }
        }

        if self.can_go_left(row, col) {
            let new_row: usize = row;
            let new_col: usize = col - 1;
            if !self.visited_points.contains(&(new_row, new_col)) {
                return_value += self.check_position_and_move(new_row, new_col);
            }
        }

        if self.can_go_right(row, col) {
            let new_row: usize = row;
            let new_col: usize = col + 1;
            if !self.visited_points.contains(&(new_row, new_col)) {
                return_value += self.check_position_and_move(new_row, new_col);
            }
        }
        return_value
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

    pub fn calculate_three_largest_basin_sizes_product(&mut self) -> u32 {
        for row in 0..self.input_values.len() {
            for col in 0..self.input_values[row].len() {
                if self.is_low_point(row, col) {
                    let mut basin_size: u32 = 0;
                    self.visited_points = Vec::new();
                    basin_size += self.check_position_and_move(row, col);
                    self.basin_sizes.push(basin_size);
                }
            }
        }
        self.basin_sizes.sort_by(|a, b| b.cmp(a));
        self.basin_sizes.iter().take(3).product::<u32>()
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

    #[test]
    fn test_calculate_three_largest_basin_sizes_product() {
        let mut lava_tubes = LavaTubes::new();
        lava_tubes.read_lines_from_input_file("sample_input.txt");

        assert_eq!(
            lava_tubes.calculate_three_largest_basin_sizes_product(),
            1134
        );
    }
}
