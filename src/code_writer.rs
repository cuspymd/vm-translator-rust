use std::{fs::File, path::Path, io::Write, collections::HashMap};


pub struct CodeWriter {
    file: File,
    first_pop: Vec<String>,
    second_pop: Vec<String>,
    final_push: Vec<String>,
    jump_symbol_table: HashMap<String, String>,
}

impl CodeWriter {
    pub fn new(file_path_str: &str) -> CodeWriter {
        let file_path = Path::new(file_path_str);

        CodeWriter {
            file: File::create(file_path).unwrap(),
            first_pop: vec![
                String::from("@SP"),
                String::from("M=M-1"),
                String::from("A=M"),
                String::from("D=M"),
            ],
            second_pop: vec![
                String::from("@SP"),
                String::from("M=M-1"),
                String::from("A=M"),
            ],
            final_push: vec![
                String::from("@SP"),
                String::from("A=M"),
                String::from("M=D"),
                String::from("@SP"),
                String::from("M=M+1"),
            ],
            jump_symbol_table: HashMap::from([
                (String::from("eq"), String::from("JEQ")),
                (String::from("gt"), String::from("JGT")),
                (String::from("lt"), String::from("JLT")),
            ])
        }
    }

    pub fn write_arithmetic(&mut self, command: &str) {
        let statements: Vec<String>;
        match command {
            "add" => {
                statements = self.get_binary_input_asm("add", vec![String::from("D=D+M")])   
            }
            "sub" => {
                statements = self.get_binary_input_asm("sub", vec![String::from("D=M-D")])   
            }
            "and" => {
                statements = self.get_binary_input_asm("and", vec![String::from("D=D&M")])   
            }
            "or" => {
                statements = self.get_binary_input_asm("or", vec![String::from("D=D|M")])   
            }
            "neg" => {
                statements = self.get_unary_input_asm("neg", vec![String::from("D=-D")])   
            }
            "not" => {
                statements = self.get_unary_input_asm("not", vec![String::from("D=!D")])   
            }
            _ => {
                statements = Vec::new()
            }
        }
        self.write_statements(statements);
    }

    fn write_statements(&mut self, statements: Vec<String>) {
        let lines: Vec<String> = statements.iter().map(CodeWriter::post_process).collect();
        for line in lines {
            self.file.write_all(line.as_bytes()).unwrap()
        }
    }

    fn post_process(statement: &String) -> String {
        if let Some(first_char) = statement.chars().next() {
            match first_char {
                '(' | '/' => format!("{}\n", statement),
                _ => format!("  {}\n", statement)
            }
        } else {
            String::from("")
        }
    }

    fn get_binary_input_asm(
        &self, command_name: &str, command_statements: Vec<String>) -> Vec<String> {

        let mut statements = vec![format!("// {}", command_name)];
        statements.extend(self.first_pop.clone());
        statements.extend(self.second_pop.clone());
        statements.extend(command_statements);
        statements.extend(self.final_push.clone());
        statements
    }

    fn get_unary_input_asm(
        &self, command_name: &str, command_statements: Vec<String>) -> Vec<String> {
        
        let mut statements = vec![format!("// {}", command_name)];
        statements.extend(self.first_pop.clone());
        statements.extend(command_statements);
        statements.extend(self.final_push.clone());
        statements
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::{self, File}, path::Path, io::Read};
    use super::CodeWriter;

    #[test]
    fn test_write_arithmetic_given_add() {
        test_write_arithmetic("add");
    }

    #[test]
    fn test_write_arithmetic_given_sub() {
        test_write_arithmetic("sub");
    }

    #[test]
    fn test_write_arithmetic_given_and() {
        test_write_arithmetic("and");
    }

    #[test]
    fn test_write_arithmetic_given_or() {
        test_write_arithmetic("or");
    }

    #[test]
    fn test_write_arithmetic_given_neg() {
        test_write_arithmetic("neg");
    }

    #[test]
    fn test_write_arithmetic_given_not() {
        test_write_arithmetic("not");
    }

    #[test]
    fn test_write_arithmetic_given_eq() {
        test_write_arithmetic("eq");
    }

    #[test]
    fn test_write_arithmetic_given_gt() {
        test_write_arithmetic("gt");
    }

    #[test]
    fn test_write_arithmetic_given_lt() {
        test_write_arithmetic("lt");
    }

    fn test_write_arithmetic(test_command: &str) {
        let out_file = format!("{}.asm", test_command);
        let mut code_writer = CodeWriter::new(&out_file);

        code_writer.write_arithmetic(test_command);
        verify_output(&out_file);
        fs::remove_file(&out_file).unwrap();
    }

    fn verify_output(out_file: &str) {
        let out_file_stem = Path::new(out_file).file_stem().unwrap().to_string_lossy().to_string();
        let solution_file = format!("test_data/solution_{}.asm", out_file_stem);

        let out = load_text(out_file);
        let solution = load_text(&solution_file);

        assert_eq!(out, solution);
    }

    fn load_text(file_path: &str) -> String {
        let mut file = File::open(file_path).unwrap();
        let mut text = String::new();

        file.read_to_string(&mut text).unwrap();
        text
    }
}
