use std::{fs::File, path::Path, io::Write, collections::HashMap};


pub struct CodeWriter {
    file: File,
    file_base_name: String,
    current_function_name: String,
    branch_index: u32,
    return_index: u32,
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
            current_function_name: String::from(""),
            branch_index: 1,
            return_index: 1,
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
            format!("@{}_THEN{}", self.get_label_prefix(), self.branch_index),
            format!("D;{}", self.jump_symbol_table[command]),
            String::from("D=0"),
            format!("@{}_END{}", self.get_label_prefix(), self.branch_index),
            String::from("0;JMP"),
            format!("({}_THEN{})", self.get_label_prefix(), self.branch_index),
            String::from("D=-1"),
            format!("({}_END{})", self.get_label_prefix(), self.branch_index),
        ];
        self.branch_index += 1;
        statements
    }

    fn get_label_prefix(&self) -> &str {
        if self.current_function_name.is_empty() {
            &self.file_base_name
        } else {
            &self.current_function_name
        }
    }

    pub fn write_push_pop(&mut self, command: &str, segment: &str, index: i32) {
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

    pub fn write_function(&mut self, function_name: &str, nvars: i32) {
        let mut statements = vec![
            format!("// function {} {}", function_name, nvars),
            format!("({})", function_name),
        ];
        statements.extend(self.get_push_nvars_asm(nvars));

        self.write_statements(statements);
        self.current_function_name = function_name.to_string()
    }

    fn get_push_nvars_asm(&self, nvars: i32) -> Vec<String> {
        let mut statements = Vec::new();
        let push_statements = vec![
            String::from("@SP"),
            String::from("A=M"),
            String::from("M=0"),
            String::from("@SP"),
            String::from("M=M+1"),
        ];

        for _ in 0..nvars {
            statements.extend(push_statements.clone());
        }

        statements
    }

    pub fn write_label(&mut self, label: &str) {
        let statements = vec![
            format!("// label {}", label),
            format!("({}${})", self.get_label_prefix(), label),
        ];
        self.write_statements(statements);
    }

    pub fn write_goto(&mut self, label: &str) {
        let statements = vec![
            format!("// goto {}", label),
            format!("@{}${}", self.get_label_prefix(), label),
            String::from("0;JMP"),
        ];
        self.write_statements(statements);
    }

    pub fn write_if(&mut self, label: &str) {
        let mut statements = vec![format!("// if {}", label)];
        statements.extend(self.first_pop.clone());
        statements.push(format!("@{}${}", self.get_label_prefix(), label));
        statements.push(String::from("D;JNE"));

        self.write_statements(statements);
    }

    pub fn write_call(&mut self, function_name: &str, nvars: i32) {
        let return_label = format!("{}$ret.{}", self.get_label_prefix(), self.return_index);
        let mut statements = vec![
            format!("// call {} {}", function_name, nvars),
            format!("@{}", &return_label),
            String::from("D=A"),
        ];
        statements.extend(self.final_push.clone());
        statements.extend(self.get_push_segment_asm("LCL"));
        statements.extend(self.get_push_segment_asm("ARG"));
        statements.extend(self.get_push_segment_asm("THIS"));
        statements.extend(self.get_push_segment_asm("THAT"));
        statements.extend(vec![
            String::from("@SP"),
            String::from("D=M"),
            String::from("@5"),
            String::from("D=D-A"),
            format!("@{}", nvars),
            String::from("D=D-A"),
            String::from("@ARG"),
            String::from("M=D"),
            String::from("@SP"),
            String::from("D=M"),
            String::from("@LCL"),
            String::from("M=D"),
            format!("@{}", function_name),
            String::from("0;JMP"),
            format!("({})", return_label),
        ]);
        self.write_statements(statements);
        self.return_index += 1;
    }

    pub fn write_return(&mut self) {
        let mut statements = vec![
            String::from("// return"),
            String::from("@LCL"),
            String::from("D=M"),
            String::from("@R13"),
            String::from("M=D"),
            String::from("@5"),
            String::from("D=D-A"),
            String::from("A=D"),
            String::from("D=M"),
            String::from("@R14"),
            String::from("M=D"),
        ];
        statements.extend(self.first_pop.clone());
        statements.extend(vec![
            String::from("@ARG"),
            String::from("A=M"),
            String::from("M=D"),
            String::from("@ARG"),
            String::from("D=M"),
            String::from("D=D+1"),
            String::from("@SP"),
            String::from("M=D"),
        ]);
        statements.extend(self.get_recover_segment_asm("THAT", 1));
        statements.extend(self.get_recover_segment_asm("THIS", 2));
        statements.extend(self.get_recover_segment_asm("ARG", 3));
        statements.extend(self.get_recover_segment_asm("LCL", 4));
        statements.extend(vec![
            String::from("@R14"),
            String::from("A=M"),
            String::from("0;JMP"),
        ]);

        self.write_statements(statements);
    }

    fn get_recover_segment_asm(&self, segment: &str, index: u32) -> Vec<String> {
        vec![
            String::from("@R13"),
            String::from("D=M"),
            format!("@{}", index),
            String::from("D=D-A"),
            String::from("A=D"),
            String::from("D=M"),
            format!("@{}", segment),
            String::from("M=D"),
        ]
    }

    fn get_push_segment_asm(&self, segment: &str) -> Vec<String> {
        let mut statements = vec![
            format!("@{}", segment),
            String::from("D=M"),
        ];
        statements.extend(self.final_push.clone());
        statements
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};
    use crate::util::load_text;

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

    #[test]
    fn test_write_function_given_no_vars() {
        test_write_function("function0", vec![("Main.test", 0)])
    }

    #[test]
    fn test_write_function_given_2_vars() {
        test_write_function("function2", vec![("Main.test", 2)])
    }

    #[test]
    fn test_write_label_given_file() {
        let out_file = "LabelInFile.asm";
        let mut code_writer = CodeWriter::new(out_file);

        code_writer.write_label("LABEL");

        verify_output(out_file);
        fs::remove_file(out_file).unwrap();
    }

    #[test]
    fn test_write_label_given_function() {
        let out_file = "LabelInFunction.asm";
        let mut code_writer = CodeWriter::new(out_file);

        code_writer.write_function("LabelInFunction.test", 0);
        code_writer.write_label("LABEL");

        verify_output(out_file);
        fs::remove_file(out_file).unwrap();
    }

    #[test]
    fn test_write_goto_given_file() {
        let out_file = "GotoInFile.asm";
        let mut code_writer = CodeWriter::new(out_file);

        code_writer.write_goto("LABEL");

        verify_output(out_file);
        fs::remove_file(out_file).unwrap();
    }

    #[test]
    fn test_write_goto_given_function() {
        let out_file = "GotoInFunction.asm";
        let mut code_writer = CodeWriter::new(out_file);

        code_writer.write_function("GotoInFunction.test", 0);
        code_writer.write_goto("LABEL");

        verify_output(out_file);
        fs::remove_file(out_file).unwrap();
    }

    #[test]
    fn test_write_if_given_file() {
        let out_file = "IfInFile.asm";
        let mut code_writer = CodeWriter::new(out_file);

        code_writer.write_if("LABEL");

        verify_output(out_file);
        fs::remove_file(out_file).unwrap();
    }

    #[test]
    fn test_write_if_given_function() {
        let out_file = "IfInFunction.asm";
        let mut code_writer = CodeWriter::new(out_file);

        code_writer.write_function("IfInFunction.test", 0);
        code_writer.write_if("LABEL");

        verify_output(out_file);
        fs::remove_file(out_file).unwrap();
    }

    #[test]
    fn test_write_call_given_file() {
        let out_file = "CallInFile.asm";
        let mut code_writer = CodeWriter::new(out_file);

        code_writer.write_call("Math.add", 2);

        verify_output(out_file);
        fs::remove_file(out_file).unwrap();
    }

    #[test]
    fn test_write_call_given_function() {
        let out_file = "CallInFunction.asm";
        let mut code_writer = CodeWriter::new(out_file);

        code_writer.write_function("CallInFunction.test", 0);
        code_writer.write_call("Math.add", 2);

        verify_output(out_file);
        fs::remove_file(out_file).unwrap();
    }

    #[test]
    fn test_write_call_given_multi_calls() {
        let out_file = "CallGivenMultiCalls.asm";
        let mut code_writer = CodeWriter::new(out_file);

        code_writer.write_function("CallGivenMultiCalls.test", 0);
        code_writer.write_call("Math.add", 2);
        code_writer.write_call("Math.sum", 0);

        verify_output(out_file);
        fs::remove_file(out_file).unwrap();
    }

    #[test]
    fn test_write_return() {
        let out_file = "Return.asm";
        let mut code_writer = CodeWriter::new(out_file);

        code_writer.write_return();

        verify_output(out_file);
        fs::remove_file(out_file).unwrap();
    }

    fn test_write_function(test_name: &str, commands: Vec<(&str, i32)>) {
        let out_file = format!("{}.asm", test_name);
        let mut code_writer = CodeWriter::new(&out_file);

        for (function_name, nvars) in commands {
            code_writer.write_function(function_name, nvars);
        }
        verify_output(&out_file);
        fs::remove_file(&out_file).unwrap();
    }

    fn test_write_push_pop(test_name: &str, commands: Vec<(&str, &str, i32)>) {
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
}
