use std::{
    collections::HashMap,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

pub struct SevenSegment {
    pub input_values: Vec<Vec<String>>,
    pub output_values: Vec<Vec<String>>,
    pub digit_mapping: HashMap<String, char>,
}

impl Default for SevenSegment {
    fn default() -> Self {
        Self::new()
    }
}

impl SevenSegment {
    pub fn new() -> Self {
        SevenSegment {
            input_values: Vec::new(),
            output_values: Vec::new(),
            digit_mapping: HashMap::new(),
        }
    }

    pub fn read_lines_from_input_file(&mut self, filename: impl AsRef<Path>) {
        let file = File::open(filename).expect("no such file");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            // Output values: splt on pipe, split right hand side on space
            // Input values: split on pipe, split left hand side on space
            let line_input_output_strings: Vec<String> = line
                .unwrap()
                .split('|')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect();

            let line_input_values: Vec<String> = line_input_output_strings[0]
                .split(' ')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect();
            self.input_values.push(line_input_values);

            let line_output_values: Vec<String> = line_input_output_strings[1]
                .split(' ')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect();
            self.output_values.push(line_output_values);
        }
    }

    fn sort_string(&self, input_string: String) -> String {
        let mut sorted_chars: Vec<char> = input_string.chars().collect();
        sorted_chars.sort_unstable();
        String::from_iter(sorted_chars)
    }

    fn contains_all_characters(&self, check_string: &str, check_characters: Vec<char>) -> bool {
        let mut contains_all_characters: bool = true;

        for character in check_characters {
            if !check_string.contains(character) {
                contains_all_characters = false;
            }
        }

        contains_all_characters
    }

    fn find_digit_key_for_value(&self, value: char) -> Option<&String> {
        self.digit_mapping
            .iter()
            .find_map(|(key, &val)| if val == value { Some(key) } else { None })
    }

    fn find_and_store_digit_pattern(
        &mut self,
        values_to_decode: &[String],
        digit_length: usize,
        digit_character: char,
    ) {
        for value in values_to_decode {
            if value.len() == digit_length {
                let sorted_value = self.sort_string(value.to_string());

                self.digit_mapping
                    .insert(sorted_value.to_string(), digit_character);
            }
        }
    }

    fn find_eight_digit_pattern(&mut self, values_to_decode: &[String]) {
        self.find_and_store_digit_pattern(values_to_decode, 7, '8');
    }

    fn find_seven_digit_pattern(&mut self, values_to_decode: &[String]) {
        self.find_and_store_digit_pattern(values_to_decode, 3, '7')
    }

    fn find_four_digit_pattern(&mut self, values_to_decode: &[String]) {
        self.find_and_store_digit_pattern(values_to_decode, 4, '4')
    }

    fn find_one_digit_pattern(&mut self, values_to_decode: &[String]) {
        self.find_and_store_digit_pattern(values_to_decode, 2, '1')
    }

    // 5 character digits
    fn find_three_digit_pattern(&mut self, values_to_decode: &[String]) {
        // Has 5 charactesr and has all the characters of 1 (use substr after sorting)
        for value in values_to_decode {
            if value.len() == 5 {
                let sorted_value = self.sort_string(value.to_string());

                let one_characters = self
                    .find_digit_key_for_value('1')
                    .unwrap()
                    .chars()
                    .collect::<Vec<char>>();
                if self.contains_all_characters(&sorted_value, one_characters) {
                    self.digit_mapping.insert(sorted_value.to_string(), '3');
                }
            }
        }
    }

    fn find_five_digit_patthern(&mut self, values_to_decode: &[String]) {
        // 5 characters long and whats left of the chracters from 4 when 1 is removed
        for value in values_to_decode {
            if value.len() == 5 {
                let sorted_value = self.sort_string(value.to_string());

                let one_characters = self
                    .find_digit_key_for_value('1')
                    .unwrap()
                    .chars()
                    .collect::<Vec<char>>();
                let mut four_minus_one_characters = self
                    .find_digit_key_for_value('4')
                    .unwrap()
                    .chars()
                    .collect::<Vec<char>>();

                for character in one_characters {
                    four_minus_one_characters.retain(|&x| x != character);
                }

                if self.contains_all_characters(&sorted_value, four_minus_one_characters) {
                    self.digit_mapping.insert(sorted_value.to_string(), '5');
                }
            }
        }
    }

    fn find_two_digit_pattern(&mut self, values_to_decode: &[String]) {
        // 2 is 5 characters, but does not contain 3 or 5
        for value in values_to_decode {
            if value.len() == 5 {
                let sorted_value = self.sort_string(value.to_string());

                let three_characters = self
                    .find_digit_key_for_value('3')
                    .unwrap()
                    .chars()
                    .collect::<Vec<char>>();
                let five_characters = self
                    .find_digit_key_for_value('5')
                    .unwrap()
                    .chars()
                    .collect::<Vec<char>>();
                if !self.contains_all_characters(&sorted_value, three_characters)
                    && !self.contains_all_characters(&sorted_value, five_characters)
                {
                    self.digit_mapping.insert(sorted_value.to_string(), '2');
                }
            }
        }
    }

    // 6 character digits
    fn find_nine_digit_pattern(&mut self, values_to_decode: &[String]) {
        // 9 is 6 digits and contains all the characters of 3
        for value in values_to_decode {
            if value.len() == 6 {
                let sorted_value = self.sort_string(value.to_string());
                let three_characters = self
                    .find_digit_key_for_value('3')
                    .unwrap()
                    .chars()
                    .collect::<Vec<char>>();

                if self.contains_all_characters(&sorted_value, three_characters) {
                    self.digit_mapping.insert(sorted_value.to_string(), '9');
                }
            }
        }
    }

    fn find_six_digit_pattern(&mut self, values_to_decode: &[String]) {
        // 6 is 6 digits and does not contain all the characters of 1
        for value in values_to_decode {
            if value.len() == 6 {
                let sorted_value = self.sort_string(value.to_string());

                let one_characters = self
                    .find_digit_key_for_value('1')
                    .unwrap()
                    .chars()
                    .collect::<Vec<char>>();
                if !self.contains_all_characters(&sorted_value, one_characters) {
                    self.digit_mapping.insert(sorted_value.to_string(), '6');
                }
            }
        }
    }

    fn find_zero_digit_pattern(&mut self, values_to_decode: &[String]) {
        // 0 is length 6 and does not contain all the chracters of 9 or 6
        for value in values_to_decode {
            if value.len() == 6 {
                let sorted_value = self.sort_string(value.to_string());
                let six_characters = self
                    .find_digit_key_for_value('6')
                    .unwrap()
                    .chars()
                    .collect::<Vec<char>>();
                let nine_characters = self
                    .find_digit_key_for_value('9')
                    .unwrap()
                    .chars()
                    .collect::<Vec<char>>();
                if !self.contains_all_characters(&sorted_value, six_characters)
                    && !self.contains_all_characters(&sorted_value, nine_characters)
                {
                    self.digit_mapping.insert(sorted_value.to_string(), '0');
                }
            }
        }
    }

    fn decode_signals(&mut self, values_to_decode: &[String]) {
        self.find_one_digit_pattern(values_to_decode);
        self.find_four_digit_pattern(values_to_decode);
        self.find_seven_digit_pattern(values_to_decode);
        self.find_eight_digit_pattern(values_to_decode);
        // 5 character class
        self.find_three_digit_pattern(values_to_decode);
        self.find_five_digit_patthern(values_to_decode);
        self.find_two_digit_pattern(values_to_decode);
        // 6 character class
        self.find_nine_digit_pattern(values_to_decode);
        self.find_six_digit_pattern(values_to_decode);

        self.find_zero_digit_pattern(values_to_decode);
    }

    pub fn decode_signals_and_sum_output_values(&mut self) -> u32 {
        let mut sum_of_output_values: u32 = 0;
        for i in 0..self.output_values.len() {
            let mut combined_values: Vec<String> = Vec::new();
            combined_values.append(&mut self.input_values[i].clone());
            combined_values.append(&mut self.output_values[i].clone());
            self.decode_signals(&combined_values);

            let mut digits_for_line: String = String::new();
            for output_string in &self.output_values[i] {
                let sorted_value = self.sort_string(output_string.to_string());
                let decoded_digit = self.digit_mapping.get(&sorted_value);
                digits_for_line.push(*decoded_digit.unwrap());
            }
            sum_of_output_values += digits_for_line.parse::<u32>().unwrap();
            self.digit_mapping = HashMap::new(); // Reset digit mapping
        }
        sum_of_output_values
    }
}

#[cfg(test)]
mod tests_day8b {
    use super::*;

    #[test]
    fn test_read_lines_from_input_file() {
        let mut seven_segment = SevenSegment::new();
        seven_segment.read_lines_from_input_file("sample_input.txt");

        assert_eq!(seven_segment.input_values.len(), 10);
        assert_eq!(seven_segment.input_values[0].len(), 10);

        assert_eq!(seven_segment.output_values.len(), 10);
        assert_eq!(seven_segment.output_values[0].len(), 4);
    }

    #[test]
    fn test_decode_and_sum_output_values_single_line() {
        let mut seven_segment = SevenSegment::new();
        seven_segment.read_lines_from_input_file("single_sample_input.txt");

        assert_eq!(seven_segment.decode_signals_and_sum_output_values(), 5353);
    }

    #[test]
    fn test_decode_and_sum_output_values() {
        let mut seven_segment = SevenSegment::new();
        seven_segment.read_lines_from_input_file("sample_input.txt");

        assert_eq!(seven_segment.decode_signals_and_sum_output_values(), 61229);
    }
}
