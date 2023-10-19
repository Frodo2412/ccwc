use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    let contents = fs::read_to_string(file_path)
        .expect("File not found");

    if query == "-c" {
        println!("{} {}", contents.len(), file_path);
    } else if query == "-l" {
        let lines: Vec<&str> = contents.split("\n").collect();
        println!("{} {}", lines.len(), file_path);
    } else if query == "-w" {
        let words: Vec<&str> = contents.split_ascii_whitespace().collect();
        println!("{} {}", words.len(), file_path);
    } else if query == "-m" {
        println!("{} {}", contents.chars().count(), file_path);
    }
}
