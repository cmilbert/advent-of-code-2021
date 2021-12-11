pub struct PowerDiagnostic {
    gamma_rate: isize,
    epsilon_rate: isize,
    pub power_consumption: isize,

    oxygen_generator_rating: isize,
    co2_scrubber_rating: isize,
    pub life_support_rating: isize,
}

impl PowerDiagnostic {
    pub fn new() -> Self {
        PowerDiagnostic {
            gamma_rate: 0,
            epsilon_rate: 0,
            power_consumption: 0,
            oxygen_generator_rating: 0,
            co2_scrubber_rating: 0,
            life_support_rating: 0,
        }
    }

    pub fn calculate_gamma_rate(&mut self, binary_values: &Vec<String>) {
        let mut gamma_string: String = "".to_owned();

        for i in 0..binary_values[0].len() {
            let mut count_of_one = 0;
            let mut count_of_zero = 0;

            for binary_value in binary_values {
                if binary_value.chars().nth(i).unwrap() == '1' {
                    count_of_one += 1;
                } else {
                    count_of_zero += 1;
                }
            }

            if count_of_one > count_of_zero {
                gamma_string.push_str("1");
            } else {
                gamma_string.push_str("0");
            }
        }

        self.gamma_rate = isize::from_str_radix(&gamma_string[..], 2).unwrap();
        self.calculate_power_consumption();
    }

    pub fn calculate_epsilon_rate(&mut self, binary_values: &Vec<String>) {
        let mut epsilon_string: String = "".to_owned();

        for i in 0..binary_values[0].len() {
            let mut count_of_one = 0;
            let mut count_of_zero = 0;

            for binary_value in binary_values {
                if binary_value.chars().nth(i).unwrap() == '1' {
                    count_of_one += 1;
                } else {
                    count_of_zero += 1;
                }
            }

            if count_of_one < count_of_zero {
                epsilon_string.push_str("1");
            } else {
                epsilon_string.push_str("0");
            }
        }

        self.epsilon_rate = isize::from_str_radix(&epsilon_string[..], 2).unwrap();
        self.calculate_power_consumption();
    }

    fn calculate_power_consumption(&mut self) {
        self.power_consumption = self.gamma_rate * self.epsilon_rate;
    }

    pub fn calculate_oxygen_generator_rating(&mut self, binary_values: &Vec<String>) {
        let mut oxygen_generator_rating_string: String = "".to_owned();
        let mut values_to_process: Vec<String> = binary_values.clone();

        for i in 0..binary_values[0].len() {
            let mut count_of_one = 0;
            let mut count_of_zero = 0;
            for j in 0..values_to_process.len() {
                let processing_character = values_to_process[j].chars().nth(i).unwrap();
                if processing_character == '1' {
                    count_of_one += 1;
                } else {
                    count_of_zero += 1;
                }
            }

            let keep_values_starting_with_one: bool = count_of_one >= count_of_zero;
            let keep_values_starting_with_zero: bool = count_of_one < count_of_zero;

            let mut updated_values = Vec::new();
            for j in 0..values_to_process.len() {
                let value = values_to_process[j].to_string();
                let character_value_at_position: char = value.chars().nth(i).unwrap();

                if character_value_at_position == '1' && keep_values_starting_with_one {
                    updated_values.push(value);
                } else if character_value_at_position == '0' && keep_values_starting_with_zero {
                    updated_values.push(value);
                }
            }
            values_to_process = updated_values;

            if values_to_process.len() == 1 {
                oxygen_generator_rating_string = values_to_process[0].to_string();
                break;
            }
        }

        self.oxygen_generator_rating =
            isize::from_str_radix(&oxygen_generator_rating_string[..], 2).unwrap();
        self.calculate_life_support_rating();
    }

    pub fn calculate_co2_scrubber_rating(&mut self, binary_values: &Vec<String>) {
        let mut co2_scrubber_rating_string: String = "".to_owned();
        let mut values_to_process: Vec<String> = binary_values.clone();

        for i in 0..binary_values[0].len() {
            let mut count_of_one = 0;
            let mut count_of_zero = 0;
            for j in 0..values_to_process.len() {
                let processing_character = values_to_process[j].chars().nth(i).unwrap();
                if processing_character == '1' {
                    count_of_one += 1;
                } else {
                    count_of_zero += 1;
                }
            }

            let keep_values_starting_with_one: bool = count_of_one < count_of_zero;
            let keep_values_starting_with_zero: bool = count_of_one >= count_of_zero;

            let mut updated_values = Vec::new();
            for j in 0..values_to_process.len() {
                let value = values_to_process[j].to_string();
                let character_value_at_position: char = value.chars().nth(i).unwrap();

                if character_value_at_position == '1' && keep_values_starting_with_one {
                    updated_values.push(value);
                } else if character_value_at_position == '0' && keep_values_starting_with_zero {
                    updated_values.push(value);
                }
            }
            values_to_process = updated_values;

            if values_to_process.len() == 1 {
                co2_scrubber_rating_string = values_to_process[0].to_string();
                break;
            }
        }

        self.co2_scrubber_rating =
            isize::from_str_radix(&co2_scrubber_rating_string[..], 2).unwrap();
        self.calculate_life_support_rating();
    }

    fn calculate_life_support_rating(&mut self) {
        self.life_support_rating = self.oxygen_generator_rating * self.co2_scrubber_rating;
    }
}

#[cfg(test)]
mod tests_day3a {
    use super::*;

    fn get_sample_data() -> Vec<String> {
        return vec![
            "00100".to_owned(),
            "11110".to_owned(),
            "10110".to_owned(),
            "10111".to_owned(),
            "10101".to_owned(),
            "01111".to_owned(),
            "00111".to_owned(),
            "11100".to_owned(),
            "10000".to_owned(),
            "11001".to_owned(),
            "00010".to_owned(),
            "01010".to_owned(),
        ];
    }

    #[test]
    fn test_create_power_diagnostic() {
        let power_diagnostic = PowerDiagnostic::new();
        assert_eq!(power_diagnostic.gamma_rate, 0);
        assert_eq!(power_diagnostic.epsilon_rate, 0);
    }

    #[test]
    fn test_calculate_gamma() {
        let mut power_diagnostic = PowerDiagnostic::new();
        power_diagnostic.calculate_gamma_rate(&get_sample_data());
        assert_eq!(power_diagnostic.gamma_rate, 22);
    }

    #[test]
    fn test_calculate_epsilon() {
        let mut power_diagnostic = PowerDiagnostic::new();
        power_diagnostic.calculate_epsilon_rate(&get_sample_data());
        assert_eq!(power_diagnostic.epsilon_rate, 9)
    }

    #[test]
    fn test_calculate_power_consumption() {
        let mut power_diagnostic = PowerDiagnostic::new();
        power_diagnostic.calculate_gamma_rate(&get_sample_data());
        power_diagnostic.calculate_epsilon_rate(&get_sample_data());
        assert_eq!(power_diagnostic.power_consumption, 198);
    }

    #[test]
    fn test_calculate_oxygen_generator_rating() {
        let mut power_diagnostic = PowerDiagnostic::new();
        power_diagnostic.calculate_oxygen_generator_rating(&get_sample_data());
        assert_eq!(power_diagnostic.oxygen_generator_rating, 23);
    }

    #[test]
    fn test_calculate_co2_scrubber_rating() {
        let mut power_diagnostic = PowerDiagnostic::new();
        power_diagnostic.calculate_co2_scrubber_rating(&get_sample_data());
        assert_eq!(power_diagnostic.co2_scrubber_rating, 10);
    }

    #[test]
    fn test_calculate_life_support_rating() {
        let mut power_diagnostic = PowerDiagnostic::new();
        power_diagnostic.calculate_oxygen_generator_rating(&get_sample_data());
        power_diagnostic.calculate_co2_scrubber_rating(&get_sample_data());
        assert_eq!(power_diagnostic.life_support_rating, 230);
    }
}
