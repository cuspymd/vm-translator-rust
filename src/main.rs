use std::{path::Path, env};

use clap::{command, Arg, ArgAction};
use code_writer::CodeWriter;
use parser::{Parser, CommandType};
use util::load_text;

mod parser;
mod code_writer;
mod util;

fn translate(input_path_str: &str, need_bootstrap: bool) {
    let input_path = Path::new(input_path_str);
    if input_path.is_file() {
        translate_file(input_path_str);
    } else if input_path.is_dir() {
        
    }
}

fn translate_file(input_path_str: &str) {
    let input_path = Path::new(input_path_str);
    let folder_path = input_path.parent().unwrap();
    let file_base_name = input_path.file_stem().unwrap()
        .to_string_lossy().to_string();
    let output_path_str = &folder_path.join(format!("{}.asm", &file_base_name))
        .to_string_lossy().to_string();
    let input_text = load_text(input_path_str);

    let mut parser = Parser::new(&input_text);
    let mut code_writer = CodeWriter::new(&output_path_str);

    while parser.has_more_lines() {
        parser.advance();

        match parser.command_type() {
            CommandType::Arithmetic => {
                code_writer.write_arithmetic(parser.arg1());
            }
            CommandType::Push => {
                code_writer.write_push_pop("push", parser.arg1(), parser.arg2());
            }
            CommandType::Pop => {
                code_writer.write_push_pop("pop", parser.arg1(), parser.arg2());
            }
            CommandType::Label => {
                code_writer.write_label(parser.arg1());
            }
            CommandType::Goto => {
                code_writer.write_goto(parser.arg1());
            }
            CommandType::If => {
                code_writer.write_if(parser.arg1());
            }
            CommandType::Function => {
                code_writer.write_function(parser.arg1(), parser.arg2());
            }
            CommandType::Call => {
                code_writer.write_call(parser.arg1(), parser.arg2());
            }
            CommandType::Return => {
                code_writer.write_return();
            }
        }
    }
}

fn main() {
    let matches = command!()
        .arg(Arg::new("input_path")
             .help("Path of vm file to be translated")
             .required(true))
        .arg(Arg::new("no_bootstrap")
             .long("no-bootstrap")
             .action(ArgAction::SetTrue)
             .help("Do not make bootstrap codes"))
        .get_matches();

    let input_path = matches.get_one::<String>("input_path").unwrap();
    let need_bootstrap = !matches.get_flag("no_bootstrap");

    println!("Start translating for '{}", input_path);
    translate(&input_path, need_bootstrap);
    println!("Completed");
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};
    use crate::{translate, util::load_text};

    #[test]
    fn test_main_given_stack_commands() {
        test_vm("Add.vm");
    }

    #[test]
    fn test_main_given_control_commands() {
        test_vm("Control.vm");
    }

    // #[test]
    // fn test_main_given_folder() {
    //     test_vm("TestFolder");
    // }

    fn test_vm(test_dest: &str) {
        let test_name = Path::new(test_dest).file_stem().unwrap()
            .to_string_lossy().to_string();
        let is_folder = test_name == test_dest;

        translate(&format!("test_data/{}", test_dest), false);

        let out_file_path = match is_folder {
            true => format!("test_data/{}/{}.asm", test_name, test_name),
            false => format!("test_data/{}.asm", test_name),
        };
        
        let out = load_text(&out_file_path);
        let solution = load_text(&format!("test_data/solution_{}.asm", test_name));

        assert_eq!(out, solution);
        fs::remove_file(out_file_path).unwrap();
        
    }
}
