use ::cmilbert_aoc_day8a::SevenSegment;

fn main() {
    let mut seven_segment = SevenSegment::new();
    seven_segment.read_lines_from_input_file("input.txt");

    println!(
        "Count of 1, 4, 7, 8: {}",
        seven_segment.count_1_4_7_8_output_values()
    );
}
