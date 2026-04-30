use std::fs;
use std::io;

fn main() {
    let mut file_path = String::new();

    println!("Please give the file path: ");
    io::stdin()
        .read_line(&mut file_path)
        .expect("Failed to read line");

    let file_contents = fs::read_to_string(file_path.trim()).expect("Failed to read the file");

    println!("The file reads:\n\n{file_contents}");
}
