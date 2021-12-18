use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

struct HydrothermalLineSegment {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

pub struct HydrothermalMap {
    line_segments: Vec<HydrothermalLineSegment>,
    point_map: Vec<Vec<usize>>,
}

impl Default for HydrothermalMap {
    fn default() -> Self {
        Self::new()
    }
}

impl HydrothermalMap {
    pub fn new() -> Self {
        HydrothermalMap {
            line_segments: Vec::new(),
            point_map: Vec::new(),
        }
    }

    pub fn read_lines_from_input_file(&mut self, filename: impl AsRef<Path>) {
        let file = File::open(filename).expect("no such file");
        let reader = BufReader::new(file);
        let mut max_x_size: usize = 0;
        let mut max_y_size: usize = 0;

        for line in reader.lines() {
            let line: String = line.unwrap();
            let parse_result: Option<HydrothermalLineSegment> = self.parse_line_segment(line);
            if let Some(..) = parse_result {
                let unwrapped_line_segment: HydrothermalLineSegment = parse_result.unwrap();
                if unwrapped_line_segment.x1 > max_x_size {
                    max_x_size = unwrapped_line_segment.x1;
                }
                if unwrapped_line_segment.x2 > max_x_size {
                    max_x_size = unwrapped_line_segment.x2;
                }
                if unwrapped_line_segment.y1 > max_y_size {
                    max_y_size = unwrapped_line_segment.y1;
                }
                if unwrapped_line_segment.y2 > max_y_size {
                    max_y_size = unwrapped_line_segment.y2;
                }
                self.line_segments.push(unwrapped_line_segment);
            }
        }
        self.point_map = vec![vec![0; max_x_size + 1]; max_y_size + 1]
    }

    fn parse_line_segment(&self, line: String) -> Option<HydrothermalLineSegment> {
        let modified_line: String = line.replace(" -> ", ","); // Replace arrow with comma
        let line_values: Vec<usize> = modified_line
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();

        let x1: usize = line_values[0];
        let y1: usize = line_values[1];
        let x2: usize = line_values[2];
        let y2: usize = line_values[3];

        if x1 == x2 || y1 == y2 {
            let new_segment = HydrothermalLineSegment { x1, y1, x2, y2 };
            return Some(new_segment);
        }

        None
    }

    pub fn populate_points_from_line_segments(&mut self) {
        for line_segment in &self.line_segments {
            let x1: usize = line_segment.x1;
            let y1: usize = line_segment.y1;
            let x2: usize = line_segment.x2;
            let y2: usize = line_segment.y2;

            if y1 == y2 {
                // Populating horizontal points
                if x1 < x2 {
                    for x in x1..=x2 {
                        self.point_map[y1][x] += 1;
                    }
                } else {
                    for x in x2..=x1 {
                        self.point_map[y1][x] += 1;
                    }
                }
            }
            if x1 == x2 {
                // Populating vertical points
                if y1 < y2 {
                    for y in y1..=y2 {
                        self.point_map[y][x1] += 1;
                    }
                } else {
                    for y in y2..=y1 {
                        self.point_map[y][x1] += 1;
                    }
                }
            }
        }
    }

    pub fn calculate_total_intersects(&self) -> usize {
        let mut total_intersects: usize = 0;

        for i in 0..self.point_map.len() {
            for j in 0..self.point_map[0].len() {
                let point: usize = self.point_map[i][j];
                if point > 1 {
                    total_intersects += 1;
                }
            }
        }

        total_intersects
    }

    pub fn print_hydorthermal_map(&self) {
        for y in 0..self.point_map.len() {
            for x in 0..self.point_map[y].len() {
                let point: usize = self.point_map[y][x];
                if point == 0 {
                    print!(". ");
                } else {
                    print!("{} ", point);
                }
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests_day5a {
    use super::*;

    #[test]
    fn test_read_line_segments_from_input_file() {
        let mut hydrothermal_map = HydrothermalMap::new();
        hydrothermal_map.read_lines_from_input_file("sample_input.txt");

        assert_eq!(hydrothermal_map.line_segments.len(), 6);
    }

    #[test]
    fn test_calculate_intersections() {
        let mut hydrothermal_map = HydrothermalMap::new();
        hydrothermal_map.read_lines_from_input_file("sample_input.txt");
        hydrothermal_map.populate_points_from_line_segments();

        hydrothermal_map.print_hydorthermal_map();
        assert_eq!(hydrothermal_map.calculate_total_intersects(), 5);
    }
}
