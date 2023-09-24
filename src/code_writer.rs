use std::{fs::File, path::Path, io::Write};


pub struct CodeWriter {
    file: File,
    first_pop: Vec<&'static str>,
    second_pop: Vec<&'static str>,
    final_push: Vec<&'static str>,
}

impl CodeWriter {
    pub fn new(file_path_str: &str) -> CodeWriter {
        let input_path = Path::new(file_path_str);

        CodeWriter {
            file: File::open(input_path).unwrap(),
            first_pop: vec![
                "@SP",
                "M=M-1",
                "A=M",
                "D=M",
            ],
            second_pop: vec![
                "@SP",
                "M=M-1",
                "A=M",
            ],
            final_push: vec![
                "@SP",
                "A=M",
                "M=D",
                "@SP",
                "M=M+1",
            ]
        }
    }

    pub fn write_arithmetic(&self, command: &str) {
        let mut statements: Vec<&str>;
        match command {
            "add" => {
                statements = self.get_binary_input_asm("add", vec![String::from("D=D+M").as_str()])   
            }
            _ => {
                
            }
        }
        self.write_statements(statements);
    }

    fn write_statements(&self, statements: Vec<&str>) {
        let lines = statements.iter().map(CodeWriter::post_process).collect();
        for line in lines {
            self.file.write_all(line.as_bytes()).unwrap()
        }
    }

    fn post_process(statement: &str) -> &str {
        if let Some(first_char) = statement.chars().next() {
            match first_char {
                '(' | '/' => format!("{}\n", statement).as_str(),
                _ => format!("  {}\n", statement).as_str()
            }
        } else {
            ""
        }
    }

    fn get_binary_input_asm(
        &self, command_name: &str, command_statements: Vec<&str>) -> Vec<&str> {

        let mut statements = vec![format!("// {}", command_name).as_str()];
        statements.extend(self.first_pop);
        statements.extend(self.second_pop);
        statements.extend(command_statements);
        statements.extend(self.final_push);
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
