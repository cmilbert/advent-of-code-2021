use ::cmilbert_aoc_day8b::SevenSegment;

fn main() {
    let mut seven_segment = SevenSegment::new();
    seven_segment.read_lines_from_input_file("input.txt");
    println!(
        "Sum of output values: {}",
        seven_segment.decode_signals_and_sum_output_values()
    );
}
