use std::{env, fs};

fn count_bytes(contents: String) -> usize {
    contents.len()
}

fn count_lines(contents: String) -> usize {
    let lines: Vec<&str> = contents.split("\n").collect();
    lines.len()
}

fn count_words(contents: String) -> usize {
    let words: Vec<&str> = contents.split_ascii_whitespace().collect();
    words.len()
}

fn count_characters(contents: String) -> usize {
    contents.chars().count()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    let contents = fs::read_to_string(file_path)
        .expect("File not found");

    if query == "-c" {
        println!("{} {}", count_bytes(contents), file_path);
    } else if query == "-l" {
        println!("{} {}", count_lines(contents), file_path);
    } else if query == "-w" {
        println!("{} {}", count_words(contents), file_path);
    } else if query == "-m" {
        println!("{} {}", count_characters(contents), file_path);
    }
}
