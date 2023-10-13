use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    if query == "-c" {
        println!("Counting bytes in file: {}", file_path);
        let contents = fs::read_to_string(file_path)
            .expect("File not found");

        println!("{} {}", contents.len(), file_path);
    }
}
