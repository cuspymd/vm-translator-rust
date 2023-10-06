use std::{path::{Path, PathBuf}, env, fs::{OpenOptions, self}, io::Write};

use clap::{command, Arg, ArgAction};
use code_writer::CodeWriter;
use glob::glob;
use parser::{Parser, CommandType};
use util::load_text;

mod parser;
mod code_writer;
mod util;

fn translate(input_path: &Path, need_bootstrap: bool) {
    if input_path.is_file() {
        translate_file(input_path);
    } else if input_path.is_dir() {
        translate_folder(input_path, need_bootstrap);
    }
}

fn translate_file(input_path: &Path) -> PathBuf {
    let folder_path = input_path.parent().unwrap();
    let file_base_name = input_path.file_stem().unwrap()
        .to_string_lossy().to_string();
    let output_path_str = folder_path.join(format!("{}.asm", &file_base_name))
        .to_string_lossy().to_string();
    let input_text = load_text(input_path);

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
    Path::new(&output_path_str).to_path_buf()
}

fn translate_folder(input_folder: &Path, need_bootstrap: bool) {
    let pattern = input_folder.join("*.vm").to_string_lossy().to_string();
    let vm_files = glob(&pattern).unwrap();
    let asm_files = vm_files
        .map(|vm_file| translate_file(vm_file.unwrap().as_path()))
        .collect::<Vec<PathBuf>>();

    let input_folder_name = input_folder.file_stem().unwrap().to_string_lossy().to_string();
    let out_file_path = input_folder.join(format!("{}.asm", input_folder_name));
    let out_file_path_str = out_file_path.to_string_lossy().to_string();

    {
        let mut code_writer = CodeWriter::new(&out_file_path_str);
        if need_bootstrap {
            code_writer.write_bootstrap();
        }
    }
    
    let mut out_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(out_file_path)
        .unwrap();

    for asm_file_path in &asm_files {
        let asm_file_name = asm_file_path.file_name().unwrap().to_string_lossy().to_string();
        let asm_text = load_text(asm_file_path);
        let final_text = format!("// > {}\n{}", asm_file_name, asm_text);

        out_file.write_all(final_text.as_bytes()).unwrap();
        fs::remove_file(asm_file_path).unwrap();
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

    let input_path_str = matches.get_one::<String>("input_path").unwrap();
    let need_bootstrap = !matches.get_flag("no_bootstrap");

    println!("Start translating for '{}", input_path_str);
    translate(Path::new(input_path_str), need_bootstrap);
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

    #[test]
    fn test_main_given_folder() {
        test_vm("TestFolder");
    }

    #[test]
    fn test_main_given_multi_comparison_commands() {
        test_vm("TestInternalSymbol");
    }

    fn test_vm(test_dest: &str) {
        let test_name = Path::new(test_dest).file_stem().unwrap()
            .to_string_lossy().to_string();
        let is_folder = test_name == test_dest;

        translate(Path::new(&format!("test_data/{}", test_dest)), false);

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
