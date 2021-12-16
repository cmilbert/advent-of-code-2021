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

    pub fn calculate_fuel_usage(&mut self) -> usize {
        self.crabs.sort();
        let minimum_position: usize = self.crabs[0];
        let maximum_position: usize = self.crabs[self.crabs.len() - 1];
        let mut minimum_fuel_cost: usize = 0;

        for position in minimum_position..=maximum_position {
            let fuel_cost_for_point = self.calculate_fuel_usage_for_point(position);
            if fuel_cost_for_point < minimum_fuel_cost || minimum_fuel_cost == 0 {
                minimum_fuel_cost = fuel_cost_for_point;
            }
        }

        return minimum_fuel_cost;
    }

    fn calculate_fuel_usage_for_point(&self, position: usize) -> usize {
        let mut fuel_usage: usize = 0;

        for i in 0..self.crabs.len() {
            let positions_moved: usize = (self.crabs[i] as f32 - position as f32).abs() as usize;
            let fuel_cost_for_move: usize = (0..=positions_moved).fold(0, |a, b| a + b);
            fuel_usage += fuel_cost_for_move;
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
    fn test_calculate_fuel_usage() {
        let mut crab_army = CrabArmy::new();
        crab_army.read_lines_from_input_file("sample_input.txt");

        assert_eq!(crab_army.calculate_fuel_usage(), 168);
    }
}
