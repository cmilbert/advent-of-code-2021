use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

const BINGO_BOARD_SIZE: usize = 5;

pub struct BingoCell {
    number_in_cell: usize,
    number_called: bool,
}

type BingoBoard = Vec<Vec<BingoCell>>;

pub struct BingoGame {
    draws: Vec<usize>,
    boards: Vec<BingoBoard>,
}

impl Default for BingoGame {
    fn default() -> Self {
        Self::new()
    }
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

        while !file_lines.is_empty() {
            file_lines.drain(0..1); // Drain new line before board
            let mut new_board: Vec<Vec<BingoCell>> = Vec::new();

            // For each of 5 rows create a new board row
            for line in file_lines.iter().take(5) {
                let new_row: Vec<BingoCell> = self.parse_bingo_board_line(line.to_string());
                new_board.push(new_row);
            }
            file_lines.drain(0..5); // Remove 6 lines
            self.boards.push(new_board);
        }
    }

    fn parse_draws(&self, draw_line: String) -> Vec<usize> {
        return draw_line
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();
    }

    fn parse_bingo_board_line(&self, bingo_board_line: String) -> Vec<BingoCell> {
        let mut return_vector: Vec<BingoCell> = Vec::new();
        let splits = bingo_board_line.split_whitespace();
        for split in splits {
            let parsed_value: usize = split.parse().unwrap();
            let bingo_cell = BingoCell {
                number_in_cell: parsed_value,
                number_called: false,
            };
            return_vector.push(bingo_cell);
        }
        return_vector
    }

    fn process_draw(&mut self, number_drawn: usize) {
        for bingo_board in self.boards.iter_mut() {
            for bingo_board_row in bingo_board.iter_mut() {
                for bingo_board_cell in bingo_board_row.iter_mut() {
                    if bingo_board_cell.number_in_cell == number_drawn {
                        bingo_board_cell.number_called = true;
                    }
                }
            }
        }
    }

    pub fn get_winning_board_positions(&self) -> Vec<usize> {
        let mut board_positions: Vec<usize> = Vec::new();
        for (board_position, board) in self.boards.iter().enumerate() {
            let row_win = self.check_board_rows(board);
            let column_win = self.check_board_columns(board);
            if row_win || column_win {
                board_positions.push(board_position);
            }
        }
        board_positions
    }

    fn check_board_rows(&self, bingo_board: &[Vec<BingoCell>]) -> bool {
        for row in bingo_board {
            let mut row_count: usize = 0;
            for cell in row {
                if cell.number_called {
                    row_count += 1;
                }
            }
            if row_count == BINGO_BOARD_SIZE {
                return true;
            }
        }
        false
    }

    fn check_board_columns(&self, bingo_board: &[Vec<BingoCell>]) -> bool {
        let mut column_counts: Vec<usize> = vec![0; bingo_board.len()];
        for i in 0..bingo_board.len() {
            for cell in bingo_board.iter().take(bingo_board[i].len()) {
                if cell[i].number_called {
                    column_counts[i] += 1;
                }
            }
        }

        for column_count in &column_counts {
            if column_count == &BINGO_BOARD_SIZE {
                return true;
            }
        }
        false
    }

    pub fn unmarked_cell_sum(&self, bingo_board: &[Vec<BingoCell>]) -> usize {
        let mut unmarked_sum: usize = 0;
        for row in bingo_board {
            for cell in row {
                if !cell.number_called {
                    unmarked_sum += cell.number_in_cell;
                }
            }
        }
        unmarked_sum
    }

    pub fn play_until_last_winner(&mut self) -> usize {
        let mut last_winning_value: usize = 0;

        for draw_position in 0..self.draws.len() {
            let number_drawn: usize = self.draws[draw_position];
            self.process_draw(number_drawn);

            if draw_position > BINGO_BOARD_SIZE {
                let mut round_winning_board_positions = self.get_winning_board_positions();
                if !round_winning_board_positions.is_empty() {
                    let last_index: usize = *round_winning_board_positions.last().unwrap();
                    let unmarked_cell_sum: usize =
                        self.unmarked_cell_sum(self.boards.get(last_index).unwrap());
                    last_winning_value = unmarked_cell_sum * number_drawn;
                    round_winning_board_positions.reverse();
                    for round_winning_board_position in &round_winning_board_positions {
                        self.boards.remove(*round_winning_board_position);
                    }
                }
            }
        }

        last_winning_value
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
            let number_drawn: usize = bingo_game.draws[i];
            bingo_game.process_draw(number_drawn)
        }

        let bingo_board: &BingoBoard = bingo_game.boards.get(0).unwrap();
        assert_eq!(bingo_board.len(), BINGO_BOARD_SIZE);
        assert_eq!(bingo_board[0][3].number_called, true);
        assert_eq!(bingo_board[1][3].number_called, true);
        assert_eq!(bingo_board[2][3].number_called, false);
        assert_eq!(bingo_board[3][3].number_called, false);
        assert_eq!(bingo_board[4][3].number_called, false);
    }

    #[test]
    fn test_run_game() {
        let mut bingo_game = BingoGame::new();
        bingo_game.read_lines_from_input_file("sample_input.txt");

        let winning_value: usize = bingo_game.play_until_last_winner();
        assert_eq!(winning_value, 1924);
    }
}
