use ::cmilbert_aoc_day9a::LavaTubes;

fn main() {
    let mut lava_tubes = LavaTubes::new();
    lava_tubes.read_lines_from_input_file("input.txt");
    println!("Risk level: {}", lava_tubes.calculate_risk_level());
}
