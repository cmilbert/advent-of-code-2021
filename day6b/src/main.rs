use ::cmilbert_aoc_day6b::LanternFishSchool;

fn main() {
    let mut lantern_fish_school = LanternFishSchool::new();
    lantern_fish_school.read_lines_from_input_file("input.txt");
    lantern_fish_school.simulate_iterations(256);

    println!("Total fish: {}", lantern_fish_school.fish.len());
}
