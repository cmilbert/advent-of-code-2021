use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

struct BingoCell {
    number_in_cell: isize,
    number_called: bool,
}

type BingoBoard = Vec<Vec<BingoCell>>;

pub struct BingoGame {
    draws: Vec<isize>,
    boards: Vec<BingoBoard>,
}

impl BingoGame {
    pub fn new() -> Self {
        BingoGame {
            draws: Vec::new(),
            boards: Vec::new(),
        }
    }

    pub fn read_lines_from_input_file(&mut self, filename: impl AsRef<Path>) {
        let file = File::open(filename).expect("no such file");
        let reader = BufReader::new(file);
        let mut file_lines: Vec<String> = Vec::new();

        for (_, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            file_lines.push(line);
        }
        self.draws = self.parse_draws(file_lines[0].to_string());
        file_lines.drain(0..1);

        while file_lines.len() > 0 {
            file_lines.drain(0..1); // Drain new line before board
            let mut new_board: Vec<Vec<BingoCell>> = Vec::new();

            // For each of 5 rows create a new board row
            for i in 0..5 {
                let new_row: Vec<BingoCell> =
                    self.parse_bingo_board_line(file_lines[i].to_string());
                new_board.push(new_row);
            }
            file_lines.drain(0..5); // Remove 6 lines
            self.boards.push(new_board);
        }
    }

    fn parse_draws(&self, draw_line: String) -> Vec<isize> {
        return draw_line
            .split(",")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();
    }

    fn parse_bingo_board_line(&self, bingo_board_line: String) -> Vec<BingoCell> {
        let mut return_vector: Vec<BingoCell> = Vec::new();
        let splits = bingo_board_line.split_whitespace();
        for split in splits {
            let parsed_value: isize = split.parse().unwrap();
            let bingo_cell = BingoCell {
                number_in_cell: parsed_value,
                number_called: false,
            };
            return_vector.push(bingo_cell);
        }
        return return_vector;
    }

    fn process_draw(&mut self, number_drawn: &isize) {
        for bingo_board in self.boards.iter_mut() {
            for bingo_board_row in bingo_board.iter_mut() {
                for bingo_board_cell in bingo_board_row.iter_mut() {
                    if bingo_board_cell.number_in_cell == *number_drawn {
                        bingo_board_cell.number_called = true;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests_day4a {
    use super::*;

    #[test]
    fn test_read_draws_from_input_file() {
        let mut bingo_game = BingoGame::new();
        bingo_game.read_lines_from_input_file("sample_input.txt");

        assert_eq!(bingo_game.draws.len(), 27);
    }

    #[test]
    fn test_read_boards_from_input_file() {
        let mut bingo_game = BingoGame::new();
        bingo_game.read_lines_from_input_file("sample_input.txt");

        assert_eq!(bingo_game.boards.len(), 3);
    }

    #[test]
    fn test_mark_bingo_cells_from_draws() {
        let mut bingo_game = BingoGame::new();
        bingo_game.read_lines_from_input_file("sample_input.txt");

        for i in 0..5 {
            let number_drawn: isize = bingo_game.draws[i];
            bingo_game.process_draw(&number_drawn)
        }

        let bingo_board: &BingoBoard = bingo_game.boards.get(0).unwrap();
        assert_eq!(bingo_board[0][3].number_called, true);
        assert_eq!(bingo_board[1][3].number_called, true);
        assert_eq!(bingo_board[2][3].number_called, false);
        assert_eq!(bingo_board[3][3].number_called, false);
        assert_eq!(bingo_board[4][3].number_called, false);
    }
}
