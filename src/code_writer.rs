use std::{fs::File, path::Path, io::Write, collections::HashMap};


pub struct CodeWriter {
    file: File,
    file_base_name: String,
    branch_index: u32,
    first_pop: Vec<String>,
    second_pop: Vec<String>,
    final_push: Vec<String>,
    jump_symbol_table: HashMap<String, String>,
    segment_symbol_table: HashMap<String, String>,
}

impl CodeWriter {
    pub fn new(file_path_str: &str) -> CodeWriter {
        let file_path = Path::new(file_path_str);

        CodeWriter {
            file: File::create(file_path).unwrap(),
            file_base_name: file_path.file_stem().unwrap().to_string_lossy().to_string(),
            branch_index: 1,
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
            ]),
            segment_symbol_table: HashMap::from([
                (String::from("local"), String::from("LCL")),
                (String::from("argument"), String::from("ARG")),
                (String::from("this"), String::from("THIS")),
                (String::from("that"), String::from("THAT")),
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
            "eq" => {
                let command_statements = self.get_comparison_asm("eq");
                statements = self.get_binary_input_asm("eq", command_statements)
            }
            "gt" => {
                let command_statements = self.get_comparison_asm("gt");
                statements = self.get_binary_input_asm("gt", command_statements)
            }
            "lt" => {
                let command_statements = self.get_comparison_asm("lt");
                statements = self.get_binary_input_asm("lt", command_statements)
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

    fn get_comparison_asm(&mut self, command: &str) -> Vec<String> {
        let statements = vec![
            String::from("D=M-D"),
            format!("@THEN{}", self.branch_index),
            format!("D;{}", self.jump_symbol_table[command]),
            String::from("D=0"),
            format!("@END{}", self.branch_index),
            String::from("0;JMP"),
            format!("(THEN{})", self.branch_index),
            String::from("D=-1"),
            format!("(END{})", self.branch_index),
        ];
        self.branch_index += 1;
        statements
    }

    pub fn write_push_pop(&mut self, command: &str, segment: &str, index: u32) {
        let mut statements = vec![format!("// {} {} {}", command, segment, index)];

        match (command, segment, index) {
            ("push", "local" | "argument" | "this" | "that", index) => {
                statements.push(format!("@{}", self.segment_symbol_table[segment]));
                statements.push(String::from("D=M"));
                statements.push(format!("@{}", index));
                statements.push(String::from("A=D+A"));
                statements.push(String::from("D=M"));
                statements.extend(self.final_push.clone());
            },
            ("pop", "local" | "argument" | "this" | "that", index) => {
                statements.push(format!("@{}", self.segment_symbol_table[segment]));
                statements.push(String::from("D=M"));
                statements.push(format!("@{}", index));
                statements.push(String::from("D=D+A"));
                statements.push(String::from("@R13"));
                statements.push(String::from("M=D"));
                statements.push(String::from("@SP"));
                statements.push(String::from("M=M-1"));
                statements.push(String::from("A=M"));
                statements.push(String::from("D=M"));
                statements.push(String::from("@R13"));
                statements.push(String::from("A=M"));
                statements.push(String::from("M=D"));
            },
            ("push", "pointer", 0) => {
                statements.push(String::from("@THIS"));
                statements.push(String::from("D=M"));
                statements.extend(self.final_push.clone());
            },
            ("push", "pointer", 1) => {
                statements.push(String::from("@THAT"));
                statements.push(String::from("D=M"));
                statements.extend(self.final_push.clone());
            },
            ("pop", "pointer", 0) => {
                statements.extend(self.first_pop.clone()); 
                statements.push(String::from("@THIS"));
                statements.push(String::from("M=D"));

            },
            ("pop", "pointer", 1) => {
                statements.extend(self.first_pop.clone()); 
                statements.push(String::from("@THAT"));
                statements.push(String::from("M=D"));
            },
            ("push", "temp", index) => {
                statements.push(String::from("@5"));
                statements.push(String::from("D=A"));
                statements.push(format!("@{}", index));
                statements.push(String::from("A=D+A"));
                statements.push(String::from("D=M"));
                statements.extend(self.final_push.clone());
            },
            ("pop", "temp", index) => {
                statements.push(String::from("@5"));
                statements.push(String::from("D=A"));
                statements.push(format!("@{}", index));
                statements.push(String::from("D=D+A"));
                statements.push(String::from("@R13"));
                statements.push(String::from("M=D"));
                statements.extend(self.first_pop.clone()); 
                statements.push(String::from("@R13"));
                statements.push(String::from("A=M"));
                statements.push(String::from("M=D"));
            },
            ("push", "constant", index) => {
                statements.push(format!("@{}", index));
                statements.push(String::from("D=A"));
                statements.extend(self.final_push.clone());
            },
            ("push", "static", index) => {
                statements.push(format!("@{}.{}", &self.file_base_name, index));
                statements.push(String::from("D=M"));
                statements.extend(self.final_push.clone());
            },
            ("pop", "static", index) => {
                statements.extend(self.first_pop.clone());
                statements.push(format!("@{}.{}", &self.file_base_name, index));
                statements.push(String::from("M=D"));
            },
            _ => {
                
            }
        }
        self.write_statements(statements);
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

    #[test]
    fn test_write_arithmetic_given_ltgt() {
        let out_file = String::from("ltgt.asm");
        let mut code_writer = CodeWriter::new(&out_file);

        code_writer.write_arithmetic("lt");
        code_writer.write_arithmetic("gt");
        verify_output(&out_file);
        fs::remove_file(&out_file).unwrap();
    }

    #[test]
    fn test_write_push_pop_given_push_local() {
        test_write_push_pop("pushlocal2", vec![("push", "local", 2)]);
    }

    #[test]
    fn test_write_push_pop_given_push_argument() {
        test_write_push_pop("pushargument2", vec![("push", "argument", 2)]);
    }

    #[test]
    fn test_write_push_pop_given_push_this() {
        test_write_push_pop("pushthis2", vec![("push", "this", 2)]);
    }

    #[test]
    fn test_write_push_pop_given_push_that() {
        test_write_push_pop("pushthat2", vec![("push", "that", 2)]);
    }

    #[test]
    fn test_write_push_pop_given_pop_local() {
        test_write_push_pop("poplocal2", vec![("pop", "local", 2)]);
    }

    #[test]
    fn test_write_push_pop_given_pop_argument() {
        test_write_push_pop("popargument2", vec![("pop", "argument", 2)]);
    }

    #[test]
    fn test_write_push_pop_given_pop_this() {
        test_write_push_pop("popthis2", vec![("pop", "this", 2)]);
    }

    #[test]
    fn test_write_push_pop_given_pop_that() {
        test_write_push_pop("popthat2", vec![("pop", "that", 2)]);
    }

    #[test]
    fn test_write_push_pop_given_push_pointer() {
        test_write_push_pop("pushpointer", vec![
            ("push", "pointer", 0),
            ("push", "pointer", 1),
        ]);
    }

    #[test]
    fn test_write_push_pop_given_pop_pointer() {
        test_write_push_pop("poppointer", vec![
            ("pop", "pointer", 0),
            ("pop", "pointer", 1),
        ]);
    }

    #[test]
    fn test_write_push_pop_given_push_temp() {
        test_write_push_pop("pushtemp2", vec![("push", "temp", 2)])
    }

    #[test]
    fn test_write_push_pop_given_pop_temp() {
        test_write_push_pop("poptemp2", vec![("pop", "temp", 2)])
    }

    #[test]
    fn test_write_push_pop_given_push_constant() {
        test_write_push_pop("pushconstant2", vec![("push", "constant", 2)])
    }

    #[test]
    fn test_write_push_pop_given_push_static() {
        test_write_push_pop("pushstatic2", vec![("push", "static", 2)])
    }

    #[test]
    fn test_write_push_pop_given_pop_static() {
        test_write_push_pop("popstatic2", vec![("pop", "static", 2)])
    }

    fn test_write_push_pop(test_name: &str, commands: Vec<(&str, &str, u32)>) {
        let out_file = format!("{}.asm", test_name);
        let mut code_writer = CodeWriter::new(&out_file);

        for (command, segment, index) in commands {
            code_writer.write_push_pop(command, segment, index);
        }
        verify_output(&out_file);
        fs::remove_file(&out_file).unwrap();
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
