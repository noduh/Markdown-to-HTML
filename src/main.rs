use std::fs;
use std::io;

fn main() {
    let mut file_path = String::new();

    println!("Please give the file path: ");
    io::stdin()
        .read_line(&mut file_path)
        .expect("Failed to read line");

    let file_path = file_path.trim();

    let file_contents = file_to_string(file_path);
    println!("The file reads:\n{file_contents}\nEOF");
}

fn file_to_string(file_path: &str) -> String {
    let file_contents = fs::read_to_string(file_path.trim()).expect("Failed to read the file");

    file_contents
}
