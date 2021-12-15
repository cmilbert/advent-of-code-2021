use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

type Crab = usize;

pub struct CrabArmy {
    pub crabs: Vec<Crab>,
}

impl CrabArmy {
    pub fn new() -> Self {
        CrabArmy { crabs: Vec::new() }
    }

    pub fn read_lines_from_input_file(&mut self, filename: impl AsRef<Path>) {
        let file = File::open(filename).expect("no such file");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            self.crabs = line
                .unwrap()
                .split(",")
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect();
        }
    }

    pub fn average(&self) -> f32 {
        return self.crabs.iter().sum::<usize>() as f32 / self.crabs.len() as f32;
    }

    pub fn median(&mut self) -> f32 {
        self.crabs.sort();
        return self.crabs[self.crabs.len() / 2] as f32;
    }

    pub fn calculate_fuel_usage(&mut self) -> usize {
        let median: f32 = self.median();
        let mut fuel_usage: usize = 0;

        for i in 0..self.crabs.len() {
            fuel_usage += (self.crabs[i] as f32 - median).abs() as usize;
        }

        return fuel_usage;
    }
}

#[cfg(test)]
mod tests_day6a {
    use super::*;

    #[test]
    fn test_read_lines_from_input_file() {
        let mut crab_army = CrabArmy::new();
        crab_army.read_lines_from_input_file("sample_input.txt");

        assert_eq!(crab_army.crabs.len(), 10);
    }

    #[test]
    fn test_calculate_average() {
        let mut crab_army = CrabArmy::new();
        crab_army.read_lines_from_input_file("sample_input.txt");

        assert_eq!(crab_army.average(), 4.9);
    }

    #[test]
    fn test_calculate_fuel_usage() {
        let mut crab_army = CrabArmy::new();
        crab_army.read_lines_from_input_file("sample_input.txt");

        assert_eq!(crab_army.calculate_fuel_usage(), 37);
    }
}
