use ::cmilbert_aoc_day4b::BingoGame;

fn main() {
    let mut bingo_game = BingoGame::new();
    bingo_game.read_lines_from_input_file("input.txt");

    let winning_value: usize = bingo_game.play_until_last_winner();
    println!("Winning value: {}", winning_value);
}
