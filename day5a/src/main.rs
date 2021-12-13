use ::cmilbert_aoc_day5a::HydrothermalMap;

fn main() {
    let mut hydrothermal_map = HydrothermalMap::new();
    hydrothermal_map.read_lines_from_input_file("input.txt");
    hydrothermal_map.populate_points_from_line_segments();

    println!(
        "Total intersections: {}",
        hydrothermal_map.calculate_total_intersects()
    );
}
