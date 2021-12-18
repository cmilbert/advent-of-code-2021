use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

const DAYS_TO_SPAWN: usize = 7;
const DAYS_TO_SPAWN_FOR_NEW_FISH: usize = DAYS_TO_SPAWN + 2;

pub struct LanternFishSchool {
    pub spawning_buckets: Vec<usize>,
}

impl Default for LanternFishSchool {
    fn default() -> Self {
        Self::new()
    }
}

impl LanternFishSchool {
    pub fn new() -> Self {
        LanternFishSchool {
            spawning_buckets: vec![0; DAYS_TO_SPAWN_FOR_NEW_FISH],
        }
    }

    pub fn read_lines_from_input_file(&mut self, filename: impl AsRef<Path>) {
        let file = File::open(filename).expect("no such file");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line_values: Vec<usize> = line
                .unwrap()
                .split(',')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect();
            for line_value in line_values {
                self.spawning_buckets[line_value] += 1;
            }
        }
    }

    pub fn simulate_iterations(&mut self, iterations: usize) {
        for _ in 0..iterations {
            self.iterate();
        }
    }

    fn iterate(&mut self) {
        let zero_value: usize = self.spawning_buckets[0];

        for i in 1..DAYS_TO_SPAWN_FOR_NEW_FISH {
            self.spawning_buckets[i - 1] = self.spawning_buckets[i];
        }

        self.spawning_buckets[DAYS_TO_SPAWN - 1] += zero_value;
        self.spawning_buckets[DAYS_TO_SPAWN_FOR_NEW_FISH - 1] = zero_value;
    }

    pub fn get_total_fish(&self) -> usize {
        let sum: usize = self.spawning_buckets.iter().sum();
        sum
    }
}

#[cfg(test)]
mod tests_day6b2 {
    use super::*;

    #[test]
    fn test_read_lines_from_input_file() {
        let mut lantern_fish_school = LanternFishSchool::new();
        lantern_fish_school.read_lines_from_input_file("sample_input.txt");

        assert_eq!(lantern_fish_school.get_total_fish(), 5);
    }

    #[test]
    fn test_run_simulate_18_iterations() {
        let mut lantern_fish_school = LanternFishSchool::new();
        lantern_fish_school.read_lines_from_input_file("sample_input.txt");

        lantern_fish_school.simulate_iterations(18);
        assert_eq!(lantern_fish_school.get_total_fish(), 26);
    }

    #[test]
    fn test_run_simulate_80_iterations() {
        let mut lantern_fish_school = LanternFishSchool::new();
        lantern_fish_school.read_lines_from_input_file("sample_input.txt");

        lantern_fish_school.simulate_iterations(80);
        assert_eq!(lantern_fish_school.get_total_fish(), 5934);
    }

    #[test]
    fn test_run_simulate_256_iterations() {
        let mut lantern_fish_school = LanternFishSchool::new();
        lantern_fish_school.read_lines_from_input_file("sample_input.txt");

        lantern_fish_school.simulate_iterations(256);
        assert_eq!(lantern_fish_school.get_total_fish(), 26984457539);
    }
}
