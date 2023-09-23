
pub struct Parser {
    lines: Vec<String>,
}

impl Parser {
    pub fn new(file_text: &str) -> Parser {
        Parser {
            lines: Parser::get_valid_lines(file_text),
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
        self.lines.len() > 0
    }
}


#[cfg(test)]
mod tests {
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

}
