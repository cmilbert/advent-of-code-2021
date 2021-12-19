use ::cmilbert_aoc_day9b::LavaTubes;

fn main() {
    let mut lava_tubes = LavaTubes::new();
    lava_tubes.read_lines_from_input_file("input.txt");
    println!(
        "Three largest basin products: {}",
        lava_tubes.calculate_three_largest_basin_sizes_product()
    );
}
