use ::cmilbert_aoc_day6b::LanternFishThreader;

fn main() {
    let mut lantern_fish_threader = LanternFishThreader::new();
    lantern_fish_threader.read_lines_from_input_file("input.txt");

    let simulation_result: usize = lantern_fish_threader.run_simulation(256);
    println!("Total fish: {}", simulation_result);
}
