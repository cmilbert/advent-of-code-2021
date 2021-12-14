use ::cmilbert_aoc_day6b2::LanternFishSchool;

fn main() {
    let mut lantern_fish_school = LanternFishSchool::new();
    lantern_fish_school.read_lines_from_input_file("input.txt");

    lantern_fish_school.simulate_iterations(256);
    println!("Total fish: {}", lantern_fish_school.get_total_fish());
}
