use std::{path::Path, env};

use clap::{command, Arg, ArgAction};
use code_writer::CodeWriter;
use parser::{Parser, CommandType};
use util::load_text;

mod parser;
mod code_writer;
mod util;

fn translate(input_path_str: &str) {
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
                code_writer.write_push_pop("push", parser.arg1(), parser.arg2())
            }
            CommandType::Pop => {
                code_writer.write_push_pop("pop", parser.arg1(), parser.arg2())
            }
            _ => {
                
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

    println!("Start translating for '{}", input_path);
    translate(&input_path);
    println!("Completed");
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::{translate, util::load_text};

    #[test]
    fn test_main() {
        translate("test_data/Add.vm");

        let out_file_path = "test_data/Add.asm";
        
        let out = load_text(out_file_path);
        let solution = load_text("test_data/solution_Add.asm");

        assert_eq!(out, solution);
        fs::remove_file(out_file_path).unwrap();
    }
}
