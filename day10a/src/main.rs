use ::cmilbert_aoc_day10a::NavigationSubsystem;

fn main() {
    let mut navigation_subsystem = NavigationSubsystem::new();
    navigation_subsystem.read_lines_from_input_file("input.txt");
    println!(
        "Syntax error score: {}",
        navigation_subsystem.calculate_score_for_invalid_brackets()
    );
}
