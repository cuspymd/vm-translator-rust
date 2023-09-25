use std::{fs::File, io::Read};

pub fn load_text(file_path: &str) -> String {
    let mut file = File::open(file_path).unwrap();
    let mut text = String::new();

    file.read_to_string(&mut text).unwrap();
    text
}
