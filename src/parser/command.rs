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
    arg2: i32,
}

impl Command {
    pub fn new(text: &str) -> Command {
        match text.split(" ").collect::<Vec<&str>>().as_slice() {
            ["push", arg1, arg2] => {
                return Command {
                    command_type: CommandType::Push,
                    arg1: arg1.to_string(),
                    arg2: arg2.parse().unwrap(),
                }
            },
            ["pop", arg1, arg2] => {
                return Command {
                    command_type: CommandType::Pop,
                    arg1: arg1.to_string(),
                    arg2: arg2.parse().unwrap(),
                }
            },
            [command] => {
                return Command {
                    command_type: CommandType::Arithmetic,
                    arg1: command.to_string(),
                    arg2: 0,
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
}
