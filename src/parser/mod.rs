pub mod command;
pub use command::{Command, CommandType};


pub struct Parser {
    lines: Vec<String>,
    current_line_number: i32,
    current_command: Command,
}

impl Parser {
    pub fn new(file_text: &str) -> Parser {
        Parser {
            lines: Parser::get_valid_lines(file_text),
            current_line_number: -1,
            current_command: Command::new(""),
        }
    }

    fn get_valid_lines(file_text: &str) -> Vec<String> {
        file_text
            .lines()
            .map(|line| Parser::get_valid_text(line))
            .filter(|line| !line.is_empty())
            .collect()
    }

    fn get_valid_text(text: &str) -> String {
        let valid_text: &str = match text.split("//").next() {
            Some(first_part) => first_part,
            None => text
        };

        valid_text.trim().to_string()
    }

    pub fn has_more_lines(&self) -> bool {
        self.current_line_number < self.lines.len() as i32 -1
    }

    pub fn advance(&mut self) {
        self.current_line_number += 1;
        self.current_command = Command::new(&self.lines[self.current_line_number as usize]);
    }

    pub fn command_type(&self) -> &CommandType {
        self.current_command.get_command_type()
    }

    pub fn arg1(&self) -> &str {
        self.current_command.get_arg1()
    }

    pub fn arg2(&self) -> i32 {
        match self.current_command.get_arg2() {
            Some(arg2) => arg2,
            None => panic!("Not available")
        }
    }
}


#[cfg(test)]
mod tests {
    use std::panic;

    use super::*;

    #[test]
    fn test_has_more_lines_given_one_lines() {
        let parser = Parser::new("push constant 17");
        assert!(parser.has_more_lines());
    }

    #[test]
    fn test_has_more_lines_given_empty_lines() {
        let parser = Parser::new("");
        assert!(!parser.has_more_lines());

        let parser = Parser::new("\n   \n     \n");
        assert!(!parser.has_more_lines());

        let parser = Parser::new("\r\n   \r\n     \r\n");
        assert!(!parser.has_more_lines());

        let parser = Parser::new("\n   \n     push local 2\n");
        assert!(parser.has_more_lines());
    }

    #[test]
    fn test_has_more_lines_given_comment_line() {
        let parser = Parser::new("// comment");
        assert!(!parser.has_more_lines());
    }

    #[test]
    fn test_advance() {
        let mut parser = Parser::new("add");
        assert!(parser.has_more_lines());
        parser.advance();
        assert!(!parser.has_more_lines());
    }

    #[test]
    fn test_advance_given_two_lines() {
        let mut parser = Parser::new("add\nsub");
        assert!(parser.has_more_lines());
        parser.advance();
        assert!(parser.has_more_lines());
        parser.advance();
        assert!(!parser.has_more_lines());
    }

    #[test]
    fn test_command_type_given_arithmetic_command() {
        let commands: [&str; 9] = [
            "add", "sub", "neg", "eq", "gt", "lt", "and", "or", "not"
        ];
        for command in commands {
            let mut parser = Parser::new(command);
            parser.advance();
            assert!(matches!(parser.command_type(), CommandType::Arithmetic));
        }
    }

    #[test]
    fn test_command_type_given_stack_command() {
        let mut parser = Parser::new("push constant 17\npop local 2");
        parser.advance();
        assert!(matches!(parser.command_type(), CommandType::Push));
        parser.advance();
        assert!(matches!(parser.command_type(), CommandType::Pop));
    }

    #[test]
    fn test_arg_given_arithmetic_command() {
        let mut parser = Parser::new("add");
        parser.advance();
        assert_eq!(parser.arg1(), "add");
    }

    #[test]
    fn test_arg_given_push_command() {
        let mut parser = Parser::new("push constant 1");
        parser.advance();
        assert_eq!(parser.arg1(), "constant");
        assert_eq!(parser.arg2(), 1);
    }

    #[test]
    fn test_arg_given_pop_command() {
        let mut parser = Parser::new("pop temp 12");
        parser.advance();
        assert_eq!(parser.arg1(), "temp");
        assert_eq!(parser.arg2(), 12);
    }

    #[test]
    fn test_arg_given_invalid_type() {
        let mut parser = Parser::new("sub");
        parser.advance();
        assert!(panic::catch_unwind(|| { parser.arg2(); }).is_err());
    }
}
