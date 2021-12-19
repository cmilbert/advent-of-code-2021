use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

pub struct NavigationSubsystem {
    pub input_values: Vec<String>,
}

impl Default for NavigationSubsystem {
    fn default() -> Self {
        Self::new()
    }
}

impl NavigationSubsystem {
    pub fn new() -> Self {
        NavigationSubsystem {
            input_values: Vec::new(),
        }
    }

    pub fn read_lines_from_input_file(&mut self, filename: impl AsRef<Path>) {
        let file = File::open(filename).expect("no such file");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            self.input_values.push(line.unwrap());
        }
    }

    fn is_left_hand_bracket(&self, bracket: char) -> bool {
        if bracket == '{' || bracket == '[' || bracket == '(' || bracket == '<' {
            return true;
        }
        false
    }

    fn is_right_hand_bracket(&self, bracket: char) -> bool {
        if bracket == '}' || bracket == ']' || bracket == ')' || bracket == '>' {
            return true;
        }
        false
    }

    fn get_bracket_type(&self, bracket: char) -> &str {
        if bracket == '(' || bracket == ')' {
            return "parenthesis";
        } else if bracket == '[' || bracket == ']' {
            return "square";
        } else if bracket == '{' || bracket == '}' {
            return "curly";
        } else if bracket == '<' || bracket == '>' {
            return "angle";
        }
        "unknown"
    }

    fn same_bracket_type(&self, left_bracket: char, right_bracket: char) -> bool {
        if self.get_bracket_type(left_bracket) == self.get_bracket_type(right_bracket) {
            return true;
        }
        false
    }

    pub fn is_chunk_valid(&self, chunk: &str) -> (bool, char) {
        let mut bracket_stack: Vec<char> = Vec::new();

        for chunk_character in chunk.chars() {
            if self.is_left_hand_bracket(chunk_character) {
                bracket_stack.push(chunk_character);
            } else if self.is_right_hand_bracket(chunk_character) {
                let popped_bracket: char = bracket_stack.pop().unwrap();
                if !self.same_bracket_type(chunk_character, popped_bracket) {
                    return (false, chunk_character);
                }
            }
        }
        (true, ' ')
    }

    fn get_points_for_invalid_bracket(&self, invalid_bracket: char) -> u32 {
        if invalid_bracket == ')' {
            return 3;
        } else if invalid_bracket == ']' {
            return 57;
        } else if invalid_bracket == '}' {
            return 1197;
        } else if invalid_bracket == '>' {
            return 25137;
        }
        0
    }

    pub fn calculate_score_for_invalid_brackets_in_chunks(&self, chunks: &[String]) -> u32 {
        let mut invalid_score: u32 = 0;
        for chunk in chunks {
            let is_chunk_valid = self.is_chunk_valid(chunk);

            if !is_chunk_valid.0 {
                invalid_score += self.get_points_for_invalid_bracket(is_chunk_valid.1);
            }
        }

        invalid_score
    }

    pub fn calculate_score_for_invalid_brackets(&self) -> u32 {
        self.calculate_score_for_invalid_brackets_in_chunks(&self.input_values)
    }
}

#[cfg(test)]
mod tests_day10a {
    use super::*;

    #[test]
    fn test_read_lines_from_input_file() {
        let mut navigation_subsystem = NavigationSubsystem::new();
        navigation_subsystem.read_lines_from_input_file("sample_input.txt");

        assert_eq!(navigation_subsystem.input_values.len(), 10);
        assert_eq!(navigation_subsystem.input_values.get(0).unwrap().len(), 24);
    }

    #[test]
    fn test_simple_valid_chunks() {
        let navigation_subsystem = NavigationSubsystem::new();

        let mut simple_chunks: Vec<String> = Vec::new();
        simple_chunks.push("([])".to_string());
        simple_chunks.push("{()()()}".to_string());
        simple_chunks.push("<([{}])>".to_string());
        simple_chunks.push("(((((((((())))))))))".to_string());
        simple_chunks.push("[<>({}){}[([])<>]]".to_string());

        for chunk in simple_chunks {
            println!("Checking chunk {}", chunk);
            assert_eq!(navigation_subsystem.is_chunk_valid(&chunk).0, true);
        }
    }

    #[test]
    fn test_simple_invalid_chunks() {
        let navigation_subsystem = NavigationSubsystem::new();

        let mut simple_chunks: Vec<String> = Vec::new();
        simple_chunks.push("{([(<{}[<>[]}>{[]{[(<()>".to_string());
        simple_chunks.push("[[<[([]))<([[{}[[()]]]".to_string());
        simple_chunks.push("[{[{({}]{}}([{[{{{}}([]".to_string());
        simple_chunks.push("[<(<(<(<{}))><([]([]()".to_string());
        simple_chunks.push("<{([([[(<>()){}]>(<<{{".to_string());

        for chunk in simple_chunks {
            assert_eq!(navigation_subsystem.is_chunk_valid(&chunk).0, false);
        }
    }

    #[test]
    fn test_calculate_score_for_invalid_brackets() {
        let navigation_subsystem = NavigationSubsystem::new();

        let mut simple_chunks: Vec<String> = Vec::new();
        simple_chunks.push("{([(<{}[<>[]}>{[]{[(<()>".to_string());
        simple_chunks.push("[[<[([]))<([[{}[[()]]]".to_string());
        simple_chunks.push("[{[{({}]{}}([{[{{{}}([]".to_string());
        simple_chunks.push("[<(<(<(<{}))><([]([]()".to_string());
        simple_chunks.push("<{([([[(<>()){}]>(<<{{".to_string());

        assert_eq!(
            navigation_subsystem.calculate_score_for_invalid_brackets_in_chunks(&simple_chunks),
            26397
        );
    }

    #[test]
    fn test_calculate_score_for_invalid_brackets_in_sample_input() {
        let mut navigation_subsystem = NavigationSubsystem::new();
        navigation_subsystem.read_lines_from_input_file("sample_input.txt");

        assert_eq!(
            navigation_subsystem.calculate_score_for_invalid_brackets(),
            26397
        );
    }
}
