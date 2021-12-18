pub struct PowerDiagnostic {
    gamma_rate: isize,
    epsilon_rate: isize,
    pub power_consumption: isize,
}

impl Default for PowerDiagnostic {
    fn default() -> Self {
        Self::new()
    }
}
impl PowerDiagnostic {
    pub fn new() -> Self {
        PowerDiagnostic {
            gamma_rate: 0,
            epsilon_rate: 0,
            power_consumption: 0,
        }
    }

    pub fn calculate_gamma_rate(&mut self, binary_values: &[String]) {
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
                gamma_string.push('1');
            } else {
                gamma_string.push('0');
            }
        }

        self.gamma_rate = isize::from_str_radix(&gamma_string[..], 2).unwrap();
        self.calculate_power_consumption();
    }

    pub fn calculate_epsilon_rate(&mut self, binary_values: &[String]) {
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
                epsilon_string.push('1');
            } else {
                epsilon_string.push('0');
            }
        }

        self.epsilon_rate = isize::from_str_radix(&epsilon_string[..], 2).unwrap();
        self.calculate_power_consumption();
    }

    fn calculate_power_consumption(&mut self) {
        self.power_consumption = self.gamma_rate * self.epsilon_rate;
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
        assert_eq!(power_diagnostic.power_consumption, 198)
    }
}
