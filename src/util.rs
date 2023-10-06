use std::{fs::File, io::Read, path::Path};

pub fn load_text<P: AsRef<Path>>(file_path: P) -> String {
    let mut file = File::open(file_path).unwrap();
    let mut text = String::new();

    file.read_to_string(&mut text).unwrap();
    text
}
