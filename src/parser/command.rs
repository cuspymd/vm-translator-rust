pub enum CommandType {
    Arithmetic,
    Push,
    Pop,
    Label,
    Goto,
    If,
    Function,
    Return,
    Call,
}

pub struct Command {
    command_type: CommandType,
    arg1: String,
    arg2: Option<i32>,
}

impl Command {
    pub fn new(text: &str) -> Command {
        match text.split(" ").collect::<Vec<&str>>().as_slice() {
            ["push", arg1, arg2] => {
                return Command {
                    command_type: CommandType::Push,
                    arg1: arg1.to_string(),
                    arg2: arg2.parse().ok(),
                }
            },
            ["pop", arg1, arg2] => {
                return Command {
                    command_type: CommandType::Pop,
                    arg1: arg1.to_string(),
                    arg2: arg2.parse().ok(),
                }
            },
            ["label", label] => {
                return Command {
                    command_type: CommandType::Label,
                    arg1: label.to_string(),
                    arg2: None,
                }
            },
            ["goto", label] => {
                return Command {
                    command_type: CommandType::Goto,
                    arg1: label.to_string(),
                    arg2: None,
                }
            },
            ["if-goto", label] => {
                return Command {
                    command_type: CommandType::If,
                    arg1: label.to_string(),
                    arg2: None,
                }
            },
            ["function", function_name, nvars] => {
                return Command {
                    command_type: CommandType::Function,
                    arg1: function_name.to_string(),
                    arg2: nvars.parse().ok(),
                }
            },
            ["call", function_name, nvars] => {
                return Command {
                    command_type: CommandType::Call,
                    arg1: function_name.to_string(),
                    arg2: nvars.parse().ok(),
                }
            },
            ["return"] => {
                return Command {
                    command_type: CommandType::Return,
                    arg1: String::from(""),
                    arg2: None,
                }
            },
            [command] => {
                return Command {
                    command_type: CommandType::Arithmetic,
                    arg1: command.to_string(),
                    arg2: None,
                }
            }
            _ => {
                panic!("Invalid command!!");
            }
        }
    }

    pub fn get_command_type(&self) -> &CommandType {
        &self.command_type
    }

    pub fn get_arg1(&self) -> &str {
        &self.arg1
    }

    pub fn get_arg2(&self) -> Option<i32> {
        self.arg2
    }
}
