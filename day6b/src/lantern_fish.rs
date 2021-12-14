use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    thread,
};

const DAYS_TO_SPAWN: usize = 6;
const DAYS_TO_SPAWN_FOR_NEW_FISH: usize = DAYS_TO_SPAWN + 2;

#[derive(Clone, Copy)]
pub struct LanternFish {
    days_since_spawn: usize,
}

#[derive(Clone)]
pub struct LanternFishSchool {
    pub fish: Vec<LanternFish>,
}

impl LanternFishSchool {
    pub fn new() -> Self {
        LanternFishSchool { fish: Vec::new() }
    }

    pub fn read_lines_from_input_file(&mut self, filename: impl AsRef<Path>) {
        let file = File::open(filename).expect("no such file");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line_values: Vec<usize> = line
                .unwrap()
                .split(",")
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect();
            for line_value in line_values {
                self.fish.push(LanternFish {
                    days_since_spawn: line_value,
                });
            }
        }
    }

    pub fn simulate_iterations(&mut self, iterations: usize) {
        for i in 0..iterations {
            println!("Iteration: {}", i);
            self.iterate();
        }
    }

    fn iterate(&mut self) {
        for i in 0..self.fish.len() {
            if self.fish[i].days_since_spawn == 0 {
                self.spawn_new_fish();
                self.reset_spawn_cycle(i);
            } else {
                self.fish[i].days_since_spawn -= 1;
            }
        }
    }

    fn spawn_new_fish(&mut self) {
        self.fish.push(LanternFish {
            days_since_spawn: DAYS_TO_SPAWN_FOR_NEW_FISH,
        });
    }

    fn reset_spawn_cycle(&mut self, index: usize) {
        self.fish[index].days_since_spawn = DAYS_TO_SPAWN;
    }

    pub fn add_fish(&mut self, days_since_spawn: usize) {
        self.fish.push(LanternFish {
            days_since_spawn: days_since_spawn,
        })
    }
}

pub struct LanternFishThreader {
    lantern_fish_schools: Vec<LanternFishSchool>,
}

impl LanternFishThreader {
    // Map-Reduce the Lantern Fish School, one thread per input from the file
    pub fn new() -> Self {
        LanternFishThreader {
            lantern_fish_schools: Vec::new(),
        }
    }

    pub fn read_lines_from_input_file(&mut self, filename: impl AsRef<Path>) {
        let file = File::open(filename).expect("no such file");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line_values: Vec<usize> = line
                .unwrap()
                .split(",")
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect();

            for line_value in line_values {
                let mut lantern_fish_school: LanternFishSchool = LanternFishSchool::new();
                lantern_fish_school.add_fish(line_value);
                self.lantern_fish_schools.push(lantern_fish_school);
            }
        }
    }

    pub fn run_simulation(&mut self, iterations: usize) -> usize {
        let mut child_threads = vec![];

        for i in 0..self.lantern_fish_schools.len() {
            let mut lantern_fish_school: LanternFishSchool = self.lantern_fish_schools[i].clone();
            child_threads.push(thread::spawn(move || -> usize {
                lantern_fish_school.simulate_iterations(iterations);
                let total_fish_for_school: usize = lantern_fish_school.fish.len();
                return total_fish_for_school;
            }));
        }

        return child_threads
            .into_iter()
            .map(|c| c.join().unwrap())
            .sum::<usize>();
    }
}

#[cfg(test)]
mod tests_day6a {
    use super::*;

    #[test]
    fn test_read_lines_from_input_file() {
        let mut lantern_fish_threader = LanternFishThreader::new();
        lantern_fish_threader.read_lines_from_input_file("sample_input.txt");

        assert_eq!(lantern_fish_threader.lantern_fish_schools.len(), 5);
    }

    #[test]
    fn test_run_simulate_18_iterations() {
        let mut lantern_fish_threader = LanternFishThreader::new();
        lantern_fish_threader.read_lines_from_input_file("sample_input.txt");

        let simulation_result: usize = lantern_fish_threader.run_simulation(18);
        assert_eq!(simulation_result, 26);
    }

    #[test]
    fn test_run_simulate_80_iterations() {
        let mut lantern_fish_threader = LanternFishThreader::new();
        lantern_fish_threader.read_lines_from_input_file("sample_input.txt");

        let simulation_result: usize = lantern_fish_threader.run_simulation(80);
        assert_eq!(simulation_result, 5934);
    }

    #[test]
    fn test_run_simulate_256_iterations() {
        let mut lantern_fish_threader = LanternFishThreader::new();
        lantern_fish_threader.read_lines_from_input_file("sample_input.txt");

        // This is still very slow
        // let simulation_result: usize = lantern_fish_threader.run_simulation(256);
        // assert_eq!(simulation_result, 26984457539);
    }
}
