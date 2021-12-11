pub struct Submarine {
    horizontal_position: u32,
    vertical_position: i32,
}

impl Submarine {
    pub fn new(horizontal_position: u32, vertical_position: i32) -> Self {
        Submarine {
            horizontal_position: horizontal_position,
            vertical_position: vertical_position,
        }
    }

    pub fn process_instructions(&mut self, instructions: Vec<String>) {
        for instruction in instructions {
            self.process_instruction(instruction)
        }
    }

    fn process_instruction(&mut self, instruction: String) {
        let split = instruction.split(" ");
        let split_vec = split.collect::<Vec<&str>>();
        let command = split_vec[0].to_lowercase();
        let count = split_vec[1];

        match command.as_ref() {
            "forward" => self.forward(count.parse::<u32>().unwrap()),
            "up" => self.up(count.parse::<i32>().unwrap()),
            "down" => self.down(count.parse::<i32>().unwrap()),
            _ => println!("Invalid command"),
        }
    }

    fn forward(&mut self, count: u32) {
        self.horizontal_position += count;
    }

    fn down(&mut self, count: i32) {
        self.vertical_position -= count;
    }

    fn up(&mut self, count: i32) {
        self.vertical_position += count;
    }

    pub fn multiply_positions(&self) -> i32 {
        return (self.horizontal_position as i32 * self.vertical_position).abs();
    }
}

#[cfg(test)]
mod tests_day2a {
    use super::*;

    #[test]
    fn test_create_submarine() {
        let submarine = Submarine::new(0, 0);
        assert_eq!(submarine.horizontal_position, 0);
        assert_eq!(submarine.vertical_position, 0);
    }

    #[test]
    fn test_move_submarine_forward() {
        let mut submarine = Submarine::new(0, 0);

        submarine.forward(5);
        assert_eq!(submarine.horizontal_position, 5);

        submarine.forward(2);
        assert_eq!(submarine.horizontal_position, 7);
    }

    #[test]
    fn test_move_submarine_down() {
        let mut submarine = Submarine::new(0, 0);

        submarine.down(3);
        assert_eq!(submarine.vertical_position, -3);

        submarine.down(2);
        assert_eq!(submarine.vertical_position, -5);
    }

    #[test]
    fn test_move_submarine_up() {
        let mut submarine = Submarine::new(0, 0);

        submarine.up(8);
        assert_eq!(submarine.vertical_position, 8);

        submarine.up(10);
        assert_eq!(submarine.vertical_position, 18);
    }

    #[test]
    fn test_process_instruction() {
        let mut submarine = Submarine::new(0, 0);

        submarine.process_instruction("forward 5".to_string());
        assert_eq!(submarine.horizontal_position, 5);

        submarine.process_instruction("up 5".to_string());
        assert_eq!(submarine.vertical_position, 5);

        submarine.process_instruction("down 10".to_string());
        assert_eq!(submarine.vertical_position, -5);
    }

    #[test]
    fn test_multiply_positions() {
        let submarine = Submarine::new(5, 10);
        assert_eq!(submarine.multiply_positions(), 50);

        let submarine2 = Submarine::new(5, -20);
        assert_eq!(submarine2.multiply_positions(), 100);
    }

    #[test]
    fn test_sample_data() {
        let instructions = vec![
            "forward 5".to_string(),
            "down 5".to_string(),
            "forward 8".to_string(),
            "up 3".to_string(),
            "down 8".to_string(),
            "forward 2".to_string(),
        ];
        let mut submarine = Submarine::new(0, 0);
        submarine.process_instructions(instructions);
        assert_eq!(submarine.multiply_positions(), 150)
    }
}
