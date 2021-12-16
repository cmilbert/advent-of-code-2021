use ::cmilbert_aoc_day7b::CrabArmy;

fn main() {
    let mut crab_army = CrabArmy::new();
    crab_army.read_lines_from_input_file("input.txt");

    println!("Fuel usage: {}", crab_army.calculate_fuel_usage());
}
