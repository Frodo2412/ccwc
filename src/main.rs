use std::{env, fs};

fn count_bytes(contents: &String) -> usize {
    contents.len()
}

fn count_lines(contents: &String) -> usize {
    let lines: Vec<&str> = contents.split("\n").collect();
    lines.len()
}

fn count_words(contents: &String) -> usize {
    let words: Vec<&str> = contents.split_ascii_whitespace().collect();
    words.len()
}

fn count_characters(contents: &String) -> usize {
    contents.chars().count()
}

fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path)
        .expect("File not found")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 {
        let query = &args[1];
        let file_path = &args[2];
        let contents = read_file(file_path);


        if query == "-c" {
            println!("{} {}", count_bytes(&contents), file_path);
        } else if query == "-l" {
            println!("{} {}", count_lines(&contents), file_path);
        } else if query == "-w" {
            println!("{} {}", count_words(&contents), file_path);
        } else if query == "-m" {
            println!("{} {}", count_characters(&contents), file_path);
        }
    } else if args.len() == 2 {
        let file_path = &args[1];
        let contents = read_file(file_path);

        println!("{} {} {} {}", count_lines(&contents), count_words(&contents), count_characters(&contents), file_path);
    }
}
