use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let operation = &args[2];
    let contents = match fs::read_to_string(file_path) {
        Ok(file_value) => file_value,
        Err(error) => panic!("Problem when trying to open {}: {}", &file_path, &error),
    };
    handle_operation(operation, contents);
}

fn handle_operation(operation: &String, file_content: String) {
    match operation.as_str() {
        "-c" => {
            let word_count = word_counter(file_content);
            println!("This file is {} words long", &word_count)
        }
        _ => println!("Unknown operation"),
    }
}

fn word_counter(file_content: String) -> usize {
    let words: Vec<&str> = file_content.split_whitespace().collect();
    words.len()
}
